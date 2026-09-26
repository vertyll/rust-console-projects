# Rust Console Projects

A collection of Rust projects covering CLI tools, REST APIs, database integration, and event-driven applications.

## Projects

| Project                   | Description                                              | Technologies                      |
| ------------------------- | -------------------------------------------------------- | --------------------------------- |
| `cat`                     | Basic implementation of the Unix `cat` command           | Rust, Clap                        |
| `echo`                    | CLI implementation of `echo`                             | Rust, Clap                        |
| `echo-clap`               | `echo` implementation using the `clap` derive API        | Rust, Clap                        |
| `echo-std`                | Minimal `echo` implementation using the standard library | Rust                              |
| `wc`                      | Basic implementation of the Unix `wc` command            | Rust, Clap                        |
| `tail`                    | Basic implementation of the Unix `tail` command          | Rust, Clap, Regex                 |
| `regex-once`              | Example of reusing a compiled regular expression         | Rust, Regex, Once Cell            |
| `clap-examples`           | Examples of building CLI applications with `clap`        | Rust, Clap                        |
| `rest-backed-by-postgres` | REST API backed by PostgreSQL                            | Rust, Actix Web, SQLx, PostgreSQL |
| `rest-db-factory`         | REST API with an abstract database access layer          | Rust, Actix Web, SQLx, PostgreSQL |
| `cdc-mongodb`             | Change Data Capture pipeline from MongoDB to Kafka       | Rust, MongoDB, Kafka, rdkafka     |

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
