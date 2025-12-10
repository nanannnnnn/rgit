mod init;

use clap::{Parser, Subcommand};
use init::init;
use std::error::Error;
use std::fs::{self};

use sha2::{Digest, Sha256};
use std::io::Write;
use zstd::stream::Encoder;

pub fn write_blob(file_path: &str) -> Result<(), Box<dyn Error>> {
    // blobファイルを作成
    let content = fs::read(file_path)?;
    let header = format!("blob {}\0", content.len());

    let mut blob = Vec::new();
    blob.extend_from_slice(header.as_bytes());
    blob.extend_from_slice(&content);

    // blobからSHA-256を計算
    let mut hasher = Sha256::new();
    hasher.update(&blob);
    let hash = hasher.finalize();
    let hex = format!("{:x}", hash);

    // 保存先ディレクトリ
    // 先頭2桁をディレクトリにk、残りをファイル名に
    let dir = format!(".git/objects/{}/", &hex[..2]);
    let file = format!("{}.git/objects/{}/{}", "", &hex[..2], &hex[2..]);

    fs::create_dir_all(&dir)?;

    // zstd 圧縮して書き込む
    let f = fs::File::create(&file[1..])?;
    let mut encoder = Encoder::new(f, 0)?;
    encoder.write_all(&blob)?;
    encoder.finish()?;

    Ok(())
}
