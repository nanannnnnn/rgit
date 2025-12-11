struct IndexEntry {
    ctime_sec: u32,  // 作成時刻（秒）
    ctime_nsec: u32, // 作成時刻（ナノ秒）
    mtime_sec: u32,  // 更新時刻（秒）
    mtime_nsec: u32, // 更新時刻（ナノ秒）
    dev: u32,        // デバイス番号
    ino: u32,        // inode番号
    mode: u32,       // ファイルモード (例: 0o100644)
    uid: u32,        // ユーザーID
    gid: u32,        // グループID
    file_size: u32,  // ファイルサイズ
    hash: [u8; 32],  // blobのSHA-256
    flags: u16,      // フラグ（ファイル名の長さ等）
    path: String,    // ファイルパス
}

struct Index {
    entries: Vec<IndexEntry>,
}

fn create_entry(path: &str, hash: [u8; 32]) -> IndexEntry {
    let metadata = fs::metadata(path).unwrap();

    IndexEntry {
        ctime_sec: metadata.ctime() as u32,
        ctime_nsec: metadata.ctime_nsec() as u32,
        mtime_sec: metadata.mtime() as u32,
        mtime_nsec: metadata.mtime_nsec() as u32,
        dev: metadata.dev() as u32,
        ino: metadata.ino() as u32,
        mode: 0o100644, // 通常ファイル
        uid: metadata.uid(),
        gid: metadata.gid(),
        file_size: metadata.len() as u32,
        hash,
        flags: path.len().min(0xFFF) as u16, // パス長（最大4095）
        path: path.to_string(),
    }
}

fn write_index(index: &Index) -> Result<(), Box<dyn Error>> {
    let mut data = Vec::new();

    // ===== ヘッダー (12バイト) =====
    data.extend_from_slice(b"DIRC"); // シグネチャ
    data.extend_from_slice(&2u32.to_be_bytes()); // バージョン2
    data.extend_from_slice(&(index.entries.len() as u32).to_be_bytes()); // エントリ数

    // ===== 各エントリ =====
    for entry in &index.entries {
        // メタデータ (40バイト)
        data.extend_from_slice(&entry.ctime_sec.to_be_bytes());
        data.extend_from_slice(&entry.ctime_nsec.to_be_bytes());
        data.extend_from_slice(&entry.mtime_sec.to_be_bytes());
        data.extend_from_slice(&entry.mtime_nsec.to_be_bytes());
        data.extend_from_slice(&entry.dev.to_be_bytes());
        data.extend_from_slice(&entry.ino.to_be_bytes());
        data.extend_from_slice(&entry.mode.to_be_bytes());
        data.extend_from_slice(&entry.uid.to_be_bytes());
        data.extend_from_slice(&entry.gid.to_be_bytes());
        data.extend_from_slice(&entry.file_size.to_be_bytes());

        // ハッシュ (32バイト)
        data.extend_from_slice(&entry.hash);

        // フラグ (2バイト)
        data.extend_from_slice(&entry.flags.to_be_bytes());

        // パス (可変長 + NUL終端)
        data.extend_from_slice(entry.path.as_bytes());
        data.push(0); // NUL終端

        // パディング: エントリ全体を8バイト境界に揃える
        // エントリの開始位置はヘッダー(12バイト)の後なので、
        // (62 + path.len() + 1 + padding) が 8 の倍数になるようにする
        let entry_len = 62 + entry.path.len() + 1; // 62 = 40 + 32 + 2 - 12(ヘッダー分は含まない計算方法もある)
        let padding_len = (8 - (entry_len % 8)) % 8;
        for _ in 0..padding_len {
            data.push(0);
        }
    }

    // ===== チェックサム (32バイト) =====
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let checksum = hasher.finalize();
    data.extend_from_slice(&checksum);

    // ===== ファイルに書き込み =====
    fs::write(".git/index", data)?;

    Ok(())
}

pub fn add_index(path: &str, hash: [u8; 32]) {
    // 既存の .git/indexを読み込む
    let mut index = read_index()?;

    // 新しいエントリを作成 ファイルのメタデータを取得（mite, sizetなど)
    let new_entry = create_entry(path, hash)?;

    match index
        .entries
        .iter_mut()
        .binary_search_by_key(&&new_entry.path, |e| &e.path)
    {
        Ok(i) => {
            index.entries[i] = new_entry;
        }
        Err(i) => {
            index.entries.insert(i, new_entry);
        }
    }
    write_index(&index)?;

    Ok(())
}
