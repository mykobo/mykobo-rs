use mykobo_rs::message_bus::models::base::{validate_required_fields, PaymentDirection};
use mykobo_rs::message_bus::{EventType, InstructionType, TransactionType};

#[test]
fn test_instruction_type_display() {
    assert_eq!(InstructionType::Payment.to_string(), "PAYMENT");
    assert_eq!(InstructionType::StatusUpdate.to_string(), "STATUS_UPDATE");
}

#[test]
fn test_transaction_type_display() {
    assert_eq!(TransactionType::Deposit.to_string(), "DEPOSIT");
    assert_eq!(TransactionType::Withdraw.to_string(), "WITHDRAW");
}

#[test]
fn test_payment_direction_display() {
    assert_eq!(PaymentDirection::Inbound.to_string(), "INBOUND");
    assert_eq!(PaymentDirection::Outbound.to_string(), "OUTBOUND");
    assert_eq!(PaymentDirection::Both.to_string(), "BOTH");
}

#[test]
fn test_payment_direction_default() {
    let direction: PaymentDirection = Default::default();
    assert_eq!(direction, PaymentDirection::Inbound);
}

#[test]
fn test_payment_direction_from_string_inbound() {
    let direction: PaymentDirection = "\"INBOUND\"".to_string().into();
    assert_eq!(direction, PaymentDirection::Inbound);
}

#[test]
fn test_payment_direction_from_string_outbound() {
    let direction: PaymentDirection = "\"OUTBOUND\"".to_string().into();
    assert_eq!(direction, PaymentDirection::Outbound);
}

#[test]
fn test_payment_direction_from_string_both() {
    let direction: PaymentDirection = "\"BOTH\"".to_string().into();
    assert_eq!(direction, PaymentDirection::Both);
}

#[test]
fn test_payment_direction_from_string_lowercase() {
    let direction: PaymentDirection = "\"inbound\"".to_string().into();
    assert_eq!(direction, PaymentDirection::Inbound);

    let direction: PaymentDirection = "outbound".to_string().into();
    assert_eq!(direction, PaymentDirection::Outbound);
}

#[test]
fn test_payment_direction_from_string_mixed_case() {
    let direction: PaymentDirection = "Inbound".to_string().into();
    assert_eq!(direction, PaymentDirection::Inbound);

    let direction: PaymentDirection = "\"OutBound\"".to_string().into();
    assert_eq!(direction, PaymentDirection::Outbound);

    let direction: PaymentDirection = "bOtH".to_string().into();
    assert_eq!(direction, PaymentDirection::Both);
}

#[test]
#[should_panic(expected = "Failed to deserialize PaymentDirection from String")]
fn test_payment_direction_from_string_invalid() {
    let _direction: PaymentDirection = "\"INVALID\"".to_string().into();
}

#[test]
fn test_event_type_display() {
    assert_eq!(EventType::NewTransaction.to_string(), "NEW_TRANSACTION");
    assert_eq!(EventType::KycEvent.to_string(), "KYC_EVENT");
}

#[test]
fn test_validate_required_fields_success() {
    let fields = vec![("source", "BANKING_SERVICE"), ("token", "test.token")];
    assert!(validate_required_fields(&fields, "TestClass").is_ok());
}

#[test]
fn test_validate_required_fields_empty() {
    let fields = vec![("source", ""), ("token", "test.token")];
    let result = validate_required_fields(&fields, "TestClass");
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.class_name, "TestClass");
        assert_eq!(e.fields, vec!["source"]);
    }
}

#[test]
fn test_validate_required_fields_whitespace() {
    let fields = vec![("source", "  "), ("token", "test.token")];
    let result = validate_required_fields(&fields, "TestClass");
    assert!(result.is_err());
}
