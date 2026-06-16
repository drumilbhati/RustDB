# RustDB: A Beginner-Friendly Database Engine

Welcome to the RustDB project! This project is designed as a learning journey into Rust through the lens of building a functional database engine.

## Vision
To build a database that is:
- **Educational:** Well-documented code that teaches Rust concepts.
- **Simple:** Minimal dependencies, focusing on standard library features where possible.
- **Functional:** Capable of storing, retrieving, and querying data reliably.

---

## 🗺️ Project Roadmap

### Phase 1: The REPL (Read-Eval-Print Loop)
Goal: Create an interactive shell to talk to our database.
- [ ] Initialize the project structure.
- [ ] Implement a basic input loop.
- [ ] Add meta-commands (e.g., `.exit`, `.help`).
- [ ] **Micro-plan:**
    - Study `std::io::stdin`.
    - Use `match` for command routing.
    - Implement a "Prompt" that prints `db > `.

### Phase 2: In-Memory Key-Value Store
Goal: Implement core logic for storing data in memory.
- [ ] Define the storage backend (using `HashMap`).
- [ ] Implement `SET <key> <value>` command.
- [ ] Implement `GET <key>` command.
- [ ] **Micro-plan:**
    - Use `std::collections::HashMap`.
    - Handle ownership of strings between the parser and storage.
    - Basic error reporting for missing keys.

### Phase 3: Persistence (Disk Storage)
Goal: Make sure data survives a restart.
- [ ] Implement a "Flush" mechanism to save to a file.
- [ ] Load existing data from the file on startup.
- [ ] Use `Serde` for easy serialization to JSON (initially).
- [ ] **Micro-plan:**
    - Implement `save_to_disk()` and `load_from_disk()`.
    - Handle `std::io::Error` gracefully.
    - Explore `serde_json` for human-readable initial storage.

### Phase 4: Structured Data & Tables
Goal: Move from simple KV to structured "Rows".
- [ ] Define a `Row` struct with fixed fields (e.g., id, username, email).
- [ ] Create a `Table` abstraction.
- [ ] Implement `insert` and `select` for rows.
- [ ] **Micro-plan:**
    - Learn about Memory Layout (how rows are stored in memory).
    - Use `Vec<Row>` or a custom Buffer Manager.

### Phase 5: Simple SQL Parser
Goal: Parse basic SQL statements instead of raw commands.
- [ ] Implement a tokenizer.
- [ ] Parse `SELECT`, `INSERT`, `DELETE` statements.
- [ ] Link the parser to the table execution logic.
- [ ] **Micro-plan:**
    - Study basic string parsing patterns in Rust.
    - Implement an `Enum` to represent supported SQL Statements.

### Phase 6: Indexing (The B-Tree)
Goal: Speed up lookups using a B-Tree.
- [ ] Implement a basic B-Tree structure.
- [ ] Use the B-Tree to index the Primary Key (ID).
- [ ] Support range queries (e.g., `id > 10`).
- [ ] **Micro-plan:**
    - Learn about B-Tree node splitting and merging.
    - Implement disk-backed B-Tree nodes.

---

## 🛠️ Tech Stack
- **Language:** Rust (Stable)
- **Serialization:** `serde`, `serde_json`
- **Error Handling:** `thiserror` or `anyhow` (optional, for later)

## 📖 Learning Resources
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Database Internals (Book)](https://www.databass.dev/) - Reference for concepts.
