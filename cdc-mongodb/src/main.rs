mod mongo_connection;
mod change_stream_handlers;
mod models;
mod kafka_producer;

use dotenv::dotenv;
use std::env;
use mongo_connection::connect_to_mongo;
use change_stream_handlers::watch_changes;
use crate::kafka_producer::KafkaProducer;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mongo_url = env::var("MONGO_URL")
        .expect("MONGO_URL is not set in .env file");
    let database_name = env::var("DATABASE_NAME")
        .expect("DATABASE_NAME is not set in .env file");
    let collection_name = env::var("COLLECTION_NAME")
        .expect("COLLECTION_NAME is not set in .env file");
    let kafka_server = env::var("KAFKA_SERVER")
        .expect("KAFKA_SERVER is not set in .env file");
    let kafka_topic = env::var("KAFKA_TOPIC")
        .expect("KAFKA_TOPIC is not set in .env file");

    let kafka_producer = KafkaProducer::new(&kafka_server, &kafka_topic);
    println!("Connected to Kafka");
    let client = connect_to_mongo(&mongo_url).await
        .expect("Error in connection to MongoDB");
    println!("Connected to MongoDB");
    println!("Starting listening for changes...");
    watch_changes(&client, &kafka_producer, &database_name, &collection_name).await;
}
