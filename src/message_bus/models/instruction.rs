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

impl From<String> for PaymentPayload {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).expect("Failed to deserialize PaymentPayload from String")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChainPaymentPayload {
    chain: String,
    hash: String,
    reference: String,
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    transaction_id: Option<String>,
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

    #[test]
    fn test_payment_payload_from_string() {
        let json = r#"{
            "external_reference": "P763763453G",
            "currency": "EUR",
            "value": "123.00",
            "source": "BANK_MODULR",
            "reference": "MYK123344545",
            "payer_name": "John Doe",
            "bank_account_number": "GB123266734836738787454"
        }"#;

        let payload: PaymentPayload = json.to_string().into();
        assert_eq!(payload.external_reference, "P763763453G");
        assert_eq!(payload.currency, "EUR");
        assert_eq!(payload.value, "123.00");
        assert_eq!(payload.source, "BANK_MODULR");
        assert_eq!(payload.reference, "MYK123344545");
    }

    #[test]
    fn test_status_update_payload_from_string() {
        let json = r#"{
            "reference": "REF123",
            "status": "COMPLETED",
            "message": "Payment processed successfully"
        }"#;

        let payload: StatusUpdatePayload = json.to_string().into();
        assert_eq!(payload.reference, "REF123");
        assert_eq!(payload.status, "COMPLETED");
        assert_eq!(
            payload.message,
            Some("Payment processed successfully".to_string())
        );
    }

    #[test]
    fn test_status_update_payload_without_message() {
        let payload =
            StatusUpdatePayload::new("REF456".to_string(), "PENDING".to_string(), None, None);
        assert!(payload.is_ok());
        let p = payload.unwrap();
        assert_eq!(p.reference, "REF456");
        assert_eq!(p.status, "PENDING");
        assert_eq!(p.message, None);
    }

    #[test]
    fn test_correction_payload_from_string() {
        let json = r#"{
            "reference": "REF123",
            "value": "50.00",
            "message": "Corrected amount",
            "currency": "USD",
            "source": "BANK_XYZ"
        }"#;

        let payload: CorrectionPayload = json.to_string().into();
        assert_eq!(payload.reference, "REF123");
        assert_eq!(payload.value, "50.00");
        assert_eq!(payload.currency, "USD");
    }

    #[test]
    fn test_transaction_payload_from_string() {
        let json = r#"{
            "external_reference": "EXT123",
            "source": "BANKING_SERVICE",
            "reference": "REF123",
            "first_name": "John",
            "last_name": "Doe",
            "transaction_type": "DEPOSIT",
            "status": "PENDING",
            "incoming_currency": "EUR",
            "outgoing_currency": "USD",
            "value": "100.00",
            "fee": "1.50",
            "payer": "Bank Account 123"
        }"#;

        let payload: TransactionPayload = json.to_string().into();
        assert_eq!(payload.external_reference, "EXT123");
        assert_eq!(payload.first_name, "John");
        assert_eq!(payload.last_name, "Doe");
        assert_eq!(payload.transaction_type, TransactionType::Deposit);
    }

    #[test]
    fn test_bank_payment_request_payload_valid() {
        let payload = BankPaymentRequestPayload::new(
            "REF456".to_string(),
            "250.00".to_string(),
            "USD".to_string(),
            "PROFILE123".to_string(),
            Some("Test payment".to_string()),
        );
        assert!(payload.is_ok());
    }

    #[test]
    fn test_chain_payment_payload_valid() {
        let payload = ChainPaymentPayload::new(
            "STELLAR".to_string(),
            "0x123abc".to_string(),
            "REF123".to_string(),
            "CONFIRMED".to_string(),
            Some("TXN456".to_string()),
        );
        assert!(payload.is_ok());
    }

    #[test]
    fn test_chain_payment_payload_from_string() {
        let json = r#"{
            "chain": "ETHEREUM",
            "hash": "0xabc123def456",
            "reference": "REF321",
            "status": "PENDING",
            "transaction_id": "TXN789"
        }"#;

        let payload: ChainPaymentPayload = json.to_string().into();
        assert_eq!(payload.chain, "ETHEREUM");
        assert_eq!(payload.hash, "0xabc123def456");
        assert_eq!(payload.reference, "REF321");
        assert_eq!(payload.status, "PENDING");
        assert_eq!(payload.transaction_id, Some("TXN789".to_string()));
    }

    #[test]
    fn test_bank_payment_request_payload_from_string() {
        let json = r#"{
            "reference": "BANK_REF123",
            "value": "500.00",
            "currency": "GBP",
            "profile_id": "PROF456",
            "message": "Bank transfer"
        }"#;

        let payload: BankPaymentRequestPayload = json.to_string().into();
        assert_eq!(payload.reference, "BANK_REF123");
        assert_eq!(payload.value, "500.00");
        assert_eq!(payload.currency, "GBP");
        assert_eq!(payload.profile_id, "PROF456");
        assert_eq!(payload.message, Some("Bank transfer".to_string()));
    }
}
