use mykobo_rs::message_bus::models::base::PaymentDirection;
use mykobo_rs::message_bus::models::event::*;
use mykobo_rs::message_bus::models::instruction::*;
use mykobo_rs::message_bus::{
    EventType, InstructionType, MessageBusMessage, MetaData, Payload, TransactionType,
};

#[test]
fn test_metadata_valid() {
    let metadata = MetaData::new(
        "BANKING_SERVICE".to_string(),
        "2021-01-01T00:00:00Z".to_string(),
        "test.token.here".to_string(),
        "unique-key-123".to_string(),
        Some(InstructionType::Payment),
        None,
        None,
    );
    assert!(metadata.is_ok());
}

#[test]
fn test_metadata_missing_instruction_and_event() {
    let metadata = MetaData::new(
        "BANKING_SERVICE".to_string(),
        "2021-01-01T00:00:00Z".to_string(),
        "test.token.here".to_string(),
        "unique-key-123".to_string(),
        None,
        None,
        Some("127.0.0.1".to_string()),
    );
    assert!(metadata.is_err());
}

#[test]
fn test_metadata_rejects_both_instruction_and_event() {
    let metadata = MetaData::new(
        "BANKING_SERVICE".to_string(),
        "2021-01-01T00:00:00Z".to_string(),
        "test.token.here".to_string(),
        "unique-key-123".to_string(),
        Some(InstructionType::Payment),
        Some(EventType::NewTransaction),
        Some("127.0.0.1".to_string()),
    );
    assert!(metadata.is_err());
    if let Err(e) = metadata {
        assert!(e
            .fields
            .contains(&"cannot specify both instruction_type and event".to_string()));
    }
}

#[test]
fn test_message_create_with_instruction() {
    let payload = PaymentPayload::new(
        "P763763453G".to_string(),
        "EUR".to_string(),
        "123.00".to_string(),
        "BANK_MODULR".to_string(),
        PaymentDirection::Inbound,
        "MYK123344545".to_string(),
        Some("John Doe".to_string()),
        Some("GB123266734836738787454".to_string()),
    )
    .unwrap();

    let message = MessageBusMessage::create(
        "BANKING_SERVICE".to_string(),
        Payload::Payment(payload),
        "test.token.here".to_string(),
        Some(InstructionType::Payment),
        None,
        None,
        Some("127.0.0.1".to_string()),
    );

    assert!(message.is_ok());
    let msg = message.unwrap();
    assert_eq!(msg.meta_data.source, "BANKING_SERVICE");
    assert_eq!(
        msg.meta_data.instruction_type,
        Some(InstructionType::Payment)
    );
}

#[test]
fn test_message_create_with_event() {
    let payload = NewTransactionEventPayload::new(
        "2021-01-01T00:00:00Z".to_string(),
        TransactionType::Deposit,
        "TXN123".to_string(),
        "BANKING_SERVICE".to_string(),
    )
    .unwrap();

    let message = MessageBusMessage::create(
        "BANKING_SERVICE".to_string(),
        Payload::NewTransaction(payload),
        "test.token.here".to_string(),
        None,
        Some(EventType::NewTransaction),
        None,
        None,
    );

    assert!(message.is_ok());
}

#[test]
fn test_message_create_rejects_both_instruction_and_event() {
    let payload = PaymentPayload::new(
        "P763763453G".to_string(),
        "EUR".to_string(),
        "123.00".to_string(),
        "BANK_MODULR".to_string(),
        PaymentDirection::Outbound,
        "MYK123344545".to_string(),
        None,
        None,
    )
    .unwrap();

    let message = MessageBusMessage::create(
        "BANKING_SERVICE".to_string(),
        Payload::Payment(payload),
        "test.token.here".to_string(),
        Some(InstructionType::Payment),
        Some(EventType::NewTransaction),
        None,
        None,
    );

    assert!(message.is_err());
}

