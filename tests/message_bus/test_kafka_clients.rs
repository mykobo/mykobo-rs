use mykobo_rs::message_bus::kafka::consumer::EventConsumer;
use mykobo_rs::message_bus::kafka::producer::EventProducer;
use mykobo_rs::models::error::KafkaError;
use serial_test::serial;
use std::env;

#[allow(unused_imports)]
use mykobo_rs::message_bus::kafka::models::IncomingMessage;

/// Helper to clear Kafka-related env vars for a clean test state.
fn clear_kafka_env() {
    env::remove_var("KAFKA_API_KEY");
    env::remove_var("KAFKA_API_SECRET");
    env::remove_var("KAFKA_API_PROTOCOL");
    env::remove_var("KAFKA_API_SASL_MECHANISM");
}

fn set_kafka_credentials() {
    env::set_var("KAFKA_API_KEY", "test-key");
    env::set_var("KAFKA_API_SECRET", "test-secret");
}

// ─── Consumer tests ──────────────────────────────────────────────────────────

#[tokio::test]
#[serial]
async fn test_consumer_creation_with_plaintext_protocol() {
    clear_kafka_env();
    env::set_var("KAFKA_API_PROTOCOL", "PLAINTEXT");

    let (tx, _rx) = tokio::sync::mpsc::channel::<IncomingMessage<serde_json::Value>>(1);

    let consumer = EventConsumer::<serde_json::Value>::new(
        "localhost:9092",
        "test-group",
        "test-client",
        3,
        &["test-topic"],
        tx,
    );

    assert!(consumer.is_ok(), "Consumer should be created with PLAINTEXT protocol without credentials");
}

#[tokio::test]
#[serial]
async fn test_consumer_creation_fails_without_api_key_for_sasl_ssl() {
    clear_kafka_env();
    // SASL_SSL is the default when KAFKA_API_PROTOCOL is not set

    let (tx, _rx) = tokio::sync::mpsc::channel::<IncomingMessage<serde_json::Value>>(1);

    let result = EventConsumer::<serde_json::Value>::new(
        "localhost:9092",
        "test-group",
        "test-client",
        3,
        &["test-topic"],
        tx,
    );

    match result {
        Err(KafkaError::ClientCreation(msg)) => {
            assert!(msg.contains("KAFKA_API_KEY"), "Error should mention KAFKA_API_KEY, got: {msg}");
        }
        Err(e) => panic!("Expected ClientCreation error, got: {e:?}"),
        Ok(_) => panic!("Expected error but consumer was created successfully"),
    }
}

#[tokio::test]
#[serial]
async fn test_consumer_creation_fails_without_api_secret_for_sasl_ssl() {
    clear_kafka_env();
    env::set_var("KAFKA_API_KEY", "test-key");
    // KAFKA_API_SECRET is not set

    let (tx, _rx) = tokio::sync::mpsc::channel::<IncomingMessage<serde_json::Value>>(1);

    let result = EventConsumer::<serde_json::Value>::new(
        "localhost:9092",
        "test-group",
        "test-client",
        3,
        &["test-topic"],
        tx,
    );

    match result {
        Err(KafkaError::ClientCreation(msg)) => {
            assert!(msg.contains("KAFKA_API_SECRET"), "Error should mention KAFKA_API_SECRET, got: {msg}");
        }
        Err(e) => panic!("Expected ClientCreation error, got: {e:?}"),
        Ok(_) => panic!("Expected error but consumer was created successfully"),
    }
}

#[tokio::test]
#[serial]
async fn test_consumer_creation_with_sasl_ssl_credentials() {
    clear_kafka_env();
    set_kafka_credentials();

    let (tx, _rx) = tokio::sync::mpsc::channel::<IncomingMessage<serde_json::Value>>(1);

    let consumer = EventConsumer::<serde_json::Value>::new(
        "localhost:9092",
        "test-group",
        "test-client",
        3,
        &["test-topic"],
        tx,
    );

    assert!(consumer.is_ok(), "Consumer should be created with valid SASL_SSL credentials");
}

// ─── Producer tests ──────────────────────────────────────────────────────────

#[test]
#[serial]
#[should_panic(expected = "Missing KAFKA_API_KEY")]
fn test_producer_creation_panics_without_api_key() {
    clear_kafka_env();

    let _ = EventProducer::new("localhost:9092", 5000, "test-topic");
}

#[test]
#[serial]
#[should_panic(expected = "Missing KAFKA_API_SECRET")]
fn test_producer_creation_panics_without_api_secret() {
    clear_kafka_env();
    env::set_var("KAFKA_API_KEY", "test-key");

    let _ = EventProducer::new("localhost:9092", 5000, "test-topic");
}

#[test]
#[serial]
fn test_producer_creation_with_valid_credentials() {
    clear_kafka_env();
    set_kafka_credentials();

    let producer = EventProducer::new("localhost:9092", 5000, "test-topic");

    assert!(producer.is_ok(), "Producer should be created with valid credentials");
}
