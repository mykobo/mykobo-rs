use super::base::{validate_required_fields, EventType, InstructionType, ValidationError};
use super::event::*;
use super::instruction::*;
use chrono::{SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
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
    pub ip_address: Option<String>,
}

impl MetaData {
    pub fn new(
        source: String,
        created_at: String,
        token: String,
        idempotency_key: String,
        instruction_type: Option<InstructionType>,
        event: Option<EventType>,
        ip_address: Option<String>,
    ) -> Result<Self, ValidationError> {
        let metadata = Self {
            source: source.clone(),
            created_at: created_at.clone(),
            token: token.clone(),
            idempotency_key: idempotency_key.clone(),
            instruction_type,
            event,
            ip_address,
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
///
/// Each variant implements `From<String>` for easy conversion from JSON strings:
///
/// # Examples
///
/// ```
/// use mykobo_rs::message_bus::models::instruction::PaymentPayload;
///
/// let json = r#"{
///     "external_reference": "P123",
///     "currency": "EUR",
///     "value": "100.00",
///     "source": "BANK",
///     "direction": "INBOUND",
///     "reference": "REF123"
/// }"#;
///
/// let payload: PaymentPayload = json.to_string().into();
/// assert_eq!(payload.currency, "EUR");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Payload {
    // Instruction payloads
    Payment(PaymentPayload),
    StatusUpdate(StatusUpdatePayload),
    Correction(CorrectionPayload),
    Transaction(TransactionPayload),
    BankPaymentRequest(BankPaymentRequestPayload),
    ChainPayment(ChainPaymentPayload),
    UpdateProfile(UpdateProfilePayload),
    Mint(MintPayload),
    Burn(BurnPayload),

    // Event payloads
    NewTransaction(NewTransactionEventPayload),
    TransactionStatus(TransactionStatusEventPayload),
    PaymentEvent(PaymentEventPayload),
    BankPayment(BankPaymentEventPayload),
    Profile(ProfileEventPayload),
    NewUser(NewUserEventPayload),
    Kyc(KycEventPayload),
    PasswordReset(PasswordResetEventPayload),
    VerificationRequested(VerificationRequestedEventPayload),

    // Generic payload
    Raw(String),
}

impl Display for Payload {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Complete message bus message structure
#[derive(Debug, Clone, Serialize, PartialEq)]
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

        // Raw payloads skip type validation
        if matches!(&self.payload, Payload::Raw(_)) {
            return Ok(());
        }

        // Validate that the payload type matches the instruction_type or event
        if let Some(instruction_type) = &self.meta_data.instruction_type {
            match (instruction_type, &self.payload) {
                (InstructionType::Payment, Payload::Payment(_)) => Ok(()),
                (InstructionType::StatusUpdate, Payload::StatusUpdate(_)) => Ok(()),
                (InstructionType::Correction, Payload::Correction(_)) => Ok(()),
                (InstructionType::Transaction, Payload::Transaction(_)) => Ok(()),
                (InstructionType::BankPaymentRequest, Payload::BankPaymentRequest(_)) => Ok(()),
                (InstructionType::ChainPayment, Payload::ChainPayment(_)) => Ok(()),
                (InstructionType::UpdateProfile, Payload::UpdateProfile(_)) => Ok(()),
                (InstructionType::Mint, Payload::Mint(_)) => Ok(()),
                (InstructionType::Burn, Payload::Burn(_)) => Ok(()),
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
                (EventType::Payment, Payload::PaymentEvent(_)) => Ok(()),
                (EventType::BankPayment, Payload::BankPayment(_)) => Ok(()),
                (EventType::NewProfile, Payload::Profile(_)) => Ok(()),
                (EventType::NewUser, Payload::NewUser(_)) => Ok(()),
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
        ip_address: Option<String>,
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
            ip_address,
        )?;

        MessageBusMessage::new(meta_data, payload)
    }
}

impl From<MessageBusMessage> for String {
    fn from(val: MessageBusMessage) -> Self {
        serde_json::to_string(&val).unwrap()
    }
}

// Custom deserializer for MessageBusMessage that uses metadata to determine payload type
impl<'de> Deserialize<'de> for MessageBusMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        // Deserialize into a generic JSON Value first
        let mut value = serde_json::Value::deserialize(deserializer)?;

        // Extract metadata to determine payload type
        let meta_data: MetaData = serde_json::from_value(
            value
                .get("meta_data")
                .ok_or_else(|| D::Error::missing_field("meta_data"))?
                .clone(),
        )
        .map_err(D::Error::custom)?;

        // Get the payload JSON value
        let payload_value = value
            .get_mut("payload")
            .ok_or_else(|| D::Error::missing_field("payload"))?
            .take();

        // Deserialize payload based on instruction_type or event
        let payload: Payload = if let Some(instruction_type) = &meta_data.instruction_type {
            // First check if it's a raw string payload
            if let Ok(raw_string) = serde_json::from_value::<String>(payload_value.clone()) {
                Payload::Raw(raw_string)
            } else {
                // Otherwise deserialize according to instruction type
                match instruction_type {
                    InstructionType::Payment => Payload::Payment(
                        serde_json::from_value(payload_value).map_err(D::Error::custom)?,
                    ),
                    InstructionType::StatusUpdate => Payload::StatusUpdate(
                        serde_json::from_value(payload_value).map_err(D::Error::custom)?,
                    ),
                    InstructionType::Correction => Payload::Correction(
                        serde_json::from_value(payload_value).map_err(D::Error::custom)?,
                    ),
                    InstructionType::Transaction => Payload::Transaction(
                        serde_json::from_value(payload_value).map_err(D::Error::custom)?,
                    ),
                    InstructionType::BankPaymentRequest => Payload::BankPaymentRequest(
                        serde_json::from_value(payload_value).map_err(D::Error::custom)?,
                    ),
                    InstructionType::ChainPayment => Payload::ChainPayment(
                        serde_json::from_value(payload_value).map_err(D::Error::custom)?,
                    ),
                    InstructionType::UpdateProfile => Payload::UpdateProfile(
                        serde_json::from_value(payload_value).map_err(D::Error::custom)?,
                    ),
                    InstructionType::Mint => Payload::Mint(
                        serde_json::from_value(payload_value).map_err(D::Error::custom)?,
                    ),
                    InstructionType::Burn => Payload::Burn(
                        serde_json::from_value(payload_value).map_err(D::Error::custom)?,
                    ),
                }
            }
        } else if let Some(event) = &meta_data.event {
            // First check if it's a raw string payload
            if let Ok(raw_string) = serde_json::from_value::<String>(payload_value.clone()) {
                Payload::Raw(raw_string)
            } else {
                // Otherwise deserialize according to event type
                match event {
                    EventType::NewTransaction => Payload::NewTransaction(
                        serde_json::from_value(payload_value).map_err(D::Error::custom)?,
                    ),
                    EventType::TransactionStatusUpdate => Payload::TransactionStatus(
                        serde_json::from_value(payload_value).map_err(D::Error::custom)?,
                    ),
                    EventType::Payment => Payload::PaymentEvent(
                        serde_json::from_value(payload_value).map_err(D::Error::custom)?,
                    ),
                    EventType::BankPayment => Payload::BankPayment(
                        serde_json::from_value(payload_value).map_err(D::Error::custom)?,
                    ),
                    EventType::NewProfile => Payload::Profile(
                        serde_json::from_value(payload_value).map_err(D::Error::custom)?,
                    ),
                    EventType::NewUser => Payload::NewUser(
                        serde_json::from_value(payload_value).map_err(D::Error::custom)?,
                    ),
                    EventType::KycEvent => Payload::Kyc(
                        serde_json::from_value(payload_value).map_err(D::Error::custom)?,
                    ),
                    EventType::PasswordResetRequested => Payload::PasswordReset(
                        serde_json::from_value(payload_value).map_err(D::Error::custom)?,
                    ),
                    EventType::VerificationRequested => Payload::VerificationRequested(
                        serde_json::from_value(payload_value).map_err(D::Error::custom)?,
                    ),
                }
            }
        } else {
            // Try to deserialize as Raw payload if no type hint
            if let Ok(raw_string) = serde_json::from_value::<String>(payload_value.clone()) {
                Payload::Raw(raw_string)
            } else {
                // Fall back to untagged deserialization
                serde_json::from_value(payload_value).map_err(D::Error::custom)?
            }
        };

        Ok(MessageBusMessage { meta_data, payload })
    }
}
