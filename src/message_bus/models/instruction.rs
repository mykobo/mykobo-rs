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
            PaymentDirection::Inbound,
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
            PaymentDirection::Inbound,
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
            "direction": "INBOUND",
            "reference": "MYK123344545",
            "payer_name": "John Doe",
            "bank_account_number": "GB123266734836738787454"
        }"#;

        let payload: PaymentPayload = json.to_string().into();
        assert_eq!(payload.external_reference, "P763763453G");
        assert_eq!(payload.currency, "EUR");
        assert_eq!(payload.value, "123.00");
        assert_eq!(payload.source, "BANK_MODULR");
        assert_eq!(payload.direction, PaymentDirection::Inbound);
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

    // Serialization/Deserialization Round-trip Tests

    #[test]
    fn test_payment_payload_serialization_roundtrip() {
        let original = PaymentPayload::new(
            "P763763453G".to_string(),
            "EUR".to_string(),
            "123.00".to_string(),
            "BANK_MODULR".to_string(),
            PaymentDirection::Inbound,
            "MYK123344545".to_string(),
            Some("John Doe".to_string()),
            Some("GB123266734836738787454".to_string()),
        )
        .unwrap();

        let serialized: String = original.clone().into();
        let deserialized: PaymentPayload = serialized.into();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_payment_payload_serialization_without_optionals() {
        let payload = PaymentPayload::new(
            "P763763453G".to_string(),
            "EUR".to_string(),
            "123.00".to_string(),
            "BANK_MODULR".to_string(),
            PaymentDirection::Outbound,
            "MYK123344545".to_string(),
            None,
            None,
        )
        .unwrap();

        let serialized = serde_json::to_string(&payload).unwrap();

        // Optional fields should not appear in JSON
        assert!(!serialized.contains("payer_name"));
        assert!(!serialized.contains("bank_account_number"));

        let deserialized: PaymentPayload = serde_json::from_str(&serialized).unwrap();
        assert_eq!(payload, deserialized);
    }

    #[test]
    fn test_status_update_payload_serialization_roundtrip() {
        let original = StatusUpdatePayload::new(
            "REF123".to_string(),
            "COMPLETED".to_string(),
            Some("Payment processed".to_string()),
            Some("TXN456".to_string()),
        )
        .unwrap();

        let serialized: String = original.clone().into();
        let deserialized: StatusUpdatePayload = serialized.into();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_status_update_payload_serialization_without_optionals() {
        let payload =
            StatusUpdatePayload::new("REF123".to_string(), "PENDING".to_string(), None, None)
                .unwrap();

        let serialized = serde_json::to_string(&payload).unwrap();

        // Optional fields should not appear in JSON
        assert!(!serialized.contains("message"));
        assert!(!serialized.contains("transaction_id"));

        let deserialized: StatusUpdatePayload = serde_json::from_str(&serialized).unwrap();
        assert_eq!(payload, deserialized);
    }

    #[test]
    fn test_correction_payload_serialization_roundtrip() {
        let original = CorrectionPayload::new(
            "REF123".to_string(),
            "50.00".to_string(),
            "Corrected amount".to_string(),
            "USD".to_string(),
            "BANK_XYZ".to_string(),
        )
        .unwrap();

        let serialized: String = original.clone().into();
        let deserialized: CorrectionPayload = serialized.into();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_transaction_payload_serialization_roundtrip() {
        let original = TransactionPayload::new(
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
            Some("Bank Account 123".to_string()),
            None,
        )
        .unwrap();

        let serialized: String = original.clone().into();
        let deserialized: TransactionPayload = serialized.into();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_transaction_payload_serialization_without_optionals() {
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
            Some("Payee Account".to_string()),
        )
        .unwrap();

        let serialized = serde_json::to_string(&payload).unwrap();

        // Optional payer field should not appear in JSON
        assert!(!serialized.contains("payer"));
        // But payee should appear
        assert!(serialized.contains("payee"));

        let deserialized: TransactionPayload = serde_json::from_str(&serialized).unwrap();
        assert_eq!(payload, deserialized);
    }

    #[test]
    fn test_chain_payment_payload_serialization_roundtrip() {
        let original = ChainPaymentPayload::new(
            "STELLAR".to_string(),
            "0x123abc".to_string(),
            "REF123".to_string(),
            "CONFIRMED".to_string(),
            Some("TXN456".to_string()),
        )
        .unwrap();

        let serialized: String = original.clone().into();
        let deserialized: ChainPaymentPayload = serialized.into();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_chain_payment_payload_serialization_without_optionals() {
        let payload = ChainPaymentPayload::new(
            "ETHEREUM".to_string(),
            "0xabc123".to_string(),
            "REF321".to_string(),
            "PENDING".to_string(),
            None,
        )
        .unwrap();

        let serialized = serde_json::to_string(&payload).unwrap();

        // Optional field should not appear in JSON
        assert!(!serialized.contains("transaction_id"));

        let deserialized: ChainPaymentPayload = serde_json::from_str(&serialized).unwrap();
        assert_eq!(payload, deserialized);
    }

    #[test]
    fn test_bank_payment_request_payload_serialization_roundtrip() {
        let original = BankPaymentRequestPayload::new(
            "BANK_REF123".to_string(),
            "500.00".to_string(),
            "GBP".to_string(),
            "PROF456".to_string(),
            Some("Bank transfer".to_string()),
        )
        .unwrap();

        let serialized: String = original.clone().into();
        let deserialized: BankPaymentRequestPayload = serialized.into();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_bank_payment_request_payload_serialization_without_optionals() {
        let payload = BankPaymentRequestPayload::new(
            "BANK_REF123".to_string(),
            "500.00".to_string(),
            "GBP".to_string(),
            "PROF456".to_string(),
            None,
        )
        .unwrap();

        let serialized = serde_json::to_string(&payload).unwrap();

        // Optional field should not appear in JSON
        assert!(!serialized.contains("message"));

        let deserialized: BankPaymentRequestPayload = serde_json::from_str(&serialized).unwrap();
        assert_eq!(payload, deserialized);
    }

    #[test]
    fn test_payload_with_special_characters() {
        let payload = CorrectionPayload::new(
            "REF-123/456".to_string(),
            "50.00".to_string(),
            "Corrected: \"quoted\" & <escaped>".to_string(),
            "USD".to_string(),
            "BANK_XYZ".to_string(),
        )
        .unwrap();

        let serialized: String = payload.clone().into();
        let deserialized: CorrectionPayload = serialized.into();

        assert_eq!(payload, deserialized);
        assert_eq!(deserialized.message, "Corrected: \"quoted\" & <escaped>");
    }

    #[test]
    fn test_payload_with_unicode() {
        let payload = PaymentPayload::new(
            "P763763453G".to_string(),
            "EUR".to_string(),
            "123.00".to_string(),
            "BANK_MODULR".to_string(),
            PaymentDirection::Both,
            "MYK123344545".to_string(),
            Some("José María García 日本語 🚀".to_string()),
            None,
        )
        .unwrap();

        let serialized: String = payload.clone().into();
        let deserialized: PaymentPayload = serialized.into();

        assert_eq!(payload, deserialized);
        assert_eq!(
            deserialized.payer_name,
            Some("José María García 日本語 🚀".to_string())
        );
    }

    #[test]
    fn test_payment_payload_direction_field() {
        // Test Inbound direction
        let inbound_payload = PaymentPayload::new(
            "P123".to_string(),
            "EUR".to_string(),
            "100.00".to_string(),
            "BANK_TEST".to_string(),
            PaymentDirection::Inbound,
            "REF123".to_string(),
            None,
            None,
        )
        .unwrap();
        assert_eq!(inbound_payload.direction, PaymentDirection::Inbound);

        // Test Outbound direction
        let outbound_payload = PaymentPayload::new(
            "P456".to_string(),
            "USD".to_string(),
            "200.00".to_string(),
            "CHAIN_ETH".to_string(),
            PaymentDirection::Outbound,
            "REF456".to_string(),
            None,
            None,
        )
        .unwrap();
        assert_eq!(outbound_payload.direction, PaymentDirection::Outbound);

        // Test Both direction
        let both_payload = PaymentPayload::new(
            "P789".to_string(),
            "GBP".to_string(),
            "300.00".to_string(),
            "OTC_CIRCLE".to_string(),
            PaymentDirection::Both,
            "REF789".to_string(),
            None,
            None,
        )
        .unwrap();
        assert_eq!(both_payload.direction, PaymentDirection::Both);
    }

    #[test]
    fn test_payment_payload_direction_serialization() {
        let payload = PaymentPayload::new(
            "P999".to_string(),
            "EUR".to_string(),
            "50.00".to_string(),
            "BANK_MODULR".to_string(),
            PaymentDirection::Inbound,
            "REF999".to_string(),
            Some("Test User".to_string()),
            None,
        )
        .unwrap();

        let serialized = serde_json::to_string(&payload).unwrap();

        // Direction should be serialized as INBOUND
        assert!(serialized.contains("\"direction\":\"INBOUND\""));

        let deserialized: PaymentPayload = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.direction, PaymentDirection::Inbound);
    }

    #[test]
    fn test_payment_payload_all_directions_roundtrip() {
        for direction in [
            PaymentDirection::Inbound,
            PaymentDirection::Outbound,
            PaymentDirection::Both,
        ] {
            let payload = PaymentPayload::new(
                "P001".to_string(),
                "EUR".to_string(),
                "75.00".to_string(),
                "BANK_TEST".to_string(),
                direction,
                "REF001".to_string(),
                None,
                None,
            )
            .unwrap();

            let serialized: String = payload.clone().into();
            let deserialized: PaymentPayload = serialized.into();

            assert_eq!(payload.direction, deserialized.direction);
        }
    }

    #[test]
    fn test_update_profile_payload_with_all_fields() {
        let payload = UpdateProfilePayload::new(
            Some("123 Main Street".to_string()),
            Some("Apt 4B".to_string()),
            Some("GB12345678901234".to_string()),
            Some("123456".to_string()),
            Some("TAX123456".to_string()),
            Some("VAT".to_string()),
            Some("GB".to_string()),
            Some("2024-01-15T10:00:00Z".to_string()),
            Some("2024-01-20T10:00:00Z".to_string()),
        );

        assert_eq!(payload.address_line_1, Some("123 Main Street".to_string()));
        assert_eq!(payload.address_line_2, Some("Apt 4B".to_string()));
        assert_eq!(
            payload.bank_account_number,
            Some("GB12345678901234".to_string())
        );
        assert_eq!(payload.bank_number, Some("123456".to_string()));
        assert_eq!(payload.tax_id, Some("TAX123456".to_string()));
        assert_eq!(payload.tax_id_name, Some("VAT".to_string()));
        assert_eq!(payload.id_country_code, Some("GB".to_string()));
        assert_eq!(
            payload.suspended_at,
            Some("2024-01-15T10:00:00Z".to_string())
        );
        assert_eq!(payload.deleted_at, Some("2024-01-20T10:00:00Z".to_string()));
    }

    #[test]
    fn test_update_profile_payload_with_no_fields() {
        let payload =
            UpdateProfilePayload::new(None, None, None, None, None, None, None, None, None);

        assert_eq!(payload.address_line_1, None);
        assert_eq!(payload.address_line_2, None);
        assert_eq!(payload.bank_account_number, None);
        assert_eq!(payload.bank_number, None);
        assert_eq!(payload.tax_id, None);
        assert_eq!(payload.tax_id_name, None);
        assert_eq!(payload.id_country_code, None);
        assert_eq!(payload.suspended_at, None);
        assert_eq!(payload.deleted_at, None);
    }

    #[test]
    fn test_update_profile_payload_with_partial_fields() {
        let payload = UpdateProfilePayload::new(
            Some("456 Oak Avenue".to_string()),
            None,
            Some("DE89370400440532013000".to_string()),
            None,
            None,
            None,
            Some("DE".to_string()),
            None,
            None,
        );

        assert_eq!(payload.address_line_1, Some("456 Oak Avenue".to_string()));
        assert_eq!(payload.address_line_2, None);
        assert_eq!(
            payload.bank_account_number,
            Some("DE89370400440532013000".to_string())
        );
        assert_eq!(payload.id_country_code, Some("DE".to_string()));
    }

    #[test]
    fn test_update_profile_payload_from_string() {
        let json = r#"{
            "address_line_1": "789 Elm Road",
            "address_line_2": "Suite 100",
            "bank_account_number": "FR7630006000011234567890189",
            "bank_number": "30006",
            "tax_id": "FR12345678901",
            "tax_id_name": "SIRET",
            "id_country_code": "FR"
        }"#;

        let payload: UpdateProfilePayload = json.to_string().into();
        assert_eq!(payload.address_line_1, Some("789 Elm Road".to_string()));
        assert_eq!(payload.address_line_2, Some("Suite 100".to_string()));
        assert_eq!(
            payload.bank_account_number,
            Some("FR7630006000011234567890189".to_string())
        );
        assert_eq!(payload.bank_number, Some("30006".to_string()));
        assert_eq!(payload.tax_id, Some("FR12345678901".to_string()));
        assert_eq!(payload.tax_id_name, Some("SIRET".to_string()));
        assert_eq!(payload.id_country_code, Some("FR".to_string()));
        assert_eq!(payload.suspended_at, None);
        assert_eq!(payload.deleted_at, None);
    }

    #[test]
    fn test_update_profile_payload_serialization_roundtrip() {
        let original = UpdateProfilePayload::new(
            Some("123 Test Street".to_string()),
            Some("Floor 2".to_string()),
            Some("GB82WEST12345698765432".to_string()),
            Some("WEST12".to_string()),
            Some("GB123456789".to_string()),
            Some("UTR".to_string()),
            Some("GB".to_string()),
            Some("2024-06-01T00:00:00Z".to_string()),
            None,
        );

        let serialized: String = original.clone().into();
        let deserialized: UpdateProfilePayload = serialized.into();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_update_profile_payload_serialization_skips_none() {
        let payload = UpdateProfilePayload::new(
            Some("Only Address".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        let serialized = serde_json::to_string(&payload).unwrap();

        // Only address_line_1 should appear in JSON
        assert!(serialized.contains("address_line_1"));
        assert!(!serialized.contains("address_line_2"));
        assert!(!serialized.contains("bank_account_number"));
        assert!(!serialized.contains("bank_number"));
        assert!(!serialized.contains("tax_id"));
        assert!(!serialized.contains("tax_id_name"));
        assert!(!serialized.contains("id_country_code"));
        assert!(!serialized.contains("suspended_at"));
        assert!(!serialized.contains("deleted_at"));

        let deserialized: UpdateProfilePayload = serde_json::from_str(&serialized).unwrap();
        assert_eq!(payload, deserialized);
    }

    #[test]
    fn test_update_profile_payload_with_special_characters() {
        let payload = UpdateProfilePayload::new(
            Some("123 Müller-Straße".to_string()),
            Some("Bürö & Co.".to_string()),
            None,
            None,
            None,
            None,
            Some("DE".to_string()),
            None,
            None,
        );

        let serialized: String = payload.clone().into();
        let deserialized: UpdateProfilePayload = serialized.into();

        assert_eq!(payload, deserialized);
        assert_eq!(
            deserialized.address_line_1,
            Some("123 Müller-Straße".to_string())
        );
        assert_eq!(deserialized.address_line_2, Some("Bürö & Co.".to_string()));
    }
}
