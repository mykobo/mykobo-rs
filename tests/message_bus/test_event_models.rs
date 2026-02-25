use mykobo_rs::message_bus::models::event::*;
use mykobo_rs::message_bus::TransactionType;

#[test]
fn test_new_transaction_event_payload_valid() {
    let payload = NewTransactionEventPayload::new(
        "2021-01-01T00:00:00Z".to_string(),
        TransactionType::Deposit,
        "TXN123".to_string(),
        "BANKING_SERVICE".to_string(),
    );
    assert!(payload.is_ok());
}

#[test]
fn test_kyc_event_payload_completed_requires_result() {
    let payload = KycEventPayload::new(
        "Profile".to_string(),
        "12345".to_string(),
        Some("completed".to_string()),
        None, // Missing review_result
    );
    assert!(payload.is_err());
}

#[test]
fn test_kyc_event_payload_completed_with_result() {
    let payload = KycEventPayload::new(
        "Profile".to_string(),
        "12345".to_string(),
        Some("completed".to_string()),
        Some("approved".to_string()),
    );
    assert!(payload.is_ok());
}

#[test]
fn test_new_transaction_event_payload_from_string() {
    let json = r#"{
        "created_at": "2021-01-01T00:00:00Z",
        "kind": "DEPOSIT",
        "reference": "TXN123",
        "source": "BANKING_SERVICE"
    }"#;

    let payload: NewTransactionEventPayload = json.to_string().into();
    assert_eq!(payload.created_at, "2021-01-01T00:00:00Z");
    assert_eq!(payload.kind, TransactionType::Deposit);
    assert_eq!(payload.reference, "TXN123");
    assert_eq!(payload.source, "BANKING_SERVICE");
}

#[test]
fn test_transaction_status_event_payload_from_string() {
    let json = r#"{
        "reference": "TXN123",
        "status": "COMPLETED"
    }"#;

    let payload: TransactionStatusEventPayload = json.to_string().into();
    assert_eq!(payload.reference, "TXN123");
    assert_eq!(payload.status, "COMPLETED");
}

#[test]
fn test_payment_event_payload_from_string() {
    let json = r#"{
        "external_reference": "PAY123",
        "source": "BANK_XYZ",
        "reference": "REF123"
    }"#;

    let payload: PaymentEventPayload = json.to_string().into();
    assert_eq!(payload.external_reference, "PAY123");
    assert_eq!(payload.source, "BANK_XYZ");
    assert_eq!(payload.reference, Some("REF123".to_string()));
}

#[test]
fn test_profile_event_payload_from_string() {
    let json = r#"{
        "title": "New User",
        "identifier": "USER123"
    }"#;

    let payload: ProfileEventPayload = json.to_string().into();
    assert_eq!(payload.title, "New User");
    assert_eq!(payload.identifier, "USER123");
}

#[test]
fn test_new_user_event_payload_valid() {
    let payload = NewUserEventPayload::new("User Profile".to_string(), "USER456".to_string());
    assert!(payload.is_ok());
}

#[test]
fn test_new_user_event_payload_from_string() {
    let json = r#"{
        "title": "New User Registration",
        "identifier": "USER789"
    }"#;

    let payload: NewUserEventPayload = json.to_string().into();
    assert_eq!(payload.title, "New User Registration");
    assert_eq!(payload.identifier, "USER789");
}

#[test]
fn test_kyc_event_payload_from_string() {
    let json = r#"{
        "title": "KYC Review",
        "identifier": "KYC123",
        "review_status": "completed",
        "review_result": "approved"
    }"#;

    let payload: KycEventPayload = json.to_string().into();
    assert_eq!(payload.title, "KYC Review");
    assert_eq!(payload.identifier, "KYC123");
    assert_eq!(payload.review_status, Some("completed".to_string()));
    assert_eq!(payload.review_result, Some("approved".to_string()));
}

#[test]
fn test_password_reset_event_payload_from_string() {
    let json = r#"{
        "to": "user@example.com",
        "subject": "Reset Your Password"
    }"#;

    let payload: PasswordResetEventPayload = json.to_string().into();
    assert_eq!(payload.to, "user@example.com");
    assert_eq!(payload.subject, "Reset Your Password");
}

