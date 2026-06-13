use super::base::{validate_required_fields, PaymentDirection, TransactionType, ValidationError};
use serde::{Deserialize, Serialize};

/// Payload for payment instructions
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentPayload {
    pub external_reference: String,
    pub payer_name: Option<String>,
    pub currency: String,
    pub value: String,
    pub source: String,
    pub direction: PaymentDirection,
    pub reference: String,
    pub bank_account_number: Option<String>,
}

impl PaymentPayload {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        external_reference: String,
        currency: String,
        value: String,
        source: String,
        direction: PaymentDirection,
        reference: String,
        payer_name: Option<String>,
        bank_account_number: Option<String>,
    ) -> Result<Self, ValidationError> {
        let payload = Self {
            external_reference: external_reference.clone(),
            payer_name,
            currency: currency.clone(),
            value: value.clone(),
            source: source.clone(),
            direction,
            reference: reference.clone(),
            bank_account_number,
        };

        payload.validate()?;
        Ok(payload)
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        validate_required_fields(
            &[
                ("external_reference", &self.external_reference),
                ("currency", &self.currency),
                ("value", &self.value),
                ("source", &self.source),
                ("reference", &self.reference),
            ],
            "PaymentPayload",
        )
    }
}

impl From<String> for PaymentPayload {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).expect("Failed to deserialize PaymentPayload from String")
    }
}

impl From<PaymentPayload> for String {
    fn from(val: PaymentPayload) -> Self {
        serde_json::to_string(&val).expect("Failed to serialize PaymentPayload to String")
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChainPaymentPayload {
    pub chain: String,
    pub hash: String,
    pub reference: String,
    pub status: String,
    pub transaction_id: Option<String>,
}

impl ChainPaymentPayload {
    pub fn new(
        chain: String,
        hash: String,
        reference: String,
        status: String,
        transaction_id: Option<String>,
    ) -> Result<Self, ValidationError> {
        let payload = Self {
            chain,
            hash,
            reference,
            status,
            transaction_id,
        };

        payload.validate()?;
        Ok(payload)
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        validate_required_fields(
            &[
                ("chain", &self.chain),
                ("hash", &self.hash),
                ("reference", &self.reference),
                ("status", &self.status),
            ],
            "ChainPaymentPayload",
        )
    }
}
impl From<String> for ChainPaymentPayload {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).expect("Failed to deserialize ChainPaymentPayload from String")
    }
}

impl From<ChainPaymentPayload> for String {
    fn from(val: ChainPaymentPayload) -> Self {
        serde_json::to_string(&val).expect("Failed to serialize ChainPaymentPayload to String")
    }
}

/// Payload for status update instructions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatusUpdatePayload {
    pub reference: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

impl StatusUpdatePayload {
    pub fn new(
        reference: String,
        status: String,
        message: Option<String>,
        transaction_id: Option<String>,
    ) -> Result<Self, ValidationError> {
        let payload = Self {
            reference: reference.clone(),
            status: status.clone(),
            message,
            transaction_id,
        };

        payload.validate()?;
        Ok(payload)
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        validate_required_fields(
            &[("reference", &self.reference), ("status", &self.status)],
            "StatusUpdatePayload",
        )
    }
}

impl From<String> for StatusUpdatePayload {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).expect("Failed to deserialize StatusUpdatePayload from String")
    }
}

impl From<StatusUpdatePayload> for String {
    fn from(val: StatusUpdatePayload) -> Self {
        serde_json::to_string(&val).expect("Failed to serialize StatusUpdatePayload to String")
    }
}

/// Payload for correction instructions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CorrectionPayload {
    pub reference: String,
    pub value: String,
    pub message: String,
    pub currency: String,
    pub source: String,
}