#[test]
fn test_message_validates_payload_type() {
    let payload = StatusUpdatePayload::new(
        "REF123".to_string(),
        "PENDING".to_string(),
        Some("Test".to_string()),
        None,
    )
    .unwrap();

    // Wrong payload type for PAYMENT instruction
    let metadata = MetaData::new(
        "BANKING_SERVICE".to_string(),
        "2021-01-01T00:00:00Z".to_string(),
        "test.token".to_string(),
        "key-123".to_string(),
        Some(InstructionType::Payment),
        None,
        None,
    )
    .unwrap();

    let message = MessageBusMessage::new(metadata, Payload::StatusUpdate(payload));
    assert!(message.is_err());
}

#[test]
fn test_raw_payload_creation() {
    let raw_data = r#"{"custom_field": "value", "another_field": 123}"#.to_string();
    let payload = Payload::Raw(raw_data.clone());

    // Verify the payload contains the expected data
    match payload {
        Payload::Raw(data) => assert_eq!(data, raw_data),
        _ => panic!("Expected Raw payload variant"),
    }
}

#[test]
fn test_raw_payload_serialization() {
    let raw_data = r#"{"custom_field": "value"}"#.to_string();
    let payload = Payload::Raw(raw_data.clone());

    // Serialize the payload
    let serialized = serde_json::to_string(&payload).unwrap();

    // Deserialize it back
    let deserialized: Payload = serde_json::from_str(&serialized).unwrap();

    // Verify the deserialized payload matches
    match deserialized {
        Payload::Raw(data) => assert_eq!(data, raw_data),
        _ => panic!("Expected Raw payload variant after deserialization"),
    }
}

#[test]
fn test_raw_payload_with_plain_string() {
    let raw_data = "Simple text message".to_string();
    let payload = Payload::Raw(raw_data.clone());

    match payload {
        Payload::Raw(data) => assert_eq!(data, "Simple text message"),
        _ => panic!("Expected Raw payload variant"),
    }
}

#[test]
fn test_raw_payload_with_message_bus_message() {
    // Test Raw payload with instruction type
    let raw_data = r#"{"custom": "data"}"#.to_string();
    let message = MessageBusMessage::create(
        "TEST_SERVICE".to_string(),
        Payload::Raw(raw_data.clone()),
        "test.token.here".to_string(),
        Some(InstructionType::Payment),
        None,
        None,
        None,
    );

    assert!(message.is_ok());
    let msg = message.unwrap();
    match msg.payload {
        Payload::Raw(data) => assert_eq!(data, raw_data),
        _ => panic!("Expected Raw payload"),
    }
}

#[test]
fn test_raw_payload_with_event_type() {
    // Test Raw payload with event type
    let raw_data = "raw event data".to_string();
    let message = MessageBusMessage::create(
        "TEST_SERVICE".to_string(),
        Payload::Raw(raw_data.clone()),
        "test.token.here".to_string(),
        None,
        Some(EventType::NewTransaction),
        None,
        None,
    );

    assert!(message.is_ok());
    let msg = message.unwrap();
    match msg.payload {
        Payload::Raw(data) => assert_eq!(data, raw_data),
        _ => panic!("Expected Raw payload"),
    }
}

#[test]
fn test_raw_payload_bypasses_type_validation() {
    // Test that Raw payload doesn't require matching instruction/event type
    let metadata = MetaData::new(
        "TEST_SERVICE".to_string(),
        "2021-01-01T00:00:00Z".to_string(),
        "test.token".to_string(),
        "key-123".to_string(),
        Some(InstructionType::Payment),
        None,
        None,
    )
    .unwrap();

    // This should succeed even though Payment instruction type doesn't match Raw payload
    let message = MessageBusMessage::new(metadata, Payload::Raw("anything".to_string()));
    assert!(message.is_ok());
}

