#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(warnings)]

mod config;
mod models;
mod schema;
mod utils;
use std;
use std::io::{self, BufRead};
use utils::database_commands::execute_command;

fn main() {
    let pid: u32 = std::process::id();
    let settings: config::Settings = config::Settings::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    if !parse_arguments(&settings) {
        eprintln!(
            "Exiting process with PID {} due to authentication failure.",
            pid
        );
        std::process::exit(1);
    }

    loop {
        let mut user_command = String::new();
        match handle.read_line(&mut user_command) {
            Ok(_) => {
                let trimmed = user_command.trim();
                if !trimmed.is_empty() {
                    // println!("PID {} received: {}", pid, trimmed);
                    execute_command(&user_command, &settings);
                } else {
                    println!("No input received for PID {}.", pid);
                }
            }
            Err(_e) => {}
        }
    }
}

fn parse_arguments(settings: &config::Settings) -> bool {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: <program> <username> <password>");
        return false;
    }
    let username = &args[1];
    let password = &args[2];

    if username == &settings.master_user_name && settings.verify_password(password) {
        println!("Authentication successful for user: {}", username);
        return true;
    } else {
        eprintln!("Authentication failed for user: {}", username);
        return false;
    }
}

mod tests;
