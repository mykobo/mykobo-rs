use mykobo_rs::message_bus::{EventType, InstructionType, MetaData};
use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};

/// A simple message envelope for testing MetaData serialization
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde_with::skip_serializing_none]
struct MessageEnvelope<T> {
    pub meta_data: MetaData,
    pub payload: T,
}

#[test]
fn test_message_serialisation() {
    let meta_data = MetaData::new(
        "TestFunction".to_string(),
        "2025-07-26T18:16:52Z".to_string(),
        "eyw83485yh9wehf89wr9gbsdfgksfg".to_string(),
        "02fb1ad1-103e-49a3-bdcc-4fba95f0f573".to_string(),
        Some(InstructionType::Payment),
        None,
        Some("127.0.0.1".to_string()),
    )
    .unwrap();

    let message = MessageEnvelope {
        meta_data,
        payload: "Test Payload".to_string(),
    };

    let serialized = serde_json::to_string(&message).unwrap();

    // Parse both as JSON to compare semantically rather than string comparison
    // (field order in JSON doesn't matter semantically)
    let actual: serde_json::Value = serde_json::from_str(&serialized).unwrap();
    let expected: serde_json::Value = serde_json::from_str(
        r#"{"meta_data":{"source":"TestFunction","created_at":"2025-07-26T18:16:52Z","token":"eyw83485yh9wehf89wr9gbsdfgksfg","idempotency_key":"02fb1ad1-103e-49a3-bdcc-4fba95f0f573","instruction_type":"PAYMENT","ip_address":"127.0.0.1"},"payload":"Test Payload"}"#,
    )
    .unwrap();

    assert_eq!(actual, expected);
}

#[test]
fn test_message_serialisation_with_ipv4() {
    let meta_data = MetaData::new(
        "PaymentService".to_string(),
        "2025-07-26T18:16:52Z".to_string(),
        "test_token_value".to_string(),
        "unique-key-456".to_string(),
        Some(InstructionType::StatusUpdate),
        None,
        Some("192.168.1.100".to_string()),
    )
    .unwrap();

    let message = MessageEnvelope {
        meta_data,
        payload: r#"{"status":"completed"}"#.to_string(),
    };

    let serialized = serde_json::to_string(&message).unwrap();
    let deserialized: MessageEnvelope<String> = serde_json::from_str(&serialized).unwrap();

    assert_eq!(
        deserialized.meta_data.ip_address,
        Some("192.168.1.100".to_string())
    );
}

#[test]
fn test_message_serialisation_with_ipv6() {
    let meta_data = MetaData::new(
        "IdentityService".to_string(),
        "2025-07-26T18:16:52Z".to_string(),
        "token123".to_string(),
        "idempotency-789".to_string(),
        None,
        Some(EventType::NewUser),
        Some("2001:db8::1".to_string()),
    )
    .unwrap();

    let message = MessageEnvelope {
        meta_data,
        payload: r#"{"user_id":"USER123"}"#.to_string(),
    };

    let serialized = serde_json::to_string(&message).unwrap();
    let deserialized: MessageEnvelope<String> = serde_json::from_str(&serialized).unwrap();

    assert_eq!(
        deserialized.meta_data.ip_address,
        Some("2001:db8::1".to_string())
    );
}

#[test]
fn test_message_serialisation_without_ip_address() {
    let meta_data = MetaData::new(
        "TestFunction".to_string(),
        "2025-07-26T18:16:52Z".to_string(),
        "eyw83485yh9wehf89wr9gbsdfgksfg".to_string(),
        "02fb1ad1-103e-49a3-bdcc-4fba95f0f573".to_string(),
        Some(InstructionType::Payment),
        None,
        None,
    )
    .unwrap();

    let message = MessageEnvelope {
        meta_data,
        payload: "Test Payload".to_string(),
    };

    let serialized = serde_json::to_string(&message).unwrap();

    // When ip_address is None, it should be omitted from serialization
    let actual: serde_json::Value = serde_json::from_str(&serialized).unwrap();

    // Verify ip_address field is not present in the JSON
    assert!(!actual["meta_data"]
        .as_object()
        .unwrap()
        .contains_key("ip_address"));
}

#[test]
fn test_message_deserialization_with_ip_address() {
    let json = r#"{
        "meta_data": {
            "source": "TestFunction",
            "created_at": "2025-07-26T18:16:52Z",
            "token": "eyw83485yh9wehf89wr9gbsdfgksfg",
            "idempotency_key": "02fb1ad1-103e-49a3-bdcc-4fba95f0f573",
            "instruction_type": "PAYMENT",
            "ip_address": "10.0.0.1"
        },
        "payload": "Test Payload"
    }"#;

    let message: MessageEnvelope<String> = serde_json::from_str(json).unwrap();
    assert_eq!(message.meta_data.ip_address, Some("10.0.0.1".to_string()));
}

#[test]
fn test_message_deserialization_without_ip_address_field() {
    let json = r#"{
        "meta_data": {
            "source": "TestFunction",
            "created_at": "2025-07-26T18:16:52Z",
            "token": "eyw83485yh9wehf89wr9gbsdfgksfg",
            "idempotency_key": "02fb1ad1-103e-49a3-bdcc-4fba95f0f573",
            "instruction_type": "PAYMENT"
        },
        "payload": "Test Payload"
    }"#;

    let message: MessageEnvelope<String> = serde_json::from_str(json).unwrap();
    assert_eq!(message.meta_data.ip_address, None);
}

#[test]
fn test_message_roundtrip_with_ip_address() {
    let original_meta_data = MetaData::new(
        "TestService".to_string(),
        "2025-07-26T18:16:52Z".to_string(),
        "token_value".to_string(),
        "key-999".to_string(),
        None,
        Some(EventType::Payment),
        Some("172.16.254.1".to_string()),
    )
    .unwrap();

    let original_message = MessageEnvelope {
        meta_data: original_meta_data,
        payload: "Roundtrip Test".to_string(),
    };

    // Serialize
    let serialized = serde_json::to_string(&original_message).unwrap();

    // Deserialize
    let deserialized: MessageEnvelope<String> = serde_json::from_str(&serialized).unwrap();

    // Verify ip_address is preserved
    assert_eq!(
        deserialized.meta_data.ip_address,
        Some("172.16.254.1".to_string())
    );
    assert_eq!(deserialized.meta_data.source, "TestService");
    assert_eq!(deserialized.payload, "Roundtrip Test");
}