#[test]
fn test_message_with_bank_payment_request() {
    let payload = BankPaymentRequestPayload::new(
        "REF123".to_string(),
        "100.00".to_string(),
        "USD".to_string(),
        "PROF123".to_string(),
        Some("Payment".to_string()),
    )
    .unwrap();

    let message = MessageBusMessage::create(
        "BANKING_SERVICE".to_string(),
        Payload::BankPaymentRequest(payload),
        "test.token.here".to_string(),
        Some(InstructionType::BankPaymentRequest),
        None,
        None,
        None,
    );

    assert!(message.is_ok());
    let msg = message.unwrap();
    assert_eq!(
        msg.meta_data.instruction_type,
        Some(InstructionType::BankPaymentRequest)
    );
}

#[test]
fn test_message_with_chain_payment() {
    let payload = ChainPaymentPayload::new(
        "STELLAR".to_string(),
        "0xabc123".to_string(),
        "REF456".to_string(),
        "CONFIRMED".to_string(),
        None,
    )
    .unwrap();

    let message = MessageBusMessage::create(
        "CHAIN_SERVICE".to_string(),
        Payload::ChainPayment(payload),
        "test.token.here".to_string(),
        Some(InstructionType::ChainPayment),
        None,
        None,
        Some("127.0.0.1".to_string()),
    );

    assert!(message.is_ok());
}

#[test]
fn test_message_validates_new_payload_types() {
    // Test that wrong payload type still fails for new types
    let payload = BankPaymentRequestPayload::new(
        "REF123".to_string(),
        "100.00".to_string(),
        "USD".to_string(),
        "PROF123".to_string(),
        None,
    )
    .unwrap();

    let metadata = MetaData::new(
        "BANKING_SERVICE".to_string(),
        "2021-01-01T00:00:00Z".to_string(),
        "test.token".to_string(),
        "key-123".to_string(),
        Some(InstructionType::Payment), // Wrong instruction type
        None,
        None,
    )
    .unwrap();

    let message = MessageBusMessage::new(metadata, Payload::BankPaymentRequest(payload));
    assert!(message.is_err());
}

#[test]
fn test_message_with_new_user_event() {
    let payload = NewUserEventPayload::new("New User".to_string(), "USER123".to_string()).unwrap();

    let message = MessageBusMessage::create(
        "IDENTITY_SERVICE".to_string(),
        Payload::NewUser(payload),
        "test.token.here".to_string(),
        None,
        Some(EventType::NewUser),
        None,
        Some("127.0.0.1".to_string()),
    );

    assert!(message.is_ok());
    let msg = message.unwrap();
    assert_eq!(msg.meta_data.event, Some(EventType::NewUser));
}

#[test]
fn test_message_with_bank_payment_event() {
    let payload = BankPaymentEventPayload::new(
        "TX12345".to_string(),
        "COMPLETED".to_string(),
        "REF789".to_string(),
        Some("Payment processed".to_string()),
    )
    .unwrap();

    let message = MessageBusMessage::create(
        "BANK_SERVICE".to_string(),
        Payload::BankPayment(payload),
        "test.token.here".to_string(),
        None,
        Some(EventType::BankPayment),
        None,
        Some("127.0.0.1".to_string()),
    );

    assert!(message.is_ok());
    let msg = message.unwrap();
    assert_eq!(msg.meta_data.event, Some(EventType::BankPayment));
}

#[test]
fn test_bank_payment_event_validation_fails_with_wrong_type() {
    let payload = BankPaymentEventPayload::new(
        "TX12345".to_string(),
        "COMPLETED".to_string(),
        "REF789".to_string(),
        None,
    )
    .unwrap();

    // Try to use wrong event type
    let metadata = MetaData::new(
        "BANK_SERVICE".to_string(),
        "2021-01-01T00:00:00Z".to_string(),
        "test.token".to_string(),
        "key-123".to_string(),
        None,
        Some(EventType::Payment), // Wrong event type
        Some("127.0.0.1".to_string()),
    )
    .unwrap();

    let message = MessageBusMessage::new(metadata, Payload::BankPayment(payload));
    assert!(message.is_err());
}

