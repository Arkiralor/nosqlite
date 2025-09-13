use std::env;
use std::path::{Path, PathBuf};

pub struct Settings {
    pub base_dir: PathBuf,
    pub config_file_name: String,
    pub config_file_path: PathBuf,
}

impl Settings {
    pub fn new() -> Self {
        let base_dir = Self::find_base_dir();
        let config_file_name = "config.json".to_string();
        let config_file_path = base_dir.join(&config_file_name);
        Settings {
            base_dir,
            config_file_name,
            config_file_path,
        }
    }

    fn find_base_dir() -> PathBuf {
        let this_file: &Path = Path::new(file!());
        let base_dir: PathBuf = this_file
            .parent() // config
            .and_then(|p| p.parent()) // src
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf();
        let cwd = env::current_dir().unwrap();
        cwd.join(base_dir).to_path_buf()
    }
}
