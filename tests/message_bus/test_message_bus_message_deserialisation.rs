use mykobo_rs::message_bus::{
    EventType, InstructionType, MessageBusMessage, Payload, TransactionType,
};
use mykobo_rs::message_bus::models::event::*;
use mykobo_rs::message_bus::models::instruction::*;
use pretty_assertions::assert_eq;

/// Test deserialization of MessageBusMessage with Payment payload
#[test]
fn test_deserialize_payment_message() {
    let json = r#"{
        "meta_data": {
            "source": "BANKING_SERVICE",
            "created_at": "2021-01-01T00:00:00Z",
            "token": "test.token.here",
            "idempotency_key": "key-123",
            "instruction_type": "PAYMENT",
            "ip_address": "192.168.1.1"
        },
        "payload": {
            "external_reference": "P763763453G",
            "currency": "EUR",
            "value": "123.00",
            "source": "BANK_MODULR",
            "reference": "MYK123344545",
            "payer_name": "John Doe",
            "bank_account_number": "GB123266734836738787454"
        }
    }"#;

    let message: MessageBusMessage = serde_json::from_str(json).unwrap();

    assert_eq!(message.meta_data.source, "BANKING_SERVICE");
    assert_eq!(message.meta_data.instruction_type, Some(InstructionType::Payment));
    assert_eq!(message.meta_data.ip_address, Some("192.168.1.1".to_string()));

    match message.payload {
        Payload::Payment(payload) => {
            assert_eq!(payload.external_reference, "P763763453G");
            assert_eq!(payload.currency, "EUR");
            assert_eq!(payload.value, "123.00");
            assert_eq!(payload.payer_name, Some("John Doe".to_string()));
        }
        _ => panic!("Expected Payment payload"),
    }
}

/// Test deserialization of MessageBusMessage with Payment payload without optional fields
#[test]
fn test_deserialize_payment_message_without_optionals() {
    let json = r#"{
        "meta_data": {
            "source": "BANKING_SERVICE",
            "created_at": "2021-01-01T00:00:00Z",
            "token": "test.token.here",
            "idempotency_key": "key-456",
            "instruction_type": "PAYMENT"
        },
        "payload": {
            "external_reference": "P999999999",
            "currency": "USD",
            "value": "50.00",
            "source": "BANK_ABC",
            "reference": "REF999"
        }
    }"#;

    let message: MessageBusMessage = serde_json::from_str(json).unwrap();

    assert_eq!(message.meta_data.ip_address, None);

    match message.payload {
        Payload::Payment(payload) => {
            assert_eq!(payload.payer_name, None);
            assert_eq!(payload.bank_account_number, None);
        }
        _ => panic!("Expected Payment payload"),
    }
}

/// Test deserialization of MessageBusMessage with StatusUpdate payload
#[test]
fn test_deserialize_status_update_message() {
    let json = r#"{
        "meta_data": {
            "source": "STATUS_SERVICE",
            "created_at": "2021-02-01T00:00:00Z",
            "token": "status.token",
            "idempotency_key": "status-key-123",
            "instruction_type": "STATUS_UPDATE"
        },
        "payload": {
            "reference": "REF123",
            "status": "COMPLETED",
            "message": "Payment processed successfully",
            "transaction_id": "TXN456"
        }
    }"#;

    let message: MessageBusMessage = serde_json::from_str(json).unwrap();

    assert_eq!(message.meta_data.instruction_type, Some(InstructionType::StatusUpdate));

    match message.payload {
        Payload::StatusUpdate(payload) => {
            assert_eq!(payload.reference, "REF123");
            assert_eq!(payload.status, "COMPLETED");
            assert_eq!(payload.message, Some("Payment processed successfully".to_string()));
            assert_eq!(payload.transaction_id, Some("TXN456".to_string()));
        }
        _ => panic!("Expected StatusUpdate payload"),
    }
}

