# Rust Console Projects

A collection of Rust projects covering CLI tools, REST APIs, database integration, and event-driven applications.

## Projects

### CLI Utilities & Unix Commands

* **`cat`** – Basic implementation of the classic Unix `cat` command.
* **`wc`** – Basic implementation of the Unix `wc` (word count) command.
* **`tail`** – Basic implementation of the Unix `tail` command.

### Echo Implementations

* **`echo`** – Standard CLI implementation of the `echo` command.
* **`echo-clap`** – Advanced `echo` implementation leveraging the `clap` derive API for argument parsing.
* **`echo-std`** – Minimal `echo` implementation built using only the Rust standard library.

### CLI Development & Regex

* **`regex-once`** – Practical example demonstrating how to efficiently reuse a compiled regular expression.
* **`clap-examples`** – Comprehensive examples of building robust command-line applications using `clap`.

### Web & Database Applications

* **`rest-backed-by-postgres`** – A fully functional REST API backed by a PostgreSQL database.
* **`rest-db-factory`** – A REST API featuring an abstract database access layer for flexible data management.

### Event-Driven & Streaming

* **`cdc-mongodb`** – A CDC pipeline streaming data changes from MongoDB to Apache Kafka.

## Requirements

- Rust
- Cargo
- Docker
- PostgreSQL
- MongoDB

## Running

Each project is a separate Cargo project.

```bash
cd <project>
cargo run
```
