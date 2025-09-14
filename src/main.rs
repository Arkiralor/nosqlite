mod config;
mod models;
mod schema;
mod utils;
use std;
use std::fs::File;
use std::io::{self, BufRead};
use utils::database_commands::execute_command;

fn main() {
    let pid: u32 = std::process::id();
    let mut iteration: u64 = 0;

    let settings: config::Settings = config::Settings::new();
    let config: models::config_models::ConfigJson =
        models::config_models::ConfigJson::read_from_file(&settings.config_file_path);

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        let mut user_command = String::new();
        match handle.read_line(&mut user_command) {
            Ok(_) => {
                let trimmed = user_command.trim();
                if !trimmed.is_empty() {
                    println!("PID {} received: {}", pid, trimmed);
                    // You can parse and handle the command here
                } else {
                    println!("No input received for PID {}.", pid);
                }
            }
            Err(e) => {}
        }
        execute_command(&user_command, &settings, &config);
    }
}