/// Test deserialization of MessageBusMessage with Correction payload
#[test]
fn test_deserialize_correction_message() {
    let json = r#"{
        "meta_data": {
            "source": "CORRECTION_SERVICE",
            "created_at": "2021-03-01T00:00:00Z",
            "token": "correction.token",
            "idempotency_key": "correction-key-789",
            "instruction_type": "CORRECTION"
        },
        "payload": {
            "reference": "REF456",
            "value": "75.00",
            "message": "Corrected amount due to error",
            "currency": "GBP",
            "source": "BANK_XYZ"
        }
    }"#;

    let message: MessageBusMessage = serde_json::from_str(json).unwrap();

    assert_eq!(message.meta_data.instruction_type, Some(InstructionType::Correction));

    match message.payload {
        Payload::Correction(payload) => {
            assert_eq!(payload.reference, "REF456");
            assert_eq!(payload.value, "75.00");
            assert_eq!(payload.currency, "GBP");
            assert_eq!(payload.message, "Corrected amount due to error");
        }
        _ => panic!("Expected Correction payload"),
    }
}

/// Test roundtrip serialization/deserialization of MessageBusMessage with Transaction payload
#[test]
fn test_roundtrip_transaction_message() {
    let payload = TransactionPayload::new(
        "EXT123".to_string(),
        "BANKING_SERVICE".to_string(),
        "REF789".to_string(),
        "John".to_string(),
        "Doe".to_string(),
        TransactionType::Deposit,
        "PENDING".to_string(),
        "EUR".to_string(),
        "USD".to_string(),
        "100.00".to_string(),
        "1.50".to_string(),
        Some("Bank Account 123".to_string()),
        None,
    )
    .unwrap();

    let message = MessageBusMessage::create(
        "TRANSACTION_SERVICE".to_string(),
        Payload::Transaction(payload),
        "txn.token".to_string(),
        Some(InstructionType::Transaction),
        None,
        None,
        None,
    )
    .unwrap();

    // Serialize and deserialize
    let json = serde_json::to_string(&message).unwrap();
    let deserialized: MessageBusMessage = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.meta_data.instruction_type, Some(InstructionType::Transaction));

    match deserialized.payload {
        Payload::Transaction(payload) => {
            assert_eq!(payload.external_reference, "EXT123");
            assert_eq!(payload.first_name, "John");
            assert_eq!(payload.last_name, "Doe");
            assert_eq!(payload.transaction_type, TransactionType::Deposit);
            assert_eq!(payload.payer, Some("Bank Account 123".to_string()));
        }
        _ => panic!("Expected Transaction payload"),
    }
}

/// Test deserialization of MessageBusMessage with BankPaymentRequest payload
#[test]
fn test_deserialize_bank_payment_request_message() {
    let json = r#"{
        "meta_data": {
            "source": "BANK_SERVICE",
            "created_at": "2021-05-01T00:00:00Z",
            "token": "bank.token",
            "idempotency_key": "bank-key-222",
            "instruction_type": "BANK_PAYMENT_REQUEST"
        },
        "payload": {
            "reference": "BANK_REF123",
            "value": "500.00",
            "currency": "USD",
            "profile_id": "PROF456",
            "message": "Bank transfer request"
        }
    }"#;

    let message: MessageBusMessage = serde_json::from_str(json).unwrap();

    assert_eq!(message.meta_data.instruction_type, Some(InstructionType::BankPaymentRequest));

    match message.payload {
        Payload::BankPaymentRequest(payload) => {
            assert_eq!(payload.reference, "BANK_REF123");
            assert_eq!(payload.value, "500.00");
            assert_eq!(payload.currency, "USD");
            assert_eq!(payload.profile_id, "PROF456");
            assert_eq!(payload.message, Some("Bank transfer request".to_string()));
        }
        _ => panic!("Expected BankPaymentRequest payload"),
    }
}

