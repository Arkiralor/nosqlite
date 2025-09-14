#[cfg(test)]
mod tests {
    use crate::config::Settings;
    use crate::models::config_models::ConfigJson;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    // Helper to create a temporary config file
    fn create_temp_config(path: &PathBuf, password: &str, secret: &str) {
        let config = format!(
            r#"{{
			"pid": 12345,
			"data_dir": "test_data",
			"master_user_name": "admin",
			"master_user_password": "{}",
			"secret_key": "{}"
		}}"#,
            password, secret
        );
        let mut file = File::create(path).expect("Failed to create temp config file");
        file.write_all(config.as_bytes())
            .expect("Failed to write config");
    }

    #[test]
    fn test_settings_new_and_fields() {
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("virtual_config.json");
        let password = "testpass";
        let secret = "testsecret";
        create_temp_config(&config_path, password, secret);

        // Patch Settings to use the virtual config file for this test
        // This requires constructing Settings manually for the test
        let config = ConfigJson::read_from_file(&config_path);
        let settings = Settings {
            base_dir: temp_dir.clone(),
            config_file_name: "virtual_config.json".to_string(),
            config_file_path: config_path.clone(),
            pid: config.pid,
            data_dir: config.data_dir.clone(),
            master_user_name: config.master_user_name.clone(),
            master_user_password: crate::utils::auth_utils::hash_password(
                &config.master_user_password,
                &config.secret_key,
            ),
            secret_key: config.secret_key.clone(),
        };

        assert_eq!(settings.master_user_name, "admin");
        assert_eq!(settings.data_dir, "test_data");
        assert_eq!(settings.pid, 12345);
        assert_eq!(settings.secret_key, secret);
        // Password should be hashed, not equal to plain
        assert_ne!(settings.master_user_password, password);
    }

    #[test]
    fn test_verify_password() {
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("virtual_config2.json");
        let password = "mypassword";
        let secret = "mysecret";
        create_temp_config(&config_path, password, secret);

        let config = ConfigJson::read_from_file(&config_path);
        let settings = Settings {
            base_dir: temp_dir.clone(),
            config_file_name: "virtual_config2.json".to_string(),
            config_file_path: config_path.clone(),
            pid: config.pid,
            data_dir: config.data_dir.clone(),
            master_user_name: config.master_user_name.clone(),
            master_user_password: crate::utils::auth_utils::hash_password(
                &config.master_user_password,
                &config.secret_key,
            ),
            secret_key: config.secret_key.clone(),
        };

        // Should verify correct password
        assert!(settings.verify_password(password));
        // Should fail for wrong password
        assert!(!settings.verify_password("wrongpassword"));
    }
}
