use serde::{Deserialize, Serialize};
use std::fmt;

/// Enum for message instruction types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InstructionType {
    Payment, // ledger payment instruction
    StatusUpdate,
    Correction,
    Transaction,
    BankPaymentRequest, // banking gateway payment request instruction
    ChainPayment,       // this is for anchors that require an update from the chain
    UpdateProfile,      // profile update instruction
    Mint,               // mint instruction - to convert FIAT to Crypto asset
    Burn,               // burn instruction - to convert Crypto asset to FIAT
}

impl fmt::Display for InstructionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_value(self)
                .ok()
                .and_then(|v| v.as_str().map(String::from))
                .unwrap_or_default()
        )
    }
}

/// Enum for transaction types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
    Deposit,
    Withdraw,
    Transfer,
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_value(self)
                .ok()
                .and_then(|v| v.as_str().map(String::from))
                .unwrap_or_default()
        )
    }
}

/// Enum for payment direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[derive(Default)]
pub enum PaymentDirection {
    #[default]
    Inbound,
    Outbound,
    Both,
}

impl fmt::Display for PaymentDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_value(self)
                .ok()
                .and_then(|v| v.as_str().map(String::from))
                .unwrap_or_default()
        )
    }
}

impl From<String> for PaymentDirection {
    fn from(value: String) -> Self {
        let normalized = format!("\"{}\"", value.trim_matches('"').to_uppercase());
        serde_json::from_str(&normalized)
            .expect("Failed to deserialize PaymentDirection from String")
    }
}

/// Enum for event types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventType {
    NewTransaction,
    TransactionStatusUpdate,
    Payment,
    BankPayment,
    NewProfile,
    NewUser,
    VerificationRequested,
    PasswordResetRequested,
    KycEvent,
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_value(self)
                .ok()
                .and_then(|v| v.as_str().map(String::from))
                .unwrap_or_default()
        )
    }
}

/// Validation error for required fields
#[derive(Debug, Clone, thiserror::Error)]
pub struct ValidationError {
    pub class_name: String,
    pub fields: Vec<String>,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} missing required fields: {}",
            self.class_name,
            self.fields.join(", ")
        )
    }
}

/// Validate that required string fields are not empty or whitespace only
pub fn validate_required_fields(
    fields: &[(&str, &str)],
    class_name: &str,
) -> Result<(), ValidationError> {
    let missing_fields: Vec<String> = fields
        .iter()
        .filter(|(_, value)| value.trim().is_empty())
        .map(|(name, _)| name.to_string())
        .collect();

    if !missing_fields.is_empty() {
        return Err(ValidationError {
            class_name: class_name.to_string(),
            fields: missing_fields,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
