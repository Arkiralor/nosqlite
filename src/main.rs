mod config;
mod models;
mod schema;
mod utils;
use std;
use std::fs::File;
use std::io::{self, BufRead};
use utils::auth_utils::{hash_password, verify_password};
use utils::database_commands::execute_command;

fn main() {
    let pid: u32 = std::process::id();
    let mut iteration: u64 = 0;

    let settings: config::Settings = config::Settings::new();
    let config: models::config_models::ConfigJson =
        models::config_models::ConfigJson::read_from_file(&settings.config_file_path);

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    if !parse_arguments(&config) {
        eprintln!(
            "Exiting process with PID {} due to authentication failure.",
            pid
        );
        std::process::exit(1);
    }
    // let mut password: &str = "admin_password";
    // println!(
    //     "Hashed master password: {}",
    //     hash_password(&password, &config.secret_key)
    // );

    loop {
        let mut user_command = String::new();
        match handle.read_line(&mut user_command) {
            Ok(_) => {
                let trimmed = user_command.trim();
                if !trimmed.is_empty() {
                    // println!("PID {} received: {}", pid, trimmed);
                    execute_command(&user_command, &settings, &config);
                } else {
                    println!("No input received for PID {}.", pid);
                }
            }
            Err(e) => {}
        }
    }
}

fn parse_arguments(config: &models::config_models::ConfigJson) -> bool {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: <program> <username> <password>");
        return false;
    }
    let username = &args[1];
    let password = &args[2];

    if username == &config.master_user_name
        && verify_password(password, &config.master_user_password, &config.secret_key)
    {
        println!("Authentication successful for user: {}", username);
        return true;
    } else {
        eprintln!("Authentication failed for user: {}", username);
        return false;
    }
}
