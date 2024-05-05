use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::config::ClientConfig;
use std::time::Duration; // Dodaj import

pub struct KafkaProducer {
    producer: FutureProducer,
    topic: String,
}

impl KafkaProducer {
    pub fn new(brokers: &str, topic: &str) -> KafkaProducer {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .create()
            .expect("Producer creation error");

        KafkaProducer {
            producer,
            topic: topic.to_string(),
        }
    }

    pub async fn send(&self, key: &str, message: &str) {
        let record = FutureRecord::to(&self.topic)
            .key(key)
            .payload(message);
        match self.producer.send(record, Duration::from_secs(0)).await {
            Ok(delivery) => println!("Sent message to {:?}", delivery),
            Err(e) => eprintln!("Error sending message: {:?}", e),
        }
    }
}
