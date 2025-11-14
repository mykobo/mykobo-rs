use super::base::{validate_required_fields, TransactionType, ValidationError};
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
    pub reference: String,
    pub bank_account_number: Option<String>,
}

impl PaymentPayload {
    pub fn new(
        external_reference: String,
        currency: String,
        value: String,
        source: String,
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

/// Payload for status update instructions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatusUpdatePayload {
    pub reference: String,
    pub status: String,
    pub message: String,
}

impl StatusUpdatePayload {
    pub fn new(reference: String, status: String, message: String) -> Result<Self, ValidationError> {
        let payload = Self {
            reference: reference.clone(),
            status: status.clone(),
            message: message.clone(),
        };

        payload.validate()?;
        Ok(payload)
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        validate_required_fields(
            &[
                ("reference", &self.reference),
                ("status", &self.status),
                ("message", &self.message),
            ],
            "StatusUpdatePayload",
        )
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payment_payload_valid() {
        let payload = PaymentPayload::new(
            "P763763453G".to_string(),
            "EUR".to_string(),
            "123.00".to_string(),
            "BANK_MODULR".to_string(),
            "MYK123344545".to_string(),
            Some("John Doe".to_string()),
            Some("GB123266734836738787454".to_string()),
        );
        assert!(payload.is_ok());
    }

    #[test]
    fn test_payment_payload_missing_required() {
        let payload = PaymentPayload::new(
            "".to_string(),
            "EUR".to_string(),
            "123.00".to_string(),
            "BANK_MODULR".to_string(),
            "MYK123344545".to_string(),
            None,
            None,
        );
        assert!(payload.is_err());
    }

    #[test]
    fn test_transaction_payload_deposit_requires_payer() {
        let payload = TransactionPayload::new(
            "EXT123".to_string(),
            "BANKING_SERVICE".to_string(),
            "REF123".to_string(),
            "John".to_string(),
            "Doe".to_string(),
            TransactionType::Deposit,
            "PENDING".to_string(),
            "EUR".to_string(),
            "USD".to_string(),
            "100.00".to_string(),
            "1.50".to_string(),
            None, // Missing payer
            None,
        );
        assert!(payload.is_err());
    }

    #[test]
    fn test_transaction_payload_withdraw_requires_payee() {
        let payload = TransactionPayload::new(
            "EXT123".to_string(),
            "BANKING_SERVICE".to_string(),
            "REF123".to_string(),
            "John".to_string(),
            "Doe".to_string(),
            TransactionType::Withdraw,
            "PENDING".to_string(),
            "EUR".to_string(),
            "USD".to_string(),
            "100.00".to_string(),
            "1.50".to_string(),
            None,
            None, // Missing payee
        );
        assert!(payload.is_err());
    }
}
