# NoSQLite

A simple NoSQL database implemented in Rust, designed for ease of use and quick setup and designed to be used as a local database for small to medium-sized applications.

## Documentation

- Create a Collection

    This command creates a new collection named "my_collection" that is not capped and has a maximum size of 1MB in the `data_dir` directory as specified in the configuration file.
    
    To create a new collection in the database, send a JSON command with the following structure:

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


## Authors
- Arkiralor
    - [eMail _(Personal)_](mailto:prithoo11335@gmail.com)
    - [Github _(Personal)_](https://github.com/Arkiralor)
    - [Github _(Official)_](https://github.com/prithoomedhi)
    - [LinkedIn](https://www.linkedin.com/in/prithoo11335/)