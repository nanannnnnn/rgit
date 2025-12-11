mod blob;
use std::error::Error;

use blob::write_blob;

pub fn add(paths: Vec<String>) -> Result<(), Box<dyn Error>> {
    for path in paths {
        write_blob(&path);
    }

    Ok(())
}
