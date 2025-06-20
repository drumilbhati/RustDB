use rustDB::{Document, RustDb, RustDbError};
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
                // Usage: insert <collection_name> <document_id> <json_document>
                // Example: insert users user1 '{"name": "Alice", "age": 30}'
                if parts.len() >= 4 {
                    let collection_name = parts[1].to_string();
                    let document_id = parts[2].to_string();

                    // Find the start of the JSON document
                    // This assumes the JSON document starts after the first 3 parts (command, collection, id)
                    let json_start_index = input.find(parts[3]).unwrap_or(0);
                    let json_str = &input[json_start_index..];

                    // IMPORTANT: If the user typed single quotes around the JSON,
                    // these quotes might be stripped by the shell before your
                    // program sees them. We need to handle cases where the JSON
                    // string itself might contain single quotes within it or
                    // where the user omitted them.

                    // A robust way to handle this is to trim surrounding quotes if present.
                    let clean_json_str = if json_str.starts_with('\'') && json_str.ends_with('\'') {
                        // If it's explicitly quoted with single quotes
                        &json_str[1..json_str.len() - 1]
                    } else if json_str.starts_with('"') && json_str.ends_with('"') {
                        // If it's explicitly quoted with double quotes
                        &json_str[1..json_str.len() - 1]
                    } else {
                        // Assume no outer quotes, or they were stripped by the shell
                        json_str
                    };

                    match serde_json::from_str::<Document>(clean_json_str) {
                        Ok(document) => {
                            match db.insert_document(
                                collection_name.clone(),
                                document_id.clone(),
                                document,
                            ) {
                                Ok(()) => println!(
                                    "Inserted document into collection '{}' with ID '{}'",
                                    collection_name, document_id
                                ),
                                Err(e) => eprintln!("Error inserting document: {:?}", e),
                            }
                        }
                        Err(e) => eprintln!("Error parsing JSON document: {:?}", e),
                    }
                } else {
                    println!("Usage: insert <collection_name> <document_id> <json_document>");
                    println!("Example: insert users user1 '{{\"name\": \"Alice\", \"age\": 30}}'");
                    println!("Or:      insert users user1 {{\"name\": \"Alice\", \"age\": 30}}");
                }
            }
            "get" => {
                if parts.len() == 3 {
                    let collection_name = parts[1];
                    let document_id = parts[2];
                    match db.get_document(collection_name, document_id) {
                        Some(document) => {
                            println!(
                                "Document in '{}' with ID '{}': {}",
                                collection_name,
                                document_id,
                                document.to_string()
                            );
                        }
                        None => println!(
                            "Document with ID '{}' not found in collection: '{}'",
                            document_id, collection_name
                        ),
                    }
                } else {
                    println!("Usage: get <collection_name> <document_id>");
                }
            }
            "delete" => {
                if parts.len() == 3 {
                    let collection_name = parts[1];
                    let document_id = parts[2];

                    match db.delete_document(collection_name, document_id) {
                        Ok(()) => println!(
                            "Deleted document with id: '{}' from collection: '{}'",
                            document_id, collection_name
                        ),
                        Err(RustDbError::KeyNotFound) => println!(
                            "Document with ID: '{}' not found in collection: '{}'",
                            document_id, collection_name
                        ),
                        Err(e) => eprintln!("Error deleting: {:?}", e),
                    }
                } else {
                    println!("Usage: delete <collection_name> <document_id>");
                }
            }
            "list" => {
                if parts.len() == 2 {
                    let collection_name = parts[1];
                    println!("Documents in collection: '{}'", collection_name);
                    let data = db.list_collection_documents(collection_name);
                    if data.is_empty() {
                        println!("No documents found in collection: '{}'", collection_name);
                    } else {
                        println!("{{");
                        for (id, doc) in data {
                            println!("  {}:\n      {}", id, doc.to_string());
                        }
                        println!("}}")
                    }
                } else {
                    println!("Usage: list <collection_name>");
                }
            }
            "clear" => {
                if parts.len() == 2 {
                    let collection_name = parts[1];
                    println!("Clearing collection: {}", collection_name);
                    match db.clear_collections(collection_name) {
                        Ok(()) => println!("All collections cleared."),
                        Err(_e) => {
                            eprintln!("Error clearing collection: {}.", collection_name)
                        }
                    }
                }
            }
            "help" => {
                println!("Commands:");
                println!("  insert <collection> <doc_id> <json_doc> - Inserts/updates a document.");
                println!("    Example: insert users user1 '{{\"name\":\"Alice\",\"age\":30}}'");
                println!("  get <collection> <doc_id>           - Retrieves a document.");
                println!("  delete <collection> <doc_id>        - Deletes a document.");
                println!(
                    "  list [collection]                   - Lists all collections or documents in a collection."
                );
                println!(
                    "  find <collection> <field> <json_val> - Finds documents by field value."
                );
                println!("    Example: find users age 30");
                println!(
                    "  clearall                            - Clears all collections and documents."
                );
                println!("  exit                                - Exits the application.");
                println!("  help                                - Displays this help message.");
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
