// src/db/mod.rs

mod db_access_trait;
mod postgres_db;
mod mongo_db;

pub use db_access_trait::DatabaseAccess;
pub use postgres_db::PostgresDB;
// Eksportuj MongoDB gdy będzie gotowy