impl CorrectionPayload {
    pub fn new(
        reference: String,
        value: String,
        message: String,
        currency: String,
        source: String,
    ) -> Result<Self, ValidationError> {
        let payload = Self {
            reference: reference.clone(),
            value: value.clone(),
            message: message.clone(),
            currency: currency.clone(),
            source: source.clone(),
        };

        payload.validate()?;
        Ok(payload)
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        validate_required_fields(
            &[
                ("reference", &self.reference),
                ("value", &self.value),
                ("message", &self.message),
                ("currency", &self.currency),
                ("source", &self.source),
            ],
            "CorrectionPayload",
        )
    }
}

impl From<String> for CorrectionPayload {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).expect("Failed to deserialize CorrectionPayload from String")
    }
}

impl From<CorrectionPayload> for String {
    fn from(val: CorrectionPayload) -> Self {
        serde_json::to_string(&val).expect("Failed to serialize CorrectionPayload to String")
    }
}

/// Payload for transaction instructions
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionPayload {
    pub external_reference: String,
    pub source: String,
    pub reference: String,
    pub first_name: String,
    pub last_name: String,
    pub transaction_type: TransactionType,
    pub status: String,
    pub incoming_currency: String,
    pub outgoing_currency: String,
    pub value: String,
    pub fee: String,
    pub payer: Option<String>,
    pub payee: Option<String>,
}

impl TransactionPayload {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        external_reference: String,
        source: String,
        reference: String,
        first_name: String,
        last_name: String,
        transaction_type: TransactionType,
        status: String,
        incoming_currency: String,
        outgoing_currency: String,
        value: String,
        fee: String,
        payer: Option<String>,
        payee: Option<String>,
    ) -> Result<Self, ValidationError> {
        let payload = Self {
            external_reference: external_reference.clone(),
            source: source.clone(),
            reference: reference.clone(),
            first_name: first_name.clone(),
            last_name: last_name.clone(),
            transaction_type,
            status: status.clone(),
            incoming_currency: incoming_currency.clone(),
            outgoing_currency: outgoing_currency.clone(),
            value: value.clone(),
            fee: fee.clone(),
            payer: payer.clone(),
            payee: payee.clone(),
        };

        payload.validate()?;
        Ok(payload)
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        // Validate transaction type specific requirements
        match self.transaction_type {
            TransactionType::Deposit if self.payer.is_none() => {
                return Err(ValidationError {
                    class_name: "TransactionPayload".to_string(),
                    fields: vec!["payer (required for DEPOSIT transactions)".to_string()],
                });
            }
            TransactionType::Withdraw if self.payee.is_none() => {
                return Err(ValidationError {
                    class_name: "TransactionPayload".to_string(),
                    fields: vec!["payee (required for WITHDRAW transactions)".to_string()],
                });
            }
            _ => {}
        }

        validate_required_fields(
            &[
                ("external_reference", &self.external_reference),
                ("source", &self.source),
                ("reference", &self.reference),
                ("first_name", &self.first_name),
                ("last_name", &self.last_name),
                ("status", &self.status),
                ("incoming_currency", &self.incoming_currency),
                ("outgoing_currency", &self.outgoing_currency),
                ("value", &self.value),
                ("fee", &self.fee),
            ],
            "TransactionPayload",
        )
    }
}

impl From<String> for TransactionPayload {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).expect("Failed to deserialize TransactionPayload from String")
    }
}

