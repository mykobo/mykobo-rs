use super::base::{validate_required_fields, TransactionType, ValidationError};
use serde::{Deserialize, Serialize};

/// Payload for a new transaction event mainly for notification purposes
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

impl From<NewTransactionEventPayload> for String {
    fn from(val: NewTransactionEventPayload) -> Self {
        serde_json::to_string(&val)
            .expect("Failed to serialize NewTransactionEventPayload to String")
    }
}

/// Payload for transaction status update event for notification purposes, this can go to the notification server
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde_with::skip_serializing_none]
pub struct TransactionStatusEventPayload {
    pub external_reference: Option<String>,
    pub reference: String,
    pub status: String,
}

impl TransactionStatusEventPayload {
    pub fn new(
        reference: String,
        status: String,
        external_reference: Option<String>,
    ) -> Result<Self, ValidationError> {
        let payload = Self {
            external_reference: external_reference.clone(),
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

impl From<TransactionStatusEventPayload> for String {
    fn from(val: TransactionStatusEventPayload) -> Self {
        serde_json::to_string(&val)
            .expect("Failed to serialize TransactionStatusEventPayload to String")
    }
}

/// Payload for notifying the business server of a bank payment event. This is generally used to let the
/// business server know to create a chain payment for the corresponding bank payment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde_with::skip_serializing_none]
pub struct BankPaymentEventPayload {
    pub transaction_id: String,
    pub status: String,
    pub reference: String,
    pub message: Option<String>,
}

impl BankPaymentEventPayload {
    pub fn new(
        transaction_id: String,
        status: String,
        reference: String,
        message: Option<String>,
    ) -> Result<Self, ValidationError> {
        let payload = Self {
            transaction_id: transaction_id.clone(),
            status: status.clone(),
            reference: reference.clone(),
            message: message.clone(),
        };

        payload.validate()?;
        Ok(payload)
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        validate_required_fields(
            &[
                ("transaction_id", &self.transaction_id),
                ("status", &self.status),
                ("reference", &self.reference),
            ],
            "BankPaymentEventPayload",
        )
    }
}

impl From<String> for BankPaymentEventPayload {
    fn from(value: String) -> Self {
        serde_json::from_str(&value)
            .expect("Failed to deserialize BankPaymentEventPayload from String")
    }
}

impl From<BankPaymentEventPayload> for String {
    fn from(val: BankPaymentEventPayload) -> Self {
        serde_json::to_string(&val).expect("Failed to serialize BankPaymentEventPayload to String")
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

impl From<PaymentEventPayload> for String {
    fn from(val: PaymentEventPayload) -> Self {
        serde_json::to_string(&val).expect("Failed to serialize PaymentEventPayload to String")
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

impl From<ProfileEventPayload> for String {
    fn from(val: ProfileEventPayload) -> Self {
        serde_json::to_string(&val).expect("Failed to serialize ProfileEventPayload to String")
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

impl From<NewUserEventPayload> for String {
    fn from(val: NewUserEventPayload) -> Self {
        serde_json::to_string(&val).expect("Failed to serialize NewUserEventPayload to String")
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

impl From<KycEventPayload> for String {
    fn from(val: KycEventPayload) -> Self {
        serde_json::to_string(&val).expect("Failed to serialize KycEventPayload to String")
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

impl From<PasswordResetEventPayload> for String {
    fn from(val: PasswordResetEventPayload) -> Self {
        serde_json::to_string(&val)
            .expect("Failed to serialize PasswordResetEventPayload to String")
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

impl From<VerificationRequestedEventPayload> for String {
    fn from(val: VerificationRequestedEventPayload) -> Self {
        serde_json::to_string(&val)
            .expect("Failed to serialize VerificationRequestedEventPayload to String")
    }
}
