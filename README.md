#Command

0. create a new project
    . cargo new hello_word

1. compile and run 
    . rustc main.rs
    . ./main
2. build and run
    . cargo build
    . cargo run
3. build and test
    . cargo build
    . cargo test

4.Cargo check
    . cargo check
5. Release mode
    . cargo build --release


### plan
🦀 Rust Learning Roadmap (8 Weeks)
Week 1 – Setup & Syntax Warmup

📖 Learn:

Install Rust (rustup), Cargo basics.

Variables, types, functions, control flow.

Differences from Java (immutability by default, no classes, only structs/enums).

Ownership & Borrowing (first exposure).

💻 Project:

CLI Calculator

Take two numbers & an operator from CLI, print result.

Week 2 – Ownership & Enums

📖 Learn:

Deep dive: Ownership, Borrowing, Lifetimes.

Option<T>, Result<T,E>, error handling with ?.

Enums & Pattern Matching (very different from Java’s enums).

💻 Project:

To-Do List CLI

Store tasks in a Vec<String>.

Add, remove, mark complete.

Save/load from JSON (serde).

Week 3 – Collections & Traits

📖 Learn:

Vectors, HashMaps.

Iterators & closures.

Traits (vs Java interfaces).

💻 Project:

Word Counter

Read a file.

Count frequency of words.

Print top 10 words.

Week 4 – Modules & Testing

📖 Learn:

Organizing code with modules (mod, pub, crate).

Writing & running tests (cargo test).

Using crates from crates.io.

💻 Project:

Configurable File Search (Mini-grep)

Search text in files.

Add CLI options (clap crate).

Week 5 – Concurrency & Async

📖 Learn:

Threads, channels.

Arc, Mutex.

Async programming (tokio, async/await).

💻 Project:

Concurrent Downloader

Download multiple URLs concurrently.

Save to files.

Week 6 – Web Development

📖 Learn:

HTTP server with axum or actix-web.

JSON serialization (serde).

Compare with Spring Boot controllers.

💻 Project:

Simple REST API

CRUD for “Books” with in-memory storage.

Endpoints: GET, POST, DELETE.

Week 7 – Databases & Error Handling

📖 Learn:

SQLx or Diesel for Postgres.

Strongly typed queries.

Better error handling with thiserror or anyhow.

💻 Project:

Bookstore API with DB

Extend previous REST API.

Store books in Postgres.

Add proper error handling.

Week 8 – Advanced Topics & Bigger Project

📖 Learn:

Traits + Generics (deeper dive).

Macros (derive, declarative).

Unsafe basics (just conceptually).

Build, lint, docs (cargo fmt, clippy, doc).

💻 Final Project (choose one):

Chat App (WebSocket with tokio).

Mini Compiler (parse a tiny language).

Portfolio API (full CRUD with Postgres + Axum).

🚀 How to Study

Daily: 1–2 hours coding + Rust Book chapter matching the week.

Weekend: Build the mini-project.

Repeat: Review errors, re-read ownership concepts until it “clicks”.