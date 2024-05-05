use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use std::io;
use std::sync::Mutex;

mod db_access;
mod handlers;
mod models;
mod routes;
mod state;

use routes::*;
use state::AppState;
use crate::db_access::ping_db;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await.unwrap();
    println!("Db pool created with settings: {:?}", db_pool.options());

    ping_db(&db_pool).await.expect("Failed to connect to Postgres.");
    println!("Connected to Postgres!");

    // Construct App State
    let shared_data = web::Data::new(AppState {
        health_check_response: "Health check ok, check count ".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    //Construct app and configure routes
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(movie_routes)
    };

    let hostname_port = env::var("SERVER_HOSTNAME_PORT")
        .expect("SERVER_HOSTNAME_PORT is not set in .env file");

    //Start HTTP server
    println!("Starting server at: {}", hostname_port);
    HttpServer::new(app).bind(hostname_port).unwrap().run().await

}