/// Test roundtrip serialization/deserialization of MessageBusMessage with ChainPayment payload
#[test]
fn test_roundtrip_chain_payment_message() {
    let payload = ChainPaymentPayload::new(
        "STELLAR".to_string(),
        "0xabc123def456".to_string(),
        "REF321".to_string(),
        "CONFIRMED".to_string(),
        Some("TXN789".to_string()),
    )
    .unwrap();

    let message = MessageBusMessage::create(
        "CHAIN_SERVICE".to_string(),
        Payload::ChainPayment(payload),
        "chain.token".to_string(),
        Some(InstructionType::ChainPayment),
        None,
        None,
        Some("10.0.0.1".to_string()),
    )
    .unwrap();

    // Serialize and deserialize
    let json = serde_json::to_string(&message).unwrap();
    let deserialized: MessageBusMessage = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.meta_data.instruction_type, Some(InstructionType::ChainPayment));
    assert_eq!(deserialized.meta_data.ip_address, Some("10.0.0.1".to_string()));

    match deserialized.payload {
        Payload::ChainPayment(payload) => {
            assert_eq!(payload.chain, "STELLAR");
            assert_eq!(payload.hash, "0xabc123def456");
            assert_eq!(payload.status, "CONFIRMED");
            assert_eq!(payload.transaction_id, Some("TXN789".to_string()));
        }
        _ => panic!("Expected ChainPayment payload"),
    }
}

/// Test deserialization of MessageBusMessage with NewTransaction event payload
#[test]
fn test_deserialize_new_transaction_event_message() {
    let json = r#"{
        "meta_data": {
            "source": "EVENT_SERVICE",
            "created_at": "2021-07-01T00:00:00Z",
            "token": "event.token",
            "idempotency_key": "event-key-444",
            "event": "NEW_TRANSACTION"
        },
        "payload": {
            "created_at": "2021-07-01T00:00:00Z",
            "kind": "DEPOSIT",
            "reference": "TXN123",
            "source": "BANKING_SERVICE"
        }
    }"#;

    let message: MessageBusMessage = serde_json::from_str(json).unwrap();

    assert_eq!(message.meta_data.event, Some(EventType::NewTransaction));

    match message.payload {
        Payload::NewTransaction(payload) => {
            assert_eq!(payload.created_at, "2021-07-01T00:00:00Z");
            assert_eq!(payload.kind, TransactionType::Deposit);
            assert_eq!(payload.reference, "TXN123");
            assert_eq!(payload.source, "BANKING_SERVICE");
        }
        _ => panic!("Expected NewTransaction payload"),
    }
}

/// Test roundtrip serialization/deserialization with TransactionStatus event payload
#[test]
fn test_roundtrip_transaction_status_event_message() {
    let payload = TransactionStatusEventPayload::new(
        "TXN456".to_string(),
        "COMPLETED".to_string(),
        Some("EXT789".to_string()),
    )
    .unwrap();

    let message = MessageBusMessage::create(
        "STATUS_EVENT_SERVICE".to_string(),
        Payload::TransactionStatus(payload),
        "status.event.token".to_string(),
        None,
        Some(EventType::TransactionStatusUpdate),
        None,
        None,
    )
    .unwrap();

    // Serialize and deserialize
    let json = serde_json::to_string(&message).unwrap();
    let deserialized: MessageBusMessage = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.meta_data.event, Some(EventType::TransactionStatusUpdate));

    match deserialized.payload {
        Payload::TransactionStatus(payload) => {
            assert_eq!(payload.reference, "TXN456");
            assert_eq!(payload.status, "COMPLETED");
            assert_eq!(payload.external_reference, Some("EXT789".to_string()));
        }
        _ => panic!("Expected TransactionStatus payload"),
    }
}

