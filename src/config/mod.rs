use std::env;
use std::path::{Path, PathBuf};

pub struct Settings {
    pub base_dir: PathBuf,
    pub config_file_name: String,
    pub config_file_path: PathBuf,
    pub pid: u32,
    pub data_dir: String,
    pub master_user_name: String,
    pub master_user_password: String,
    pub secret_key: String,
}

impl Settings {
    pub fn new() -> Self {
        let base_dir = Self::find_base_dir();
        let config_file_name = "config.json".to_string();
        let config_file_path = base_dir.join(&config_file_name);

        let pid = u32::default();
        let data_dir = "data".to_string();
        let master_user_name = "admin".to_string();
        let master_user_password = "password".to_string();
        let secret_key = "my-s3cre4t-k3y".to_string();
        Settings {
            base_dir,
            config_file_name,
            config_file_path,
            pid,
            data_dir,
            master_user_name,
            master_user_password,
            secret_key,
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
