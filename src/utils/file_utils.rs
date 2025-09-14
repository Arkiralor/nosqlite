use std::io::Write;
use std::path::{Path, PathBuf};

pub fn create_file(file_path: &Path, contents: &[u8]) -> std::io::Result<()> {
    if !file_path.exists() {
        let mut file = std::fs::File::create(file_path)?;
        file.write_all(contents)?;
    }
    Ok(())
}
