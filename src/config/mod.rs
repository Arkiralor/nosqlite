use crate::models::config_models::ConfigJson;
use crate::utils::auth_utils::{hash_password, verify_password};
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

        let config: ConfigJson = ConfigJson::read_from_file(&config_file_path);

        let pid = config.pid;
        let data_dir = config.data_dir.clone();
        let master_user_name = config.master_user_name.clone();
        let master_user_password = hash_password(&config.master_user_password, &config.secret_key);
        let secret_key = config.secret_key.clone();
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

    pub fn verify_password(&self, password: &str) -> bool {
        verify_password(password, &self.master_user_password, &self.secret_key)
    }
}
