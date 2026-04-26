use mykobo_rs::message_bus::kafka::consumer::EventConsumer;
use mykobo_rs::message_bus::kafka::producer::{build_message_headers, EventProducer, MESSAGE_SOURCE};
use mykobo_rs::models::error::KafkaError;
use rdkafka::message::Headers;
use serial_test::serial;
use std::collections::HashMap;
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

    assert!(
        consumer.is_ok(),
        "Consumer should be created with PLAINTEXT protocol without credentials"
    );
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
            assert!(
                msg.contains("KAFKA_API_KEY"),
                "Error should mention KAFKA_API_KEY, got: {msg}"
            );
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
            assert!(
                msg.contains("KAFKA_API_SECRET"),
                "Error should mention KAFKA_API_SECRET, got: {msg}"
            );
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

    assert!(
        consumer.is_ok(),
        "Consumer should be created with valid SASL_SSL credentials"
    );
}

// ─── Producer tests ──────────────────────────────────────────────────────────

#[test]
#[serial]
fn test_producer_creation_fails_without_api_key_for_sasl_ssl() {
    clear_kafka_env();
    // SASL_SSL is the default when KAFKA_API_PROTOCOL is not set

    let result = EventProducer::new("localhost:9092", 5000, "test-topic");

    match result {
        Err(KafkaError::ClientCreation(msg)) => {
            assert!(
                msg.contains("KAFKA_API_KEY"),
                "Error should mention KAFKA_API_KEY, got: {msg}"
            );
        }
        Err(e) => panic!("Expected ClientCreation error, got: {e:?}"),
        Ok(_) => panic!("Expected error but producer was created successfully"),
    }
}

#[test]
#[serial]
fn test_producer_creation_fails_without_api_secret_for_sasl_ssl() {
    clear_kafka_env();
    env::set_var("KAFKA_API_KEY", "test-key");
    // KAFKA_API_SECRET is not set

    let result = EventProducer::new("localhost:9092", 5000, "test-topic");

    match result {
        Err(KafkaError::ClientCreation(msg)) => {
            assert!(
                msg.contains("KAFKA_API_SECRET"),
                "Error should mention KAFKA_API_SECRET, got: {msg}"
            );
        }
        Err(e) => panic!("Expected ClientCreation error, got: {e:?}"),
        Ok(_) => panic!("Expected error but producer was created successfully"),
    }
}

#[test]
#[serial]
fn test_producer_creation_with_sasl_ssl_credentials() {
    clear_kafka_env();
    set_kafka_credentials();

    let producer = EventProducer::new("localhost:9092", 5000, "test-topic");

    assert!(
        producer.is_ok(),
        "Producer should be created with valid SASL_SSL credentials"
    );
}

#[test]
#[serial]
fn test_producer_creation_with_plaintext_protocol() {
    clear_kafka_env();
    env::set_var("KAFKA_API_PROTOCOL", "PLAINTEXT");

    let producer = EventProducer::new("localhost:9092", 5000, "test-topic");

    assert!(
        producer.is_ok(),
        "Producer should be created with PLAINTEXT protocol without credentials"
    );
}

// ─── Header tests ────────────────────────────────────────────────────────────

fn headers_to_map(headers: &rdkafka::message::OwnedHeaders) -> HashMap<String, String> {
    headers
        .iter()
        .map(|h| {
            (
                h.key.to_string(),
                h.value
                    .map(|v| std::str::from_utf8(v).unwrap_or("").to_string())
                    .unwrap_or_default(),
            )
        })
        .collect()
}

#[test]
fn test_message_source_uses_package_name_and_version() {
    let expected = format!(
        "{}/{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    assert_eq!(MESSAGE_SOURCE, expected);
}

#[test]
fn test_build_message_headers_sets_source_to_package_name_and_version() {
    let headers = build_message_headers();
    let map = headers_to_map(&headers);

    let source = map
        .get("source")
        .expect("source header should be present");
    assert_eq!(
        source,
        &format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
    );
}

#[test]
fn test_build_message_headers_sets_generated_at() {
    let before = chrono::Utc::now();
    let headers = build_message_headers();
    let after = chrono::Utc::now();

    let map = headers_to_map(&headers);
    let generated_at_str = map
        .get("generated_at")
        .expect("generated_at header should be present");

    assert!(
        !generated_at_str.is_empty(),
        "generated_at header should not be empty"
    );

    let parsed = chrono::DateTime::parse_from_rfc3339(generated_at_str)
        .unwrap_or_else(|e| panic!("generated_at must be RFC3339, got '{generated_at_str}': {e}"))
        .with_timezone(&chrono::Utc);

    assert!(
        parsed >= before && parsed <= after,
        "generated_at ({parsed}) should be between {before} and {after}"
    );
}