// Tests for ip_address field
#[test]
fn test_metadata_with_ip_address() {
    let metadata = MetaData::new(
        "BANKING_SERVICE".to_string(),
        "2021-01-01T00:00:00Z".to_string(),
        "test.token.here".to_string(),
        "unique-key-123".to_string(),
        Some(InstructionType::Payment),
        None,
        Some("192.168.1.1".to_string()),
    );
    assert!(metadata.is_ok());
    let meta = metadata.unwrap();
    assert_eq!(meta.ip_address, Some("192.168.1.1".to_string()));
}

#[test]
fn test_metadata_with_ipv6_address() {
    let metadata = MetaData::new(
        "BANKING_SERVICE".to_string(),
        "2021-01-01T00:00:00Z".to_string(),
        "test.token.here".to_string(),
        "unique-key-123".to_string(),
        Some(InstructionType::Payment),
        None,
        Some("2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string()),
    );
    assert!(metadata.is_ok());
    let meta = metadata.unwrap();
    assert_eq!(
        meta.ip_address,
        Some("2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string())
    );
}

#[test]
fn test_metadata_without_ip_address() {
    let metadata = MetaData::new(
        "BANKING_SERVICE".to_string(),
        "2021-01-01T00:00:00Z".to_string(),
        "test.token.here".to_string(),
        "unique-key-123".to_string(),
        Some(InstructionType::Payment),
        None,
        None,
    );
    assert!(metadata.is_ok());
    let meta = metadata.unwrap();
    assert_eq!(meta.ip_address, None);
}

#[test]
fn test_message_create_with_ip_address() {
    let payload = PaymentPayload::new(
        "P763763453G".to_string(),
        "EUR".to_string(),
        "123.00".to_string(),
        "BANK_MODULR".to_string(),
        PaymentDirection::Both,
        "MYK123344545".to_string(),
        Some("John Doe".to_string()),
        Some("GB123266734836738787454".to_string()),
    )
    .unwrap();

    let message = MessageBusMessage::create(
        "BANKING_SERVICE".to_string(),
        Payload::Payment(payload),
        "test.token.here".to_string(),
        Some(InstructionType::Payment),
        None,
        None,
        Some("203.0.113.45".to_string()),
    );

    assert!(message.is_ok());
    let msg = message.unwrap();
    assert_eq!(msg.meta_data.ip_address, Some("203.0.113.45".to_string()));
}

#[test]
fn test_message_create_without_ip_address() {
    let payload = PaymentPayload::new(
        "P763763453G".to_string(),
        "EUR".to_string(),
        "123.00".to_string(),
        "BANK_MODULR".to_string(),
        PaymentDirection::Inbound,
        "MYK123344545".to_string(),
        None,
        None,
    )
    .unwrap();

    let message = MessageBusMessage::create(
        "BANKING_SERVICE".to_string(),
        Payload::Payment(payload),
        "test.token.here".to_string(),
        Some(InstructionType::Payment),
        None,
        None,
        None,
    );

    assert!(message.is_ok());
    let msg = message.unwrap();
    assert_eq!(msg.meta_data.ip_address, None);
}

#[test]
fn test_ip_address_with_localhost() {
    let metadata = MetaData::new(
        "BANKING_SERVICE".to_string(),
        "2021-01-01T00:00:00Z".to_string(),
        "test.token.here".to_string(),
        "unique-key-123".to_string(),
        None,
        Some(EventType::Payment),
        Some("localhost".to_string()),
    );
    assert!(metadata.is_ok());
    let meta = metadata.unwrap();
    assert_eq!(meta.ip_address, Some("localhost".to_string()));
}

#[test]
fn test_ip_address_with_loopback() {
    let metadata = MetaData::new(
        "BANKING_SERVICE".to_string(),
        "2021-01-01T00:00:00Z".to_string(),
        "test.token.here".to_string(),
        "unique-key-123".to_string(),
        None,
        Some(EventType::Payment),
        Some("::1".to_string()),
    );
    assert!(metadata.is_ok());
    let meta = metadata.unwrap();
    assert_eq!(meta.ip_address, Some("::1".to_string()));
}

#[test]
fn test_ip_address_serialization() {
    let metadata = MetaData::new(
        "BANKING_SERVICE".to_string(),
        "2021-01-01T00:00:00Z".to_string(),
        "test.token.here".to_string(),
        "unique-key-123".to_string(),
        Some(InstructionType::Payment),
        None,
        Some("10.0.0.1".to_string()),
    )
    .unwrap();

    let serialized = serde_json::to_string(&metadata).unwrap();
    assert!(serialized.contains("\"ip_address\":\"10.0.0.1\""));
}

#[test]
fn test_ip_address_omitted_when_none() {
    let metadata = MetaData::new(
        "BANKING_SERVICE".to_string(),
        "2021-01-01T00:00:00Z".to_string(),
        "test.token.here".to_string(),
        "unique-key-123".to_string(),
        Some(InstructionType::Payment),
        None,
        None,
    )
    .unwrap();

    let serialized = serde_json::to_string(&metadata).unwrap();
    // Due to #[serde_with::skip_serializing_none], ip_address should not be in the JSON
    assert!(!serialized.contains("ip_address"));
}

#[test]
fn test_ip_address_deserialization() {
    let json = r#"{
        "source": "BANKING_SERVICE",
        "created_at": "2021-01-01T00:00:00Z",
        "token": "test.token.here",
        "idempotency_key": "unique-key-123",
        "instruction_type": "PAYMENT",
        "ip_address": "172.16.0.1"
    }"#;

    let metadata: MetaData = serde_json::from_str(json).unwrap();
    assert_eq!(metadata.ip_address, Some("172.16.0.1".to_string()));
}

#[test]
fn test_ip_address_deserialization_missing_field() {
    let json = r#"{
        "source": "BANKING_SERVICE",
        "created_at": "2021-01-01T00:00:00Z",
        "token": "test.token.here",
        "idempotency_key": "unique-key-123",
        "instruction_type": "PAYMENT"
    }"#;

    let metadata: MetaData = serde_json::from_str(json).unwrap();
    assert_eq!(metadata.ip_address, None);
}

