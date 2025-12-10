use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;

pub fn init() -> Result<(), Box<dyn Error>> {
    let git_dir = Path::new(".git");
    if git_dir.exists() {
        if git_dir.is_dir() {
            println!("already .git repository initialized");
        } else {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                ".git exists but is not a directory",
            )
            .into());
        }
    } else {
        fs::create_dir(".git")?;

        // HEAD
        fs::write(git_dir.join("HEAD"), b"ref: refs/heads/main\n")?;

        // objects
        let objects_dir = git_dir.join("objects");
        fs::create_dir_all(objects_dir.join("info"))?;
        fs::create_dir_all(objects_dir.join("pack"))?;

        // refs
        let refs_dir = git_dir.join("refs");
        fs::create_dir(refs_dir.join("heads"))?;
        fs::create_dir(refs_dir.join("tags"))?;
    }
    Ok(())
}
