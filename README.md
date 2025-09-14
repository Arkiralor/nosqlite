# nosqlite

## Overview

nosqlite is a lightweight, file-based NoSQL database written in Rust. It supports MongoDB-style queries and basic database operations such as creating collections, dropping collections, and managing users. The project is designed for simplicity, extensibility, and easy integration into Rust applications.

## Features

- MongoDB-inspired query language (JSON-based)
- Create, drop, and query collections
- Create and delete users
- Configurable settings via a singleton pattern
- Interactive command-line interface

## Getting Started

### Build and Run

Clone the repository and build with Cargo:

```sh
git clone <repo-url>
cd nosqlite
cargo build --release
```

Run the application:

```sh
cargo run <username> <password>
```

### Configuration

Configuration is loaded from `src/config.json`. Example:

```json
{
    "pid": 12345,
    "data_dir": "repository",
    "master_user_name": "admin",
    "master_user_password": "admin_password",
    "secret_key": "your_secret_key"
}
```

## Query Language

Queries are provided as JSON strings. Supported commands:

### Create Collection
```json
{
    "command": "createcollection",
    "name": "my_collection",
    "options": {
        "capped": false,
        "size": 1048576
    }
}
```

### Drop Collection
```json
{
    "command": "dropcollection",
    "name": "my_collection"
}
```

### Find Documents
```json
{
    "command": "find",
    "find": "my_collection",
    "filter": { "age": { "$gt": 18 } },
    "sort": { "age": -1 },
    "limit": 10
}
```

### Create User
```json
{
    "command": "createuser",
    "username": "new_user",
    "password": "new_password",
    "roles": ["readWrite", "dbAdmin"]
}
```

### Delete User
```json
{
    "command": "deleteuser",
    "username": "old_user"
}
```

## Usage Example

Start the application and enter queries interactively:

```
$ cargo run admin admin_password
Iteration 1. PID 12345. Type a command and press Enter:
{ "command": "createcollection", "name": "users" }
Collection directory 'users' created at '/path/to/repository/users'.
```

## Project Structure

- `src/main.rs` — Entry point, argument parsing, interactive loop
- `src/config/` — Settings and configuration loader
- `src/models/` — Data models, including query parser
- `src/utils/database_commands.rs` — Command execution logic
- `src/config.json` — Configuration file

### Config.JSON Structure

Please ensure you create a `config.json` file in the `src` directory with the following structure:

```json
{
    "pid": <the process ID of the running application>,
    "data_dir": "<the directory where database files are stored>",
    "master_user_name": "<the admin username>",
    "master_user_password": "<the admin password>",
    "secret_key": "<the secret key for encryption or hashing>"
}
```

Please use the following Python code to generate a SECRET_KEY.

```py
from secrets import token_hex
print(token_hex(128))
```

## Contributing

Pull requests and issues are welcome!

## License

[MIT License](LICENSE.md) under the principles of free & open source software and copyleft. 


## Authors
- Arkiralor
    - [eMail _(Personal)_](mailto:prithoo11335@gmail.com)
    - [Github _(Personal)_](https://github.com/Arkiralor)
    - [Github _(Official)_](https://github.com/prithoomedhi)
    - [LinkedIn](https://www.linkedin.com/in/prithoo11335/)