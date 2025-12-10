use crate::models::error::{KafkaError, KafkaResult};
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::Serialize;
use std::env;
use std::time::Duration;

pub struct EventProducer {
    producer: FutureProducer,
    topic: String,
    timeout: Duration,
}

impl EventProducer {
    pub fn new(brokers: &str, timeout_in_secs: u64, topic: &str) -> KafkaResult<Self> {
        let sasl_username =
            env::var("KAFKA_API_KEY").expect("Missing KAFKA_API_KEY environment variable");
        let sasl_password =
            env::var("KAFKA_API_SECRET").expect("Missing KAFKA_API_SECRET environment variable");

        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", timeout_in_secs.to_string())
            .set("request.timeout.ms", 30000.to_string())
            .set("request.timeout.ms", 60000.to_string())
            .set("compression.type", "gzip")
            .set("retries", "3")
            .set("retry.backoff.ms", "500")
            .set("request.required.acks", "all")
            .set("queue.buffering.max.messages", "100000")
            .set("security.protocol", "SASL_SSL")
            .set("sasl.mechanisms", "PLAIN")
            .set("sasl.username", sasl_username)
            .set("sasl.password", sasl_password)
            .set_log_level(RDKafkaLogLevel::Info)
            .create()
            .map_err(|e| KafkaError::ClientCreation(e.to_string()))?;

        Ok(EventProducer {
            producer,
            topic: topic.to_string(),
            timeout: Duration::from_secs(timeout_in_secs),
        })
    }

    pub async fn send_event<T: Serialize>(&self, key: String, payload: T) -> KafkaResult<()> {
        let payload_json =
            serde_json::to_string(&payload).map_err(|e| KafkaError::MessageSend(e.to_string()))?;
        let record: FutureRecord<String, String> = FutureRecord::to(&self.topic)
            .headers(OwnedHeaders::new().insert(Header {
                key: "source",
                value: Some("kafka_test_rs/producer.rs"),
            }))
            .payload(&payload_json)
            .key(&key);

        self.producer
            .send(record, self.timeout)
            .await
            .map_err(|(err, _)| KafkaError::MessageSend(err.to_string()))?;

        Ok(())
    }
}