/// Test deserialization of MessageBusMessage with Payment event payload
#[test]
fn test_deserialize_payment_event_message() {
    let json = r#"{
        "meta_data": {
            "source": "PAYMENT_EVENT_SERVICE",
            "created_at": "2021-09-01T00:00:00Z",
            "token": "payment.event.token",
            "idempotency_key": "payment-event-key-666",
            "event": "PAYMENT"
        },
        "payload": {
            "external_reference": "PAY123",
            "source": "BANK_XYZ",
            "reference": "REF123"
        }
    }"#;

    let message: MessageBusMessage = serde_json::from_str(json).unwrap();

    assert_eq!(message.meta_data.event, Some(EventType::Payment));

    match message.payload {
        Payload::PaymentEvent(payload) => {
            assert_eq!(payload.external_reference, "PAY123");
            assert_eq!(payload.source, "BANK_XYZ");
            assert_eq!(payload.reference, Some("REF123".to_string()));
        }
        _ => panic!("Expected PaymentEvent payload"),
    }
}

/// Test roundtrip serialization/deserialization with BankPayment event payload
#[test]
fn test_roundtrip_bank_payment_event_message() {
    let payload = BankPaymentEventPayload::new(
        "TX12345".to_string(),
        "COMPLETED".to_string(),
        "REF789".to_string(),
        Some("Payment processed successfully".to_string()),
    )
    .unwrap();

    let message = MessageBusMessage::create(
        "BANK_EVENT_SERVICE".to_string(),
        Payload::BankPayment(payload),
        "bank.event.token".to_string(),
        None,
        Some(EventType::BankPayment),
        None,
        Some("172.16.0.1".to_string()),
    )
    .unwrap();

    // Serialize and deserialize
    let json = serde_json::to_string(&message).unwrap();
    let deserialized: MessageBusMessage = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.meta_data.event, Some(EventType::BankPayment));
    assert_eq!(deserialized.meta_data.ip_address, Some("172.16.0.1".to_string()));

    match deserialized.payload {
        Payload::BankPayment(payload) => {
            assert_eq!(payload.transaction_id, "TX12345");
            assert_eq!(payload.status, "COMPLETED");
            assert_eq!(payload.reference, "REF789");
            assert_eq!(payload.message, Some("Payment processed successfully".to_string()));
        }
        _ => panic!("Expected BankPayment payload"),
    }
}

/// Test deserialization of MessageBusMessage with Profile event payload
#[test]
fn test_deserialize_profile_event_message() {
    let json = r#"{
        "meta_data": {
            "source": "PROFILE_SERVICE",
            "created_at": "2021-11-01T00:00:00Z",
            "token": "profile.token",
            "idempotency_key": "profile-key-888",
            "event": "NEW_PROFILE"
        },
        "payload": {
            "title": "New User Profile",
            "identifier": "USER123"
        }
    }"#;

    let message: MessageBusMessage = serde_json::from_str(json).unwrap();

    assert_eq!(message.meta_data.event, Some(EventType::NewProfile));

    match message.payload {
        Payload::Profile(payload) => {
            assert_eq!(payload.title, "New User Profile");
            assert_eq!(payload.identifier, "USER123");
        }
        _ => panic!("Expected Profile payload"),
    }
}

/// Test roundtrip serialization/deserialization with NewUser event payload
#[test]
fn test_roundtrip_new_user_event_message() {
    let payload = NewUserEventPayload::new("New User Registration".to_string(), "USER456".to_string()).unwrap();

    let message = MessageBusMessage::create(
        "USER_SERVICE".to_string(),
        Payload::NewUser(payload),
        "user.token".to_string(),
        None,
        Some(EventType::NewUser),
        None,
        None,
    )
    .unwrap();

    // Serialize and deserialize
    let json = serde_json::to_string(&message).unwrap();
    let deserialized: MessageBusMessage = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.meta_data.event, Some(EventType::NewUser));

    match deserialized.payload {
        Payload::NewUser(payload) => {
            assert_eq!(payload.title, "New User Registration");
            assert_eq!(payload.identifier, "USER456");
        }
        _ => panic!("Expected NewUser payload"),
    }
}

