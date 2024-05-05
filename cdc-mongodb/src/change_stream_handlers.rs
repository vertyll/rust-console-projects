use mongodb::{Client, Collection};
use futures::stream::StreamExt;
use mongodb::bson::{doc};
use crate::models::Movie;
use mongodb::change_stream::event::{ChangeStreamEvent, ResumeToken};
use serde_json::json;
use crate::kafka_producer::KafkaProducer;


pub async fn watch_changes(client: &Client, kafka_producer: &KafkaProducer, database: &str, collection: &str) {
    let db = client.database(database);
    let coll: Collection<Movie> = db.collection(collection);

    // Definiowanie pipeline do śledzenia określonych operacji
    let pipeline = vec![
        doc! {
            "$match": {
                "operationType": {
                    "$in": ["insert", "update", "delete", "replace"]
                }
            }
        }
    ];
    println!("Pipeline for cdc created: {:?}", pipeline);

    let mut change_stream = coll.watch(pipeline, None).await.unwrap();

    while let Some(change) = change_stream.next().await {
        match change {
            Ok(event) => process_change_event(event, kafka_producer).await,
            Err(e) => println!("Error occured: {:?}", e),
        }
    }
}

async fn process_change_event(event: ChangeStreamEvent<Movie>, kafka_producer: &KafkaProducer) {
    let resume_token = event.id.clone(); // Pobranie resume token
    println!("Current resume token: {:?}", &resume_token);

    use mongodb::change_stream::event::OperationType::*;

    match event.operation_type {
        Insert => process_insert_event(&event, kafka_producer, &resume_token).await,
        Update => process_update_event(&event, kafka_producer, &resume_token).await,
        Delete => process_delete_event(&event, kafka_producer, &resume_token).await,
        Replace => process_replace_event(&event, kafka_producer, &resume_token).await,
        _ => println!("Unknown operation type: {:?}", event.operation_type),
    }
}

async fn process_replace_event(event: &ChangeStreamEvent<Movie>, kafka_producer: &KafkaProducer, resume_token: &ResumeToken) {
    if let Some(document) = &event.full_document {
        let payload = json!({
            "operationType": "replace",
            "document": document,
            "resumeToken": resume_token
        });
        let message = serde_json::to_string(&payload).unwrap();
        kafka_producer.send("replace", &message).await;
        println!("Replace message sent to kafka: {}", message);
    }
}

async fn process_insert_event(event: &ChangeStreamEvent<Movie>, kafka_producer: &KafkaProducer, resume_token: &ResumeToken) {
    if let Some(document) = &event.full_document {
        let payload = json!({
            "operationType": "insert",
            "document": document,
            "resumeToken": resume_token
        });
        let message = serde_json::to_string(&payload).unwrap();
        kafka_producer.send("insert", &message).await;
        println!("Insert message sent to kafka: {}", message);
    }
}

async fn process_update_event(event: &ChangeStreamEvent<Movie>, kafka_producer: &KafkaProducer, resume_token: &ResumeToken) {
    let key = event.document_key.as_ref().map(|doc| doc.to_string()).unwrap_or_default();
    let update_info = event.update_description.as_ref().map(|upd| serde_json::to_string(upd).unwrap()).unwrap_or_default();

    let payload = json!({
        "operationType": "update",
        "documentKey": key,
        "updateDescription": update_info,
        "resumeToken": resume_token
    });
    let message = serde_json::to_string(&payload).unwrap();
    kafka_producer.send("update", &message).await;
    println!("Update message sent to kafka: {}", message);
}

async fn process_delete_event(event: &ChangeStreamEvent<Movie>, kafka_producer: &KafkaProducer, resume_token: &ResumeToken) {
    let key = event.document_key.as_ref().map(|doc| doc.to_string()).unwrap_or_default();

    let payload = json!({
        "operationType": "delete",
        "documentKey": key,
        "resumeToken": resume_token
    });
    let message = serde_json::to_string(&payload).unwrap();
    kafka_producer.send("delete", &message).await;
    println!("Delete message sent to kafka: {}", message);
}