#[test]
fn test_message_with_update_profile() {
    let payload = UpdateProfilePayload::new(
        "PROF123".to_string(),
        Some("123 Main Street".to_string()),
        Some("Apt 4B".to_string()),
        Some("GB12345678901234".to_string()),
        Some("123456".to_string()),
        None,
        None,
        Some("GB".to_string()),
        None,
        None,
    );

    let message = MessageBusMessage::create(
        "IDENTITY_SERVICE".to_string(),
        Payload::UpdateProfile(payload),
        "test.token.here".to_string(),
        Some(InstructionType::UpdateProfile),
        None,
        None,
        Some("127.0.0.1".to_string()),
    );

    assert!(message.is_ok());
    let msg = message.unwrap();
    assert_eq!(
        msg.meta_data.instruction_type,
        Some(InstructionType::UpdateProfile)
    );
}

#[test]
fn test_message_update_profile_validates_payload_type() {
    let payload = UpdateProfilePayload::new(
        "PROF456".to_string(),
        Some("123 Main Street".to_string()),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );

    // Wrong instruction type for UpdateProfile payload
    let metadata = MetaData::new(
        "IDENTITY_SERVICE".to_string(),
        "2021-01-01T00:00:00Z".to_string(),
        "test.token".to_string(),
        "key-123".to_string(),
        Some(InstructionType::Payment), // Wrong instruction type
        None,
        None,
    )
    .unwrap();

    let message = MessageBusMessage::new(metadata, Payload::UpdateProfile(payload));
    assert!(message.is_err());
}

#[test]
fn test_message_with_mint_instruction() {
    let payload = MintPayload::new(
        "100.00".to_string(),
        "EUR".to_string(),
        "MYK_MINT_001".to_string(),
        "stellar".to_string(),
        Some("Mint for deposit".to_string()),
    )
    .unwrap();

    let message = MessageBusMessage::create(
        "LEDGER_SERVICE".to_string(),
        Payload::Mint(payload),
        "test.token.here".to_string(),
        Some(InstructionType::Mint),
        None,
        None,
        None,
    );

    assert!(message.is_ok());
    let msg = message.unwrap();
    assert_eq!(msg.meta_data.instruction_type, Some(InstructionType::Mint));
}

