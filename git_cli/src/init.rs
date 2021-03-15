use std::fs;
use std::io::Write;
use std::path::Path;

use super::error::RegitError;

pub fn init() -> Result<(), RegitError> {
    let dir = Path::new(".regit");
    fs::create_dir(dir)?;
    fs::create_dir(dir.join("objects"))?;
    fs::create_dir(dir.join("refs"))?;
    fs::create_dir(dir.join("refs").join("heads"))?;

    let mut head = fs::File::create(dir.join("HEAD"))?;
    head.write_all("refs: refs/heads/master".as_bytes())?;
    Ok(())
}