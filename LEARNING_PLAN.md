# Rust Learning Plan: Building RustDB

This document outlines a phase-by-phase learning plan to master Rust while building a Redis-like database engine (RustDB). It pairs core Rust concepts directly with practical implementation steps.

---

## Step 1: Rust Basics & the REPL
**Goal:** Understand project structure, variables, and basic Control Flow while building the interactive shell for RustDB.

### Topics to Learn
- `cargo` (Rust's package manager).
- Variables, Mutability (`mut`), and Data Types.
- Functions and Control Flow (`loop`, `match`, `if/else`).
- Standard Input/Output (`std::io`).

### Project Implementation (RustDB Phase 1)
Write a `loop` that reads a string from the user (`stdin`), uses `match` to check if the user typed `.exit`, and otherwise echoes the command back.

### Resources
- [The Rust Book: Ch 1 - Getting Started](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
- [The Rust Book: Ch 2 - Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) *(Highly recommended: This perfectly mirrors building a REPL)*
- [The Rust Book: Ch 3 - Common Programming Concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)

---

## Step 2: Ownership, Borrowing, & Data Structures
**Goal:** Conquer Rust's most famous feature (Ownership) and use standard library collections to store data in memory.

### Topics to Learn
- Ownership, References (`&`), and Borrowing.
- The difference between `String` (owned) and `&str` (borrowed string slice).
- Enums (creating a `Command` enum to represent `SET` and `GET`).
- Collections, specifically `HashMap`.

### Project Implementation (RustDB Phase 1 cont.)
Create a `HashMap<String, String>`. Parse the user's input into `SET key value` or `GET key`, and modify/read from the `HashMap`.

### Resources
- [The Rust Book: Ch 4 - Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [The Rust Book: Ch 6 - Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [The Rust Book: Ch 8.3 - Storing Keys with Associated Values in Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)

---

## Step 3: Error Handling & File I/O
**Goal:** Make the database robust and implement our Append-Only File (AOF) persistence.

### Topics to Learn
- The `Result<T, E>` and `Option<T>` enums.
- The `?` operator for propagating errors concisely.
- Reading and writing files (`std::fs::File`, `OpenOptions`).

### Project Implementation (RustDB Phase 2)
Open a file named `appendonly.aof`. Every time a `SET` command succeeds, write the raw command string to this file. On startup, read this file line-by-line and execute the commands to restore the `HashMap`.

### Resources
- [The Rust Book: Ch 9 - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Rust by Example: File I/O](https://doc.rust-lang.org/rust-by-example/std_misc/file.html)

---

## Step 4: Structs, Traits, and Serialization
**Goal:** Organize your code into clean modules and implement RDB Snapshotting.

### Topics to Learn
- Structs and Methods (`impl` blocks).
- Traits (Rust's version of interfaces).
- Modules (`mod`, `use`, `pub`) for splitting code into multiple files.
- Using third-party crates (`serde` and `bincode`).

### Project Implementation (RustDB Phase 3 & 4)
Refactor the code to move the Parser, VM, and Storage Engine into separate files. Add `serde` to serialize the entire `HashMap` into a binary `.rdb` file.

### Resources
- [The Rust Book: Ch 5 - Using Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [The Rust Book: Ch 7 - Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Serde Framework Documentation](https://serde.rs/)

---

## Step 5: Advanced Data Types & Lifetimes
**Goal:** Add Lists (`LPUSH`, `LPOP`) and Sets (`SADD`) to the database.

### Topics to Learn
- Advanced Enums containing data (e.g., `enum Value { Str(String), List(VecDeque<String>) }`).
- Lifetimes (if storing references inside your parser/structs).

### Project Implementation (RustDB Phase 3)
Change your HashMap to `HashMap<String, Value>`. Use `std::collections::VecDeque` for fast list operations.

### Resources
- [The Rust Book: Ch 10 - Generic Types, Traits, and Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html)
- [Rust Standard Library Docs: VecDeque](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)

---

## Step 6: Concurrency & Networking
**Goal:** Turn your local REPL into a real server that accepts multiple clients over TCP.

### Topics to Learn
- TCP Sockets (`std::net::TcpListener`, `TcpStream`).
- Threads (`std::thread::spawn`).
- Shared State Concurrency (`Arc` - Atomic Reference Counted, `Mutex` or `RwLock`).

### Project Implementation (RustDB Phase 6)
Wrap your `HashMap` in an `Arc<Mutex<HashMap>>`. Start a TCP server that listens for connections, spawns a new thread for each client, and allows them to safely modify the database simultaneously.

### Resources
- [The Rust Book: Ch 16 - Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [The Rust Book: Ch 20 - Final Project: Building a Multithreaded Web Server](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)

---

## 🛠️ Recommended Tools
- **Rustlings:** Small exercises to get you used to reading and writing Rust code. Run `cargo install rustlings` in your terminal.
- **Rust Analyzer:** The official extension for VSCode/Zed. Provides real-time error checking, type hints, and code completions. Required for a good Rust experience.
