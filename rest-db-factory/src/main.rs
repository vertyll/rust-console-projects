mod db;
mod handlers;
mod models;
mod routes;
mod state;
mod utils;

// src/main.rs

use std::{env, io};
use dotenv::dotenv;
use sqlx::PgPool;
use crate::db::{PostgresDB, DatabaseAccess};
// ... reszta importów

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // ... kod inicjalizujący

    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL is not set in .env file");

    let db_pool = PgPool::connect(&database_url).await.unwrap();
    let db = PostgresDB::new(db_pool);

    db.ping().await.expect("Failed to connect to Postgres.");

    // ... reszta kodu
}