/// Test roundtrip serialization/deserialization with KYC event payload
#[test]
fn test_roundtrip_kyc_event_message() {
    let payload = KycEventPayload::new(
        "KYC Review".to_string(),
        "KYC123".to_string(),
        Some("completed".to_string()),
        Some("approved".to_string()),
    )
    .unwrap();

    let message = MessageBusMessage::create(
        "KYC_SERVICE".to_string(),
        Payload::Kyc(payload),
        "kyc.token".to_string(),
        None,
        Some(EventType::KycEvent),
        None,
        None,
    )
    .unwrap();

    // Serialize and deserialize
    let json = serde_json::to_string(&message).unwrap();
    let deserialized: MessageBusMessage = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.meta_data.event, Some(EventType::KycEvent));

    match deserialized.payload {
        Payload::Kyc(payload) => {
            assert_eq!(payload.title, "KYC Review");
            assert_eq!(payload.identifier, "KYC123");
            assert_eq!(payload.review_status, Some("completed".to_string()));
            assert_eq!(payload.review_result, Some("approved".to_string()));
        }
        _ => panic!("Expected Kyc payload"),
    }
}

/// Test deserialization of MessageBusMessage with PasswordReset event payload
#[test]
fn test_deserialize_password_reset_event_message() {
    let json = r#"{
        "meta_data": {
            "source": "AUTH_SERVICE",
            "created_at": "2022-02-01T00:00:00Z",
            "token": "auth.token",
            "idempotency_key": "auth-key-202",
            "event": "PASSWORD_RESET_REQUESTED"
        },
        "payload": {
            "to": "user@example.com",
            "subject": "Reset Your Password"
        }
    }"#;

    let message: MessageBusMessage = serde_json::from_str(json).unwrap();

    assert_eq!(message.meta_data.event, Some(EventType::PasswordResetRequested));

    match message.payload {
        Payload::PasswordReset(payload) => {
            assert_eq!(payload.to, "user@example.com");
            assert_eq!(payload.subject, "Reset Your Password");
        }
        _ => panic!("Expected PasswordReset payload"),
    }
}

/// Test roundtrip serialization/deserialization with VerificationRequested event payload
#[test]
fn test_roundtrip_verification_requested_event_message() {
    let payload = VerificationRequestedEventPayload::new(
        "user@example.com".to_string(),
        "Verify Your Email".to_string(),
    )
    .unwrap();

    let message = MessageBusMessage::create(
        "VERIFICATION_SERVICE".to_string(),
        Payload::VerificationRequested(payload),
        "verify.token".to_string(),
        None,
        Some(EventType::VerificationRequested),
        None,
        None,
    )
    .unwrap();

    // Serialize and deserialize
    let json = serde_json::to_string(&message).unwrap();
    let deserialized: MessageBusMessage = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.meta_data.event, Some(EventType::VerificationRequested));

    match deserialized.payload {
        Payload::VerificationRequested(payload) => {
            assert_eq!(payload.to, "user@example.com");
            assert_eq!(payload.subject, "Verify Your Email");
        }
        _ => panic!("Expected VerificationRequested payload"),
    }
}

/// Test deserialization of MessageBusMessage with Raw payload
#[test]
fn test_deserialize_raw_payload_message() {
    let json = r#"{
        "meta_data": {
            "source": "RAW_SERVICE",
            "created_at": "2022-04-01T00:00:00Z",
            "token": "raw.token",
            "idempotency_key": "raw-key-404",
            "instruction_type": "PAYMENT"
        },
        "payload": "arbitrary string data"
    }"#;

    let message: MessageBusMessage = serde_json::from_str(json).unwrap();

    assert_eq!(message.meta_data.instruction_type, Some(InstructionType::Payment));

    match message.payload {
        Payload::Raw(data) => {
            assert_eq!(data, "arbitrary string data");
        }
        _ => panic!("Expected Raw payload"),
    }
}

