use super::base::{validate_required_fields, EventType, InstructionType, ValidationError};
use super::event::*;
use super::instruction::*;
use chrono::{SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Metadata for message bus messages
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetaData {
    pub source: String,
    pub created_at: String,
    pub token: String,
    pub idempotency_key: String,
    pub instruction_type: Option<InstructionType>,
    pub event: Option<EventType>,
}

impl MetaData {
    pub fn new(
        source: String,
        created_at: String,
        token: String,
        idempotency_key: String,
        instruction_type: Option<InstructionType>,
        event: Option<EventType>,
    ) -> Result<Self, ValidationError> {
        let metadata = Self {
            source: source.clone(),
            created_at: created_at.clone(),
            token: token.clone(),
            idempotency_key: idempotency_key.clone(),
            instruction_type,
            event,
        };

        metadata.validate()?;
        Ok(metadata)
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        // Validate required base fields
        validate_required_fields(
            &[
                ("source", &self.source),
                ("created_at", &self.created_at),
                ("token", &self.token),
                ("idempotency_key", &self.idempotency_key),
            ],
            "MetaData",
        )?;

        // Ensure at least one of instruction_type or event is provided
        if self.instruction_type.is_none() && self.event.is_none() {
            return Err(ValidationError {
                class_name: "MetaData".to_string(),
                fields: vec!["either instruction_type or event must be provided".to_string()],
            });
        }

        // Ensure both instruction_type and event are not provided together
        if self.instruction_type.is_some() && self.event.is_some() {
            return Err(ValidationError {
                class_name: "MetaData".to_string(),
                fields: vec!["cannot specify both instruction_type and event".to_string()],
            });
        }

        Ok(())
    }
}

/// Enum containing all possible payload types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Payload {
    // Instruction payloads
    Payment(PaymentPayload),
    StatusUpdate(StatusUpdatePayload),
    Correction(CorrectionPayload),
    Transaction(TransactionPayload),

    // Event payloads
    NewTransaction(NewTransactionEventPayload),
    TransactionStatus(TransactionStatusEventPayload),
    PaymentEvent(PaymentEventPayload),
    Profile(ProfileEventPayload),
    Kyc(KycEventPayload),
    PasswordReset(PasswordResetEventPayload),
    VerificationRequested(VerificationRequestedEventPayload),
}

/// Complete message bus message structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageBusMessage {
    pub meta_data: MetaData,
    pub payload: Payload,
}

impl MessageBusMessage {
    pub fn new(meta_data: MetaData, payload: Payload) -> Result<Self, ValidationError> {
        let message = Self {
            meta_data: meta_data.clone(),
            payload: payload.clone(),
        };

        message.validate()?;
        Ok(message)
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        self.meta_data.validate()?;

        // Validate that the payload type matches the instruction_type or event
        if let Some(instruction_type) = &self.meta_data.instruction_type {
            match (instruction_type, &self.payload) {
                (InstructionType::Payment, Payload::Payment(_)) => Ok(()),
                (InstructionType::StatusUpdate, Payload::StatusUpdate(_)) => Ok(()),
                (InstructionType::Correction, Payload::Correction(_)) => Ok(()),
                (InstructionType::Transaction, Payload::Transaction(_)) => Ok(()),
                _ => Err(ValidationError {
                    class_name: "MessageBusMessage".to_string(),
                    fields: vec![format!(
                        "message type {}: {} requires matching payload type",
                        instruction_type, instruction_type
                    )],
                }),
            }
        } else if let Some(event) = &self.meta_data.event {
            match (event, &self.payload) {
                (EventType::NewTransaction, Payload::NewTransaction(_)) => Ok(()),
                (EventType::TransactionStatusUpdate, Payload::TransactionStatus(_)) => Ok(()),
                (EventType::NewBankPayment, Payload::PaymentEvent(_)) => Ok(()),
                (EventType::NewChainPayment, Payload::PaymentEvent(_)) => Ok(()),
                (EventType::NewProfile, Payload::Profile(_)) => Ok(()),
                (EventType::KycEvent, Payload::Kyc(_)) => Ok(()),
                (EventType::PasswordResetRequested, Payload::PasswordReset(_)) => Ok(()),
                (EventType::VerificationRequested, Payload::VerificationRequested(_)) => Ok(()),
                _ => Err(ValidationError {
                    class_name: "MessageBusMessage".to_string(),
                    fields: vec![format!(
                        "message type {}: {} requires matching payload type",
                        event, event
                    )],
                }),
            }
        } else {
            Err(ValidationError {
                class_name: "MessageBusMessage".to_string(),
                fields: vec!["either instruction_type or event must be provided".to_string()],
            })
        }?;

        Ok(())
    }

    /// Convenience function to create a complete MessageBusMessage
    pub fn create(
        source: String,
        payload: Payload,
        service_token: String,
        instruction_type: Option<InstructionType>,
        event: Option<EventType>,
        idempotency_key: Option<String>,
    ) -> Result<Self, ValidationError> {
        let idempotency_key = idempotency_key.unwrap_or_else(|| Uuid::new_v4().to_string());
        let created_at = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);

        // MetaData::new will handle all validations
        let meta_data = MetaData::new(
            source,
            created_at,
            service_token,
            idempotency_key,
            instruction_type,
            event,
        )?;

        MessageBusMessage::new(meta_data, payload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_valid() {
        let metadata = MetaData::new(
            "BANKING_SERVICE".to_string(),
            "2021-01-01T00:00:00Z".to_string(),
            "test.token.here".to_string(),
            "unique-key-123".to_string(),
            Some(InstructionType::Payment),
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
            super::super::base::TransactionType::Deposit,
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
        );

        assert!(message.is_err());
    }

    #[test]
    fn test_message_validates_payload_type() {
        let payload = StatusUpdatePayload::new(
            "REF123".to_string(),
            "PENDING".to_string(),
            "Test".to_string(),
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
        )
        .unwrap();

        let message = MessageBusMessage::new(metadata, Payload::StatusUpdate(payload));
        assert!(message.is_err());
    }
}
