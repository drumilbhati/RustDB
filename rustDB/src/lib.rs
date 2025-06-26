use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
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

// Database Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct RustDb {
    data: HashMap<String, String>,
    file_path: PathBuf,
    log_path: PathBuf,
}

// Implement methods
impl RustDb {
    // Associated function to create a new RustDb instance.
    pub fn new(file_path: &str) -> Result<Self, RustDbError> {
        let path = PathBuf::from(file_path);
        let mut db = RustDb {
            data: HashMap::new(),
            file_path: path.clone(),
            log_path: path.with_extension("wal"),
        };
        match db.load() {
            Ok(_) => {
                // replay any remaining logs
                db.replay_log()?;
                db.clear_log()?;
                Ok(db)
            }
            Err(RustDbError::IoError(ref err)) if err.kind() == io::ErrorKind::NotFound => {
                println!(
                    "Database file not found. Creating a new database at {}",
                    file_path
                );
                Ok(db)
            }
            Err(e) => Err(e),
        }
    }

    fn load(&mut self) -> Result<(), RustDbError> {
        let content = fs::read_to_string(&self.file_path)?;
        if content.trim().is_empty() {
            self.data = HashMap::new();
        } else {
            self.data = serde_json::from_str(&content)?;
        }
        Ok(())
    }

    fn write_log(&self, entry: &str) -> Result<(), RustDbError> {
        // create a file "object"
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)?;

        // append to the file the log_entry
        writeln!(file, "{}", entry)?;
        Ok(())
    }

    pub fn insert(&mut self, key: String, value: String) -> Result<(), RustDbError> {
        self.write_log(&format!("insert {} {}", key, value))?;
        self.data.insert(key, value);
        self.save()?;
        Ok(())
    }

    pub fn save(&self) -> Result<(), RustDbError> {
        let serialized = serde_json::to_string_pretty(&self.data)?;
        fs::write(&self.file_path, serialized.as_bytes())?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn delete(&mut self, key: &str) -> Result<(), RustDbError> {
        self.write_log(&format!("delete {}", key))?;
        if self.data.remove(key).is_some() {
            self.save()?;
            Ok(())
        } else {
            Err(RustDbError::KeyNotFound)
        }
    }

    fn replay_log(&mut self) -> Result<(), RustDbError> {
        if let Ok(content) = fs::read_to_string(&self.log_path) {
            for line in content.lines() {
                let parts: Vec<&str> = line.splitn(3, ' ').collect();

                match parts.as_slice() {
                    ["insert", key, value] => {
                        self.data.insert(key.to_string(), value.to_string());
                    }
                    ["delete", key] => {
                        self.data.remove(*key);
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn clear_log(&mut self) -> Result<(), RustDbError> {
        fs::write(&self.log_path, "")?;
        Ok(())
    }

    pub fn list_all(&self) -> Vec<(String, String)> {
        self.data
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}