/// Test full roundtrip serialization/deserialization for Payment message
#[test]
fn test_payment_message_roundtrip() {
    let payload = PaymentPayload::new(
        "P888888888".to_string(),
        "EUR".to_string(),
        "999.99".to_string(),
        "BANK_TEST".to_string(),
        "REF888".to_string(),
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
        Some("203.0.113.45".to_string()),
    )
    .unwrap();

    // Serialize
    let serialized = serde_json::to_string(&message).unwrap();

    // Deserialize
    let deserialized: MessageBusMessage = serde_json::from_str(&serialized).unwrap();

    // Compare
    assert_eq!(message, deserialized);
}

/// Test deserialization with IPv6 address
#[test]
fn test_deserialize_message_with_ipv6() {
    let json = r#"{
        "meta_data": {
            "source": "IPV6_SERVICE",
            "created_at": "2022-05-01T00:00:00Z",
            "token": "ipv6.token",
            "idempotency_key": "ipv6-key-505",
            "instruction_type": "PAYMENT",
            "ip_address": "2001:0db8:85a3:0000:0000:8a2e:0370:7334"
        },
        "payload": {
            "external_reference": "P123",
            "currency": "USD",
            "value": "10.00",
            "source": "TEST",
            "reference": "REF123"
        }
    }"#;

    let message: MessageBusMessage = serde_json::from_str(json).unwrap();
    assert_eq!(
        message.meta_data.ip_address,
        Some("2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string())
    );
}

/// Test deserialization with special characters in payload
#[test]
fn test_deserialize_message_with_special_characters() {
    let json = r#"{
        "meta_data": {
            "source": "SPECIAL_SERVICE",
            "created_at": "2022-06-01T00:00:00Z",
            "token": "special.token",
            "idempotency_key": "special-key-606",
            "instruction_type": "CORRECTION"
        },
        "payload": {
            "reference": "REF-123/456",
            "value": "50.00",
            "message": "Corrected: \"quoted\" & <escaped>",
            "currency": "USD",
            "source": "BANK_XYZ"
        }
    }"#;

    let message: MessageBusMessage = serde_json::from_str(json).unwrap();

    match message.payload {
        Payload::Correction(payload) => {
            assert_eq!(payload.message, "Corrected: \"quoted\" & <escaped>");
        }
        _ => panic!("Expected Correction payload"),
    }
}

/// Test deserialization with unicode characters
#[test]
fn test_deserialize_message_with_unicode() {
    let json = r#"{
        "meta_data": {
            "source": "UNICODE_SERVICE",
            "created_at": "2022-07-01T00:00:00Z",
            "token": "unicode.token",
            "idempotency_key": "unicode-key-707",
            "instruction_type": "PAYMENT"
        },
        "payload": {
            "external_reference": "P123",
            "currency": "EUR",
            "value": "123.00",
            "source": "BANK",
            "reference": "REF123",
            "payer_name": "José María García 日本語 🚀"
        }
    }"#;

    let message: MessageBusMessage = serde_json::from_str(json).unwrap();

    match message.payload {
        Payload::Payment(payload) => {
            assert_eq!(
                payload.payer_name,
                Some("José María García 日本語 🚀".to_string())
            );
        }
        _ => panic!("Expected Payment payload"),
    }
}

/// Test deserialization failure with missing required metadata field
#[test]
fn test_deserialize_message_missing_required_metadata_field() {
    let json = r#"{
        "meta_data": {
            "source": "TEST_SERVICE",
            "created_at": "2022-08-01T00:00:00Z",
            "token": "test.token"
        },
        "payload": {
            "external_reference": "P123",
            "currency": "USD",
            "value": "10.00",
            "source": "TEST",
            "reference": "REF123"
        }
    }"#;

    // Should fail to deserialize because idempotency_key is missing
    let result: Result<MessageBusMessage, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

