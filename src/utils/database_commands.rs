use crate::config::Settings;
use crate::models::query::MongoQuery;
use crate::utils::file_utils::create_file;
use std::fmt::format;
use std::path::Path;

pub fn execute_command(command: &str, settings: &Settings) {
    println!("Executing command: {}", command);
    match MongoQuery::parse(command) {
        Ok(parsed_command) => {
            println!("Parsed command: {:?}", parsed_command);
            execute_parsed_query(&parsed_command, settings);
        }
        Err(e) => {
            eprintln!("Failed to parse command: {}", e);
        }
    }
}

fn execute_parsed_query(query: &MongoQuery, settings: &Settings) {
    match query {
        MongoQuery::Find {
            find,
            filter,
            sort,
            limit,
        } => {
            println!("Find in collection: {}", find);
            println!("Filter: {:?}", filter);
            println!("Sort: {:?}", sort);
            println!("Limit: {:?}", limit);
            // Add find logic here
        }
        MongoQuery::CreateCollection { name, options } => {
            println!("Create collection: {}", name);
            println!("Options: {:?}", options);
            let collection_path = format!(
                "{}/{}/{}",
                &settings.base_dir.display(),
                settings.data_dir,
                name
            );
            let collection_dir = Path::new(&collection_path);
            if !collection_dir.exists() {
                match std::fs::create_dir_all(collection_dir) {
                    Ok(_) => {
                        println!(
                            "Collection directory '{}' created at '{}'.",
                            name,
                            collection_dir.display()
                        );
                        let options_str = match options {
                            Some(opts) => format!("{}", opts),
                            None => "{}".to_string(),
                        };
                        let init_path = collection_dir.join("__init__.json");
                        if let Err(e) = create_file(&init_path, options_str.as_bytes()) {
                            eprintln!("Failed to create __init__.json: {}", e);
                        }
                    }
                    Err(e) => eprintln!("Failed to create collection directory '{}': {}", name, e),
                }
            } else {
                println!(
                    "Collection directory '{}' already exists at '{}'.",
                    name,
                    collection_dir.display()
                );
            }
        }
        MongoQuery::DropCollection { name } => {
            println!("Drop collection: {}", name);
            let collection_path = format!(
                "{}/{}/{}",
                &settings.base_dir.display(),
                settings.data_dir,
                name
            );
            let collection_dir = Path::new(&collection_path);
            if collection_dir.exists() {
                match std::fs::remove_dir_all(collection_dir) {
                    Ok(_) => println!(
                        "Collection directory '{}' removed from '{}'.",
                        name,
                        collection_dir.display()
                    ),
                    Err(e) => eprintln!("Failed to remove collection directory '{}': {}", name, e),
                }
            } else {
                println!(
                    "Collection directory '{}' does not exist at '{}'.",
                    name,
                    collection_dir.display()
                );
            }
        }
        MongoQuery::CreateUser {
            username,
            password,
            roles,
        } => {
            println!("Create user: {}", username);
            println!("Password: {}", password);
            println!("Roles: {:?}", roles);
            // Add create user logic here
        }
        MongoQuery::DeleteUser { username } => {
            println!("Delete user: {}", username);
            // Add delete user logic here
        }
    }
}
