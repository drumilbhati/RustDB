# RustDB: A Beginner-Friendly Redis-like Engine

Welcome to the RustDB project! This project is designed as a learning journey into Rust through the lens of building a functional, high-performance in-memory database with disk persistence—inspired by Redis.

## Vision
To build a database that is:
- **Fast:** Primary data lives in memory for sub-millisecond access.
- **Durable:** Changes are persisted to disk so data survives restarts.
- **Educational:** Learn about Rust's memory safety, concurrency, and network programming.

---

## 🗺️ Project Roadmap

### Phase 1: The REPL & Core Commands
Goal: Create an interactive shell and basic String storage.
- [ ] Implement the REPL loop.
- [ ] Implement `SET <key> <value>` and `GET <key>`.
- [ ] **Micro-plan:**
    - Use `HashMap<String, String>` as the primary store.
    - Implement a basic parser for "SET/GET" commands.

### Phase 2: Persistence - Append Only File (AOF)
Goal: Ensure every write is logged to disk.
- [ ] Implement a logger that writes every `SET` command to a file.
- [ ] Implement a "Replay" mechanism on startup to reconstruct state from the AOF.
- [ ] **Micro-plan:**
    - Learn about `std::fs::OpenOptions` for appending to files.
    - Implement a simple line-based protocol for the log.

### Phase 3: Complex Data Types (Lists & Sets)
Goal: Expand beyond simple strings.
- [ ] Implement `LPUSH`, `RPUSH`, and `LPOP` for Lists.
- [ ] Implement `SADD` and `SMEMBERS` for Sets.
- [ ] **Micro-plan:**
    - Use `Enum` for `Value` (String, List, Set).
    - Use `VecDeque` for Lists and `HashSet` for Sets.

### Phase 4: Persistence - Snapshotting (RDB)
Goal: Save the entire dataset to disk periodically for faster loading.
- [ ] Implement a background "Save" command.
- [ ] Binary serialization of the entire memory state.
- [ ] **Micro-plan:**
    - Learn about `serde` and `bincode` for binary serialization.
    - Explore basic threading (background saving).

### Phase 5: Expiry & Eviction (TTL)
Goal: Automatically delete keys after a certain time.
- [ ] Implement `EXPIRE <key> <seconds>`.
- [ ] Implement a periodic cleanup task.
- [ ] **Micro-plan:**
    - Add a `timestamp` field to stored values.
    - Learn about `std::time::Instant`.

### Phase 6: Networking (The Server)
Goal: Turn the REPL into a real server.
- [ ] Implement a TCP server using `std::net::TcpListener`.
- [ ] Implement a simple protocol (RESP-lite).
- [ ] **Micro-plan:**
    - Learn about multi-threading or async (`tokio`) to handle multiple clients.

---

## 🛠️ Tech Stack
- **Language:** Rust (Stable)
- **Data Structures:** `HashMap`, `VecDeque`, `HashSet`
- **Persistence:** Custom AOF (Text), Bincode (Binary Snapshots)

## 📖 Learning Resources
- [Redis Documentation (Command Reference)](https://redis.io/commands)
- [The Rust Book: Smart Pointers & Concurrency](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