/// Test deserialization failure with malformed JSON
#[test]
fn test_deserialize_message_malformed_json() {
    let json = r#"{
        "meta_data": {
            "source": "TEST_SERVICE",
            "created_at": "2022-09-01T00:00:00Z",
            "token": "test.token",
            "idempotency_key": "key-808",
            "instruction_type": "PAYMENT"
        },
        "payload": {
            "external_reference": "P123",
            "currency": "USD",
            "value": "10.00"
            "source": "TEST",
    }"#;

    // Should fail to deserialize because of malformed JSON (missing comma)
    let result: Result<MessageBusMessage, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

/// Test deserialization with both instruction_type and event (should fail validation)
#[test]
#[should_panic(expected = "cannot specify both instruction_type and event")]
fn test_deserialize_message_with_both_instruction_and_event() {
    let json = r#"{
        "meta_data": {
            "source": "INVALID_SERVICE",
            "created_at": "2022-10-01T00:00:00Z",
            "token": "invalid.token",
            "idempotency_key": "invalid-key-909",
            "instruction_type": "PAYMENT",
            "event": "NEW_TRANSACTION"
        },
        "payload": {
            "external_reference": "P123",
            "currency": "USD",
            "value": "10.00",
            "source": "TEST",
            "reference": "REF123"
        }
    }"#;

    // This should deserialize JSON successfully but fail validation
    let message: MessageBusMessage = serde_json::from_str(json).unwrap();

    // Validation should fail
    message.validate().unwrap();
}

/// Test roundtrip with complex transaction message
#[test]
fn test_roundtrip_complex_transaction_message() {
    let payload = TransactionPayload::new(
        "EXT-2022-11-001".to_string(),
        "MOBILE_BANKING_APP_V2".to_string(),
        "INT-REF-99999".to_string(),
        "María José".to_string(),
        "García-Rodríguez".to_string(),
        TransactionType::Withdraw,
        "PROCESSING".to_string(),
        "EUR".to_string(),
        "GBP".to_string(),
        "1234.56".to_string(),
        "12.34".to_string(),
        None,
        Some("IBAN:GB29NWBK60161331926819".to_string()),
    )
    .unwrap();

    let message = MessageBusMessage::create(
        "COMPLEX_SERVICE".to_string(),
        Payload::Transaction(payload),
        "complex.token.with.lots.of.segments".to_string(),
        Some(InstructionType::Transaction),
        None,
        None,
        Some("192.168.100.200".to_string()),
    )
    .unwrap();

    // Serialize and deserialize
    let serialized = serde_json::to_string(&message).unwrap();
    let deserialized: MessageBusMessage = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.meta_data.source, "COMPLEX_SERVICE");
    assert_eq!(deserialized.meta_data.ip_address, Some("192.168.100.200".to_string()));

    match deserialized.payload {
        Payload::Transaction(payload) => {
            assert_eq!(payload.first_name, "María José");
            assert_eq!(payload.last_name, "García-Rodríguez");
            assert_eq!(payload.transaction_type, TransactionType::Withdraw);
            assert_eq!(payload.value, "1234.56");
            assert_eq!(payload.payee, Some("IBAN:GB29NWBK60161331926819".to_string()));
        }
        _ => panic!("Expected Transaction payload"),
    }
}

