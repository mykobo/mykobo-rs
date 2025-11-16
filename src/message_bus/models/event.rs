use super::base::{validate_required_fields, TransactionType, ValidationError};
use serde::{Deserialize, Serialize};

/// Payload for new transaction event
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewTransactionEventPayload {
    pub created_at: String,
    pub kind: TransactionType,
    pub reference: String,
    pub source: String,
}

impl NewTransactionEventPayload {
    pub fn new(
        created_at: String,
        kind: TransactionType,
        reference: String,
        source: String,
    ) -> Result<Self, ValidationError> {
        let payload = Self {
            created_at: created_at.clone(),
            kind,
            reference: reference.clone(),
            source: source.clone(),
        };

        payload.validate()?;
        Ok(payload)
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        validate_required_fields(
            &[
                ("created_at", &self.created_at),
                ("reference", &self.reference),
                ("source", &self.source),
            ],
            "NewTransactionEventPayload",
        )
    }
}

impl From<String> for NewTransactionEventPayload {
    fn from(value: String) -> Self {
        serde_json::from_str(&value)
            .expect("Failed to deserialize NewTransactionEventPayload from String")
    }
}

/// Payload for transaction status update event
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionStatusEventPayload {
    pub reference: String,
    pub status: String,
}

impl TransactionStatusEventPayload {
    pub fn new(reference: String, status: String) -> Result<Self, ValidationError> {
        let payload = Self {
            reference: reference.clone(),
            status: status.clone(),
        };

        payload.validate()?;
        Ok(payload)
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        validate_required_fields(
            &[("reference", &self.reference), ("status", &self.status)],
            "TransactionStatusEventPayload",
        )
    }
}

impl From<String> for TransactionStatusEventPayload {
    fn from(value: String) -> Self {
        serde_json::from_str(&value)
            .expect("Failed to deserialize TransactionStatusEventPayload from String")
    }
}

/// Payload for bank payment event
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentEventPayload {
    pub external_reference: String,
    pub reference: Option<String>,
    pub source: String,
}

impl PaymentEventPayload {
    pub fn new(
        external_reference: String,
        source: String,
        reference: Option<String>,
    ) -> Result<Self, ValidationError> {
        let payload = Self {
            external_reference: external_reference.clone(),
            reference,
            source: source.clone(),
        };

        payload.validate()?;
        Ok(payload)
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        validate_required_fields(
            &[
                ("external_reference", &self.external_reference),
                ("source", &self.source),
            ],
            "PaymentEventPayload",
        )
    }
}

impl From<String> for PaymentEventPayload {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).expect("Failed to deserialize PaymentEventPayload from String")
    }
}

/// Payload for new profile event
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProfileEventPayload {
    pub title: String,
    pub identifier: String,
}

impl ProfileEventPayload {
    pub fn new(title: String, identifier: String) -> Result<Self, ValidationError> {
        let payload = Self {
            title: title.clone(),
            identifier: identifier.clone(),
        };

        payload.validate()?;
        Ok(payload)
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        validate_required_fields(
            &[("title", &self.title), ("identifier", &self.identifier)],
            "ProfileEventPayload",
        )
    }
}

impl From<String> for ProfileEventPayload {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).expect("Failed to deserialize ProfileEventPayload from String")
    }
}

/// Payload for new user event
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewUserEventPayload {
    pub title: String,
    pub identifier: String,
}

impl NewUserEventPayload {
    pub fn new(title: String, identifier: String) -> Result<Self, ValidationError> {
        let payload = Self {
            title: title.clone(),
            identifier: identifier.clone(),
        };

        payload.validate()?;
        Ok(payload)
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        validate_required_fields(
            &[("title", &self.title), ("identifier", &self.identifier)],
            "NewUserEventPayload",
        )
    }
}

impl From<String> for NewUserEventPayload {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).expect("Failed to deserialize NewUserEventPayload from String")
    }
}

/// Payload for KYC event
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KycEventPayload {
    pub title: String,
    pub identifier: String,
    pub review_status: Option<String>,
    pub review_result: Option<String>,
}

impl KycEventPayload {
    pub fn new(
        title: String,
        identifier: String,
        review_status: Option<String>,
        review_result: Option<String>,
    ) -> Result<Self, ValidationError> {
        let payload = Self {
            title: title.clone(),
            identifier: identifier.clone(),
            review_status: review_status.clone(),
            review_result: review_result.clone(),
        };

        payload.validate()?;
        Ok(payload)
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        validate_required_fields(
            &[("title", &self.title), ("identifier", &self.identifier)],
            "KycEventPayload",
        )?;

        // Additional validation: if review_status is "completed", review_result is required
        if let Some(ref status) = self.review_status {
            if status.to_lowercase() == "completed" && self.review_result.is_none() {
                return Err(ValidationError {
                    class_name: "KycEventPayload".to_string(),
                    fields: vec![
                        "review_result must be provided if review_status is completed".to_string(),
                    ],
                });
            }
        }

        Ok(())
    }
}

impl From<String> for KycEventPayload {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).expect("Failed to deserialize KycEventPayload from String")
    }
}

/// Payload for password reset event
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PasswordResetEventPayload {
    pub to: String,
    pub subject: String,
}

impl PasswordResetEventPayload {
    pub fn new(to: String, subject: String) -> Result<Self, ValidationError> {
        let payload = Self {
            to: to.clone(),
            subject: subject.clone(),
        };

        payload.validate()?;
        Ok(payload)
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        validate_required_fields(
            &[("to", &self.to), ("subject", &self.subject)],
            "PasswordResetEventPayload",
        )
    }
}

impl From<String> for PasswordResetEventPayload {
    fn from(value: String) -> Self {
        serde_json::from_str(&value)
            .expect("Failed to deserialize PasswordResetEventPayload from String")
    }
}

/// Payload for verification requested event
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerificationRequestedEventPayload {
    pub to: String,
    pub subject: String,
}

impl VerificationRequestedEventPayload {
    pub fn new(to: String, subject: String) -> Result<Self, ValidationError> {
        let payload = Self {
            to: to.clone(),
            subject: subject.clone(),
        };

        payload.validate()?;
        Ok(payload)
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        validate_required_fields(
            &[("to", &self.to), ("subject", &self.subject)],
            "VerificationRequestedEventPayload",
        )
    }
}

impl From<String> for VerificationRequestedEventPayload {
    fn from(value: String) -> Self {
        serde_json::from_str(&value)
            .expect("Failed to deserialize VerificationRequestedEventPayload from String")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