#[test]
fn test_verification_requested_event_payload_from_string() {
    let json = r#"{
        "to": "user@example.com",
        "subject": "Verify Your Email"
    }"#;

    let payload: VerificationRequestedEventPayload = json.to_string().into();
    assert_eq!(payload.to, "user@example.com");
    assert_eq!(payload.subject, "Verify Your Email");
}

// Serialization/Deserialization Round-trip Tests

#[test]
fn test_new_transaction_event_payload_serialization_roundtrip() {
    let original = NewTransactionEventPayload::new(
        "2021-01-01T00:00:00Z".to_string(),
        TransactionType::Deposit,
        "TXN123".to_string(),
        "BANKING_SERVICE".to_string(),
    )
    .unwrap();

    let serialized: String = original.clone().into();
    let deserialized: NewTransactionEventPayload = serialized.into();

    assert_eq!(original, deserialized);
}

#[test]
fn test_transaction_status_event_payload_serialization_roundtrip() {
    let original =
        TransactionStatusEventPayload::new("TXN123".to_string(), "COMPLETED".to_string(), None)
            .unwrap();

    let serialized: String = original.clone().into();
    let deserialized: TransactionStatusEventPayload = serialized.into();

    assert_eq!(original, deserialized);
}

#[test]
fn test_payment_event_payload_serialization_roundtrip() {
    let original = PaymentEventPayload::new(
        "PAY123".to_string(),
        "BANK_XYZ".to_string(),
        Some("REF123".to_string()),
    )
    .unwrap();

    let serialized: String = original.clone().into();
    let deserialized: PaymentEventPayload = serialized.into();

    assert_eq!(original, deserialized);
}

#[test]
fn test_payment_event_payload_serialization_without_optionals() {
    let payload =
        PaymentEventPayload::new("PAY456".to_string(), "BANK_ABC".to_string(), None).unwrap();

    let serialized = serde_json::to_string(&payload).unwrap();

    // Optional field should not appear in JSON (check for the field name with quotes and colon)
    assert!(!serialized.contains("\"reference\":"));

    let deserialized: PaymentEventPayload = serde_json::from_str(&serialized).unwrap();
    assert_eq!(payload, deserialized);
}

#[test]
fn test_profile_event_payload_serialization_roundtrip() {
    let original = ProfileEventPayload::new("New User".to_string(), "USER123".to_string()).unwrap();

    let serialized: String = original.clone().into();
    let deserialized: ProfileEventPayload = serialized.into();

    assert_eq!(original, deserialized);
}

#[test]
fn test_new_user_event_payload_serialization_roundtrip() {
    let original =
        NewUserEventPayload::new("New User Registration".to_string(), "USER789".to_string())
            .unwrap();

    let serialized: String = original.clone().into();
    let deserialized: NewUserEventPayload = serialized.into();

    assert_eq!(original, deserialized);
}

#[test]
fn test_kyc_event_payload_serialization_roundtrip() {
    let original = KycEventPayload::new(
        "KYC Review".to_string(),
        "KYC123".to_string(),
        Some("completed".to_string()),
        Some("approved".to_string()),
    )
    .unwrap();

    let serialized: String = original.clone().into();
    let deserialized: KycEventPayload = serialized.into();

    assert_eq!(original, deserialized);
}

#[test]
fn test_kyc_event_payload_serialization_without_optionals() {
    let payload =
        KycEventPayload::new("KYC Review".to_string(), "KYC456".to_string(), None, None).unwrap();

    let serialized = serde_json::to_string(&payload).unwrap();

    // Optional fields should not appear in JSON
    assert!(!serialized.contains("review_status"));
    assert!(!serialized.contains("review_result"));

    let deserialized: KycEventPayload = serde_json::from_str(&serialized).unwrap();
    assert_eq!(payload, deserialized);
}

#[test]
fn test_password_reset_event_payload_serialization_roundtrip() {
    let original = PasswordResetEventPayload::new(
        "user@example.com".to_string(),
        "Reset Your Password".to_string(),
    )
    .unwrap();

    let serialized: String = original.clone().into();
    let deserialized: PasswordResetEventPayload = serialized.into();

    assert_eq!(original, deserialized);
}

#[test]
fn test_verification_requested_event_payload_serialization_roundtrip() {
    let original = VerificationRequestedEventPayload::new(
        "user@example.com".to_string(),
        "Verify Your Email".to_string(),
    )
    .unwrap();

    let serialized: String = original.clone().into();
    let deserialized: VerificationRequestedEventPayload = serialized.into();

    assert_eq!(original, deserialized);
}