#[test]
fn test_message_with_burn_instruction() {
    let payload = BurnPayload::new(
        "75.00".to_string(),
        "EUR".to_string(),
        "MYK_BURN_001".to_string(),
        "stellar".to_string(),
        Some("Burn for withdrawal".to_string()),
    )
    .unwrap();

    let message = MessageBusMessage::create(
        "LEDGER_SERVICE".to_string(),
        Payload::Burn(payload),
        "test.token.here".to_string(),
        Some(InstructionType::Burn),
        None,
        None,
        None,
    );

    assert!(message.is_ok());
    let msg = message.unwrap();
    assert_eq!(msg.meta_data.instruction_type, Some(InstructionType::Burn));
}

#[test]
fn test_message_mint_validates_payload_type() {
    let payload = MintPayload::new(
        "100.00".to_string(),
        "EUR".to_string(),
        "MYK123".to_string(),
        "stellar".to_string(),
        None,
    )
    .unwrap();

    // Wrong instruction type for Mint payload
    let metadata = MetaData::new(
        "LEDGER_SERVICE".to_string(),
        "2021-01-01T00:00:00Z".to_string(),
        "test.token".to_string(),
        "key-123".to_string(),
        Some(InstructionType::Payment), // Wrong instruction type
        None,
        None,
    )
    .unwrap();

    let message = MessageBusMessage::new(metadata, Payload::Mint(payload));
    assert!(message.is_err());
}

#[test]
fn test_message_burn_validates_payload_type() {
    let payload = BurnPayload::new(
        "75.00".to_string(),
        "EUR".to_string(),
        "MYK456".to_string(),
        "stellar".to_string(),
        None,
    )
    .unwrap();

    // Wrong instruction type for Burn payload
    let metadata = MetaData::new(
        "LEDGER_SERVICE".to_string(),
        "2021-01-01T00:00:00Z".to_string(),
        "test.token".to_string(),
        "key-456".to_string(),
        Some(InstructionType::Payment), // Wrong instruction type
        None,
        None,
    )
    .unwrap();

    let message = MessageBusMessage::new(metadata, Payload::Burn(payload));
    assert!(message.is_err());
}

#[test]
fn test_message_mint_serialization_roundtrip() {
    let payload = MintPayload::new(
        "100.00".to_string(),
        "EUR".to_string(),
        "MYK_MINT_RT".to_string(),
        "stellar".to_string(),
        Some("Roundtrip test".to_string()),
    )
    .unwrap();

    let message = MessageBusMessage::create(
        "LEDGER_SERVICE".to_string(),
        Payload::Mint(payload),
        "test.token.here".to_string(),
        Some(InstructionType::Mint),
        None,
        Some("idem-key-mint".to_string()),
        None,
    )
    .unwrap();

    let serialized: String = message.clone().into();
    let deserialized: MessageBusMessage = serde_json::from_str(&serialized).unwrap();

    assert_eq!(message, deserialized);
}

#[test]
fn test_message_burn_serialization_roundtrip() {
    let payload = BurnPayload::new(
        "75.00".to_string(),
        "EUR".to_string(),
        "MYK_BURN_RT".to_string(),
        "stellar".to_string(),
        None,
    )
    .unwrap();

    let message = MessageBusMessage::create(
        "LEDGER_SERVICE".to_string(),
        Payload::Burn(payload),
        "test.token.here".to_string(),
        Some(InstructionType::Burn),
        None,
        Some("idem-key-burn".to_string()),
        None,
    )
    .unwrap();

    let serialized: String = message.clone().into();
    let deserialized: MessageBusMessage = serde_json::from_str(&serialized).unwrap();

    assert_eq!(message, deserialized);
}
