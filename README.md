# Rust Console Projects

A collection of Rust projects covering CLI tools, REST APIs, database integration, and event-driven applications.

## Projects

| Project                   | Description                                              |
| ------------------------- | -------------------------------------------------------- |
| `cat`                     | Basic implementation of the Unix `cat` command           |
| `echo`                    | CLI implementation of `echo`                             |
| `echo-clap`               | `echo` implementation using the `clap` derive API        |
| `echo-std`                | Minimal `echo` implementation using the standard library |
| `wc`                      | Basic implementation of the Unix `wc` command            |
| `tail`                    | Basic implementation of the Unix `tail` command          |
| `regex-once`              | Example of reusing a compiled regular expression         |
| `clap-examples`           | Examples of building CLI applications with `clap`        |
| `rest-backed-by-postgres` | REST API backed by PostgreSQL                            |
| `rest-db-factory`         | REST API with an abstract database access layer          |
| `cdc-mongodb`             | Change Data Capture pipeline from MongoDB to Kafka       |

## Requirements

* Rust
* Cargo
* Docker
* PostgreSQL
* MongoDB

## Running

Each project is a separate Cargo project.

```bash
cd <project>
cargo run
```

Build:

```bash
cargo build
```

Tests:

```bash
cargo test
```

Projects requiring external infrastructure use Docker Compose:

```bash
docker compose up -d
```

## Technologies

* Rust
* Cargo
* Tokio
* Clap
* Actix Web
* SQLx
* PostgreSQL
* MongoDB
* Apache Kafka
* Docker
* Serde
