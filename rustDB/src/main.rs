use rustDB::{RustDb, RustDbError};
use std::io::{self, Write};

fn main() {
    let db_file = "my_db.json";

    let mut db = RustDb::new(db_file).expect("Failed to initialize database");

    println!("Welcome to the RustDB CLI");
    println!("Type 'help' for commands.");

    loop {
        print!("db> ");
        io::stdout().flush().expect("Could not flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "insert" => {
                if parts.len() == 3 {
                    let key = parts[1].to_string();
                    let value = parts[2].to_string();
                    match db.insert(key.clone(), value.clone()) {
                        Ok(()) => println!("Inserted: {} = {}", key, value),
                        Err(e) => eprintln!("Error inserting: {:?}", e),
                    }
                } else {
                    println!("Usage: insert <key> <value>");
                }
            }
            "get" => {
                if parts.len() == 2 {
                    let key = parts[1];
                    match db.get(key) {
                        Some(value) => println!("Value for {} {}", key, value),
                        None => println!("Key '{}' not found", key),
                    }
                } else {
                    println!("Usage: get <key>");
                }
            }
            "delete" => {
                if parts.len() == 2 {
                    let key = parts[1];
                    match db.delete(key) {
                        Ok(()) => println!("Deleted key: {}", key),
                        Err(RustDbError::KeyNotFound) => println!("Key '{}' not found", key),
                        Err(e) => eprintln!("Error deleting: {:?}", e),
                    }
                } else {
                    println!("Usage: delete <key>");
                }
            }
            "list" => {
                println!("Current database content:");
                let data = db.list_all();
                if data.is_empty() {
                    println!("No data found");
                } else {
                    println!("{{");
                    for (key, value) in data {
                        println!("  {}: {},", key, value);
                    }
                    println!("}}");
                }
            }
            "clear" => {
                println!("Deleting the database");
                db.clear();
            }
            "help" => {
                println!("Commands:");
                println!("  insert <key> <value> - Inserts or updates a key-value pair.");
                println!("  get <key>            - Retrieves the value for a given key.");
                println!("  delete <key>         - Deletes a key-value pair.");
                println!("  list                 - (Not yet fully implemented)");
                println!("  clear                - (Not yet fully implemented)");
                println!("  exit                 - Exits the application.");
                println!("  help                 - Displays this help message.");
            }
            "exit" => {
                println!("Exiting database CLI");
                break;
            }
            _ => {
                println!("Unknown command: '{}'. Type 'help' for commands.", parts[0]);
            }
        }
    }
}