#[test]
fn test_event_payload_with_special_characters() {
    let payload = ProfileEventPayload::new(
        "User: \"John\" <admin>".to_string(),
        "USER-123/456".to_string(),
    )
    .unwrap();

    let serialized: String = payload.clone().into();
    let deserialized: ProfileEventPayload = serialized.into();

    assert_eq!(payload, deserialized);
    assert_eq!(deserialized.title, "User: \"John\" <admin>");
}

#[test]
fn test_event_payload_with_unicode() {
    let payload = NewUserEventPayload::new(
        "Usuario: José García 日本語 🎉".to_string(),
        "USER-中文-123".to_string(),
    )
    .unwrap();

    let serialized: String = payload.clone().into();
    let deserialized: NewUserEventPayload = serialized.into();

    assert_eq!(payload, deserialized);
    assert_eq!(deserialized.title, "Usuario: José García 日本語 🎉");
    assert_eq!(deserialized.identifier, "USER-中文-123");
}

#[test]
fn test_email_payload_with_special_characters() {
    let payload = PasswordResetEventPayload::new(
        "user+test@example.com".to_string(),
        "Reset: \"Your\" Password & Account".to_string(),
    )
    .unwrap();

    let serialized: String = payload.clone().into();
    let deserialized: PasswordResetEventPayload = serialized.into();

    assert_eq!(payload, deserialized);
    assert_eq!(deserialized.to, "user+test@example.com");
    assert_eq!(deserialized.subject, "Reset: \"Your\" Password & Account");
}

#[test]
fn test_bank_payment_event_payload_valid() {
    let payload = BankPaymentEventPayload::new(
        "TX12345".to_string(),
        "COMPLETED".to_string(),
        "REF789".to_string(),
        Some("Payment processed successfully".to_string()),
    );
    assert!(payload.is_ok());
}

#[test]
fn test_bank_payment_event_payload_valid_without_message() {
    let payload = BankPaymentEventPayload::new(
        "TX12345".to_string(),
        "PENDING".to_string(),
        "REF789".to_string(),
        None,
    );
    assert!(payload.is_ok());
}

#[test]
fn test_bank_payment_event_payload_invalid_empty_transaction_id() {
    let payload = BankPaymentEventPayload::new(
        "".to_string(),
        "COMPLETED".to_string(),
        "REF789".to_string(),
        None,
    );
    assert!(payload.is_err());
}

#[test]
fn test_bank_payment_event_payload_from_string() {
    let json = r#"{
        "transaction_id": "TX99999",
        "status": "FAILED",
        "reference": "REF001",
        "message": "Insufficient funds"
    }"#;

    let payload: BankPaymentEventPayload = json.to_string().into();
    assert_eq!(payload.transaction_id, "TX99999");
    assert_eq!(payload.status, "FAILED");
    assert_eq!(payload.reference, "REF001");
    assert_eq!(payload.message, Some("Insufficient funds".to_string()));
}

#[test]
fn test_bank_payment_event_payload_serialization_roundtrip() {
    let original = BankPaymentEventPayload::new(
        "TX54321".to_string(),
        "PROCESSING".to_string(),
        "REF456".to_string(),
        Some("Transaction in progress".to_string()),
    )
    .unwrap();

    let serialized: String = original.clone().into();
    let deserialized: BankPaymentEventPayload = serialized.into();

    assert_eq!(original, deserialized);
}

#[test]
fn test_bank_payment_event_payload_serialization_without_optionals() {
    let payload = BankPaymentEventPayload::new(
        "TX11111".to_string(),
        "SUCCESS".to_string(),
        "REF222".to_string(),
        None,
    )
    .unwrap();

    let serialized = serde_json::to_string(&payload).unwrap();

    // Verify serialization works
    assert!(serialized.contains("\"transaction_id\":\"TX11111\""));
    assert!(serialized.contains("\"status\":\"SUCCESS\""));
    assert!(serialized.contains("\"reference\":\"REF222\""));

    let deserialized: BankPaymentEventPayload = serde_json::from_str(&serialized).unwrap();
    assert_eq!(payload, deserialized);
    assert_eq!(deserialized.message, None);
}
