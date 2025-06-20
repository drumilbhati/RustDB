use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

// Custom error type
#[derive(Debug)]
pub enum RustDbError {
    KeyNotFound,
    IoError(io::Error),
    SerializationError(serde_json::Error),
    DeserializationError(serde_json::Error),
}

impl From<io::Error> for RustDbError {
    fn from(err: io::Error) -> Self {
        RustDbError::IoError(err)
    }
}

impl From<serde_json::Error> for RustDbError {
    fn from(err: serde_json::Error) -> Self {
        if err.is_eof() {
            RustDbError::DeserializationError(err)
        } else {
            RustDbError::SerializationError(err)
        }
    }
}

// Document custom type
pub type Document = serde_json::Value;

// Database Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct RustDb {
    collections: HashMap<String, HashMap<String, Document>>,
    file_path: PathBuf,
}

// Implement methods
impl RustDb {
    // Associated function to create a new RustDb instance.
    pub fn new(file_path: &str) -> Result<Self, RustDbError> {
        let path = PathBuf::from(file_path);
        let mut db = RustDb {
            collections: HashMap::new(),
            file_path: path,
        };

        // try to load the database initially into the hashmap
        match db.load() {
            Ok(_) => Ok(db),
            Err(RustDbError::IoError(ref err)) if err.kind() == io::ErrorKind::NotFound => {
                println!(
                    "Database file not found. Creating a new database at {}.",
                    file_path
                );
                Ok(db)
            }
            Err(e) => Err(e),
        }
    }

    // load the data from file_path to hashmap
    fn load(&mut self) -> Result<(), RustDbError> {
        let content = fs::read_to_string(&self.file_path)?;
        if content.trim().is_empty() {
            self.collections = HashMap::new();
        } else {
            self.collections = serde_json::from_str(&content)?;
        }
        Ok(())
    }

    // insert a new document to a document
    pub fn insert_document(
        &mut self,
        collection_name: String,
        document_id: String,
        document: Document,
    ) -> Result<(), RustDbError> {
        self.collections
            .entry(collection_name)
            .or_default()
            .insert(document_id, document);
        self.save()?;
        Ok(())
    }

    // save the hashmap to a json file
    pub fn save(&self) -> Result<(), RustDbError> {
        let serialized = serde_json::to_string_pretty(&self.collections)?;
        fs::write(&self.file_path, serialized.as_bytes())?;
        Ok(())
    }

    // Get a document from a collection
    pub fn get_document(&self, collection_name: &str, document_id: &str) -> Option<&Document> {
        self.collections
            .get(collection_name)
            .and_then(|collection| collection.get(document_id))
    }

    // Delete a document from a collection
    pub fn delete_document(
        &mut self,
        collection_name: &str,
        document_id: &str,
    ) -> Result<(), RustDbError> {
        if let Some(collection) = self.collections.get_mut(collection_name) {
            if collection.remove(document_id).is_some() {
                self.save()?;
                Ok(())
            } else {
                // Document not found
                Err(RustDbError::KeyNotFound)
            }
        } else {
            // collection not found
            Err(RustDbError::KeyNotFound)
        }
    }

    // List all documents in a collection
    pub fn list_collection_documents(&self, collection_name: &str) -> Vec<(String, Document)> {
        self.collections
            .get(collection_name)
            .map(|collection| {
                collection
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect()
            })
            .unwrap_or_default()
    }

    // Clear a specific collection
    pub fn clear_collections(&mut self, collection_name: &str) -> Result<(), RustDbError> {
        if let Some(collection) = self.collections.get_mut(collection_name) {
            collection.clear()
        }
        self.save()?;
        Ok(())
    }
}
