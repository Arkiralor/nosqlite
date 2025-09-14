use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::fs::File;
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
///Struct to represent the configuration settings loaded from a JSON file.
pub struct ConfigJson {
    pub pid: u32,
    pub data_dir: String,
    pub master_user_name: String,
    pub master_user_password: String,
    pub secret_key: String,
}

impl ConfigJson {
    pub fn new(
        pid: u32,
        data_dir: String,
        master_user_name: String,
        master_user_password: String,
        secret_key: String,
    ) -> Self {
        ConfigJson {
            pid,
            data_dir,
            master_user_name,
            master_user_password,
            secret_key,
        }
    }

    pub fn create(
        pid: Option<u32>,
        data_dir: Option<String>,
        master_user_name: Option<String>,
        master_user_password: Option<String>,
        secret_key: Option<String>,
    ) -> Self {
        //! Create a new config object from the given data.
        let pid: u32 = match pid {
            Some(val) => val,
            None => u32::default(),
        };

        let data_dir: String = match data_dir {
            Some(val) => val,
            None => String::from("data"),
        };

        let master_user_name: String = match master_user_name {
            Some(val) => val,
            None => String::from("admin"),
        };

        let master_user_password: String = match master_user_password {
            Some(val) => val,
            None => String::from("password"),
        };

        let secret_key: String = match secret_key {
            Some(val) => val,
            None => String::from("my-s3cre4t-k3y"),
        };

        ConfigJson {
            pid,
            data_dir,
            master_user_name,
            master_user_password,
            secret_key,
        }
    }

    pub fn read_from_file(file_path: &PathBuf) -> Self {
        let panic_msg: String = format!("File '{}' not found.", file_path.display());
        let config_json_file_obj: File = File::open(file_path).expect(&panic_msg);
        let imported_config: ConfigJson =
            serde_json::from_reader(config_json_file_obj).expect("Error while reading file");
        imported_config
    }
}
impl Display for ConfigJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "pid: {}, data_dir: \"{}\", master_user_name: \"{}\", master_user_password: \"{}\", secret_key: \"{}\"",
            self.pid, self.data_dir, self.master_user_name, self.master_user_password, self.secret_key
        )
    }
}

impl Debug for ConfigJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "pid: {}, data_dir: \"{}\", master_user_name: \"{}\", master_user_password: \"{}\", secret_key: \"{}\"",
            self.pid, self.data_dir, self.master_user_name, self.master_user_password, self.secret_key
        )
    }
}
