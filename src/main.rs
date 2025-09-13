mod config;
mod models;
mod schema;
mod utils;
use std;
use std::fs::File;

fn main() {
    let pid: u32 = std::process::id();
    let mut iteration: u64 = 0;

    let settings = config::Settings::new();

    let panic_msg: String = format!("File '{}' not found.", settings.config_file_path.display());
    let config_json_file_obj: File = File::open(settings.config_file_path).expect(&panic_msg);
    let imported_config: models::config_models::ConfigJson =
        serde_json::from_reader(config_json_file_obj).expect("Error while reading file");

    loop {
        iteration = iteration + 1;
        // print!("{}. Process running on Port: {}.\n", iteration, pid);
        // print!("Config Json File: {:?}\n", settings.config_file_path);
        print!("Config Data: {:?}\n", imported_config);
    }
}