/// Test that all instruction payload types can roundtrip successfully
#[test]
fn test_all_instruction_payloads_roundtrip() {
    // Payment
    let payment = PaymentPayload::new(
        "P123".to_string(),
        "EUR".to_string(),
        "100.00".to_string(),
        "BANK".to_string(),
        "REF123".to_string(),
        None,
        None,
    )
    .unwrap();
    let msg = MessageBusMessage::create(
        "SRC".to_string(),
        Payload::Payment(payment),
        "token".to_string(),
        Some(InstructionType::Payment),
        None,
        None,
        None,
    )
    .unwrap();
    let json = serde_json::to_string(&msg).unwrap();
    let _: MessageBusMessage = serde_json::from_str(&json).unwrap();

    // StatusUpdate
    let status = StatusUpdatePayload::new("REF".to_string(), "COMPLETED".to_string(), None, None).unwrap();
    let msg = MessageBusMessage::create(
        "SRC".to_string(),
        Payload::StatusUpdate(status),
        "token".to_string(),
        Some(InstructionType::StatusUpdate),
        None,
        None,
        None,
    )
    .unwrap();
    let json = serde_json::to_string(&msg).unwrap();
    let _: MessageBusMessage = serde_json::from_str(&json).unwrap();

    // Correction
    let correction = CorrectionPayload::new(
        "REF".to_string(),
        "50.00".to_string(),
        "Msg".to_string(),
        "USD".to_string(),
        "BANK".to_string(),
    )
    .unwrap();
    let msg = MessageBusMessage::create(
        "SRC".to_string(),
        Payload::Correction(correction),
        "token".to_string(),
        Some(InstructionType::Correction),
        None,
        None,
        None,
    )
    .unwrap();
    let json = serde_json::to_string(&msg).unwrap();
    let _: MessageBusMessage = serde_json::from_str(&json).unwrap();

    // BankPaymentRequest
    let bank = BankPaymentRequestPayload::new(
        "REF".to_string(),
        "100.00".to_string(),
        "USD".to_string(),
        "PROF".to_string(),
        None,
    )
    .unwrap();
    let msg = MessageBusMessage::create(
        "SRC".to_string(),
        Payload::BankPaymentRequest(bank),
        "token".to_string(),
        Some(InstructionType::BankPaymentRequest),
        None,
        None,
        None,
    )
    .unwrap();
    let json = serde_json::to_string(&msg).unwrap();
    let _: MessageBusMessage = serde_json::from_str(&json).unwrap();
}

/// Test that all event payload types can roundtrip successfully
#[test]
fn test_all_event_payloads_roundtrip() {
    // NewTransaction
    let payload = NewTransactionEventPayload::new(
        "2021-01-01T00:00:00Z".to_string(),
        TransactionType::Deposit,
        "TXN123".to_string(),
        "SRC".to_string(),
    )
    .unwrap();
    let msg = MessageBusMessage::create(
        "SRC".to_string(),
        Payload::NewTransaction(payload),
        "token".to_string(),
        None,
        Some(EventType::NewTransaction),
        None,
        None,
    )
    .unwrap();
    let json = serde_json::to_string(&msg).unwrap();
    let _: MessageBusMessage = serde_json::from_str(&json).unwrap();

    // PaymentEvent
    let payload = PaymentEventPayload::new("PAY123".to_string(), "BANK".to_string(), None).unwrap();
    let msg = MessageBusMessage::create(
        "SRC".to_string(),
        Payload::PaymentEvent(payload),
        "token".to_string(),
        None,
        Some(EventType::Payment),
        None,
        None,
    )
    .unwrap();
    let json = serde_json::to_string(&msg).unwrap();
    let _: MessageBusMessage = serde_json::from_str(&json).unwrap();

    // Profile
    let payload = ProfileEventPayload::new("Title".to_string(), "ID123".to_string()).unwrap();
    let msg = MessageBusMessage::create(
        "SRC".to_string(),
        Payload::Profile(payload),
        "token".to_string(),
        None,
        Some(EventType::NewProfile),
        None,
        None,
    )
    .unwrap();
    let json = serde_json::to_string(&msg).unwrap();
    let _: MessageBusMessage = serde_json::from_str(&json).unwrap();

    // PasswordReset
    let payload =
        PasswordResetEventPayload::new("user@test.com".to_string(), "Reset".to_string()).unwrap();
    let msg = MessageBusMessage::create(
        "SRC".to_string(),
        Payload::PasswordReset(payload),
        "token".to_string(),
        None,
        Some(EventType::PasswordResetRequested),
        None,
        None,
    )
    .unwrap();
    let json = serde_json::to_string(&msg).unwrap();
    let _: MessageBusMessage = serde_json::from_str(&json).unwrap();
}