impl From<TransactionPayload> for String {
    fn from(val: TransactionPayload) -> Self {
        serde_json::to_string(&val).expect("Failed to serialize TransactionPayload to String")
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BankPaymentRequestPayload {
    pub reference: String,
    pub value: String,
    pub currency: String,
    pub profile_id: String,
    pub message: Option<String>,
}

impl BankPaymentRequestPayload {
    pub fn new(
        reference: String,
        value: String,
        currency: String,
        profile_id: String,
        message: Option<String>,
    ) -> Result<Self, ValidationError> {
        let payload = Self {
            reference: reference.clone(),
            value: value.clone(),
            currency: currency.clone(),
            profile_id: profile_id.clone(),
            message,
        };

        payload.validate()?;
        Ok(payload)
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        validate_required_fields(
            &[
                ("reference", &self.reference),
                ("value", &self.value),
                ("currency", &self.currency),
                ("profile_id", &self.profile_id),
            ],
            "BankPaymentRequestPayload",
        )
    }
}

impl From<String> for BankPaymentRequestPayload {
    fn from(value: String) -> Self {
        serde_json::from_str(&value)
            .expect("Failed to deserialize BankPaymentRequestPayload from String")
    }
}

impl From<BankPaymentRequestPayload> for String {
    fn from(val: BankPaymentRequestPayload) -> Self {
        serde_json::to_string(&val)
            .expect("Failed to serialize BankPaymentRequestPayload to String")
    }
}

/// Payload for profile update instructions
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateProfilePayload {
    pub profile_id: String,
    pub address_line_1: Option<String>,
    pub address_line_2: Option<String>,
    pub bank_account_number: Option<String>,
    pub bank_number: Option<String>,
    pub tax_id: Option<String>,
    pub tax_id_name: Option<String>,
    pub id_country_code: Option<String>,
    pub suspended_at: Option<String>,
    pub deleted_at: Option<String>,
}

impl UpdateProfilePayload {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        profile_id: String,
        address_line_1: Option<String>,
        address_line_2: Option<String>,
        bank_account_number: Option<String>,
        bank_number: Option<String>,
        tax_id: Option<String>,
        tax_id_name: Option<String>,
        id_country_code: Option<String>,
        suspended_at: Option<String>,
        deleted_at: Option<String>,
    ) -> Self {
        Self {
            profile_id,
            address_line_1,
            address_line_2,
            bank_account_number,
            bank_number,
            tax_id,
            tax_id_name,
            id_country_code,
            suspended_at,
            deleted_at,
        }
    }
}

impl From<String> for UpdateProfilePayload {
    fn from(value: String) -> Self {
        serde_json::from_str(&value)
            .expect("Failed to deserialize UpdateProfilePayload from String")
    }
}

impl From<UpdateProfilePayload> for String {
    fn from(val: UpdateProfilePayload) -> Self {
        serde_json::to_string(&val).expect("Failed to serialize UpdateProfilePayload to String")
    }
}

/// Payload for mint instructions
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MintPayload {
    pub value: String,
    pub currency: String,
    pub reference: String,
    pub chain: String,
    pub message: Option<String>,
}

impl MintPayload {
    pub fn new(
        value: String,
        currency: String,
        reference: String,
        chain: String,
        message: Option<String>,
    ) -> Result<Self, ValidationError> {
        let payload = Self {
            value,
            currency,
            reference,
            chain,
            message,
        };

        payload.validate()?;
        Ok(payload)
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        validate_required_fields(
            &[
                ("value", &self.value),
                ("currency", &self.currency),
                ("reference", &self.reference),
                ("chain", &self.chain),
            ],
            "MintPayload",
        )
    }
}

impl From<String> for MintPayload {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).expect("Failed to deserialize MintPayload from String")
    }
}

impl From<MintPayload> for String {
    fn from(val: MintPayload) -> Self {
        serde_json::to_string(&val).expect("Failed to serialize MintPayload to String")
    }
}

/// Payload for burn instructions
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BurnPayload {
    pub value: String,
    pub currency: String,
    pub reference: String,
    pub chain: String,
    pub message: Option<String>,
}

impl BurnPayload {
    pub fn new(
        value: String,
        currency: String,
        reference: String,
        chain: String,
        message: Option<String>,
    ) -> Result<Self, ValidationError> {
        let payload = Self {
            value,
            currency,
            reference,
            chain,
            message,
        };

        payload.validate()?;
        Ok(payload)
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        validate_required_fields(
            &[
                ("value", &self.value),
                ("currency", &self.currency),
                ("reference", &self.reference),
                ("chain", &self.chain),
            ],
            "BurnPayload",
        )
    }
}

impl From<String> for BurnPayload {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).expect("Failed to deserialize BurnPayload from String")
    }
}

impl From<BurnPayload> for String {
    fn from(val: BurnPayload) -> Self {
        serde_json::to_string(&val).expect("Failed to serialize BurnPayload to String")
    }
}
