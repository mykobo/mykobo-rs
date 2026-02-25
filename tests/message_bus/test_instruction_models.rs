use mykobo_rs::message_bus::models::base::PaymentDirection;
use mykobo_rs::message_bus::models::instruction::*;
use mykobo_rs::message_bus::TransactionType;

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
    let payload = StatusUpdatePayload::new("REF456".to_string(), "PENDING".to_string(), None, None);
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
        StatusUpdatePayload::new("REF123".to_string(), "PENDING".to_string(), None, None).unwrap();

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
        "PROF123".to_string(),
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

    assert_eq!(payload.profile_id, "PROF123".to_string());
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
fn test_update_profile_payload_with_no_optional_fields() {
    let payload = UpdateProfilePayload::new(
        "PROF456".to_string(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );

    assert_eq!(payload.profile_id, "PROF456".to_string());
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
        "PROF789".to_string(),
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

    assert_eq!(payload.profile_id, "PROF789".to_string());
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
        "profile_id": "PROF_FR123",
        "address_line_1": "789 Elm Road",
        "address_line_2": "Suite 100",
        "bank_account_number": "FR7630006000011234567890189",
        "bank_number": "30006",
        "tax_id": "FR12345678901",
        "tax_id_name": "SIRET",
        "id_country_code": "FR"
    }"#;

    let payload: UpdateProfilePayload = json.to_string().into();
    assert_eq!(payload.profile_id, "PROF_FR123".to_string());
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
        "PROF_ROUND".to_string(),
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
        "PROF_SKIP".to_string(),
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

    // profile_id and address_line_1 should appear in JSON
    assert!(serialized.contains("profile_id"));
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
        "PROF_SPECIAL".to_string(),
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

// Mint payload tests

#[test]
fn test_mint_payload_valid() {
    let payload = MintPayload::new(
        "100.00".to_string(),
        "EUR".to_string(),
        "MYK123456".to_string(),
        "stellar".to_string(),
        Some("Mint for deposit".to_string()),
    );
    assert!(payload.is_ok());
}

#[test]
fn test_mint_payload_valid_without_message() {
    let payload = MintPayload::new(
        "50.00".to_string(),
        "USD".to_string(),
        "MYK789012".to_string(),
        "stellar".to_string(),
        None,
    );
    assert!(payload.is_ok());
    let p = payload.unwrap();
    assert_eq!(p.value, "50.00");
    assert_eq!(p.currency, "USD");
    assert_eq!(p.reference, "MYK789012");
    assert_eq!(p.message, None);
}

#[test]
fn test_mint_payload_missing_value() {
    let payload = MintPayload::new(
        "".to_string(),
        "EUR".to_string(),
        "MYK123456".to_string(),
        "stellar".to_string(),
        None,
    );
    assert!(payload.is_err());
}

#[test]
fn test_mint_payload_missing_currency() {
    let payload = MintPayload::new(
        "100.00".to_string(),
        "".to_string(),
        "MYK123456".to_string(),
        "stellar".to_string(),
        None,
    );
    assert!(payload.is_err());
}

#[test]
fn test_mint_payload_missing_reference() {
    let payload = MintPayload::new(
        "100.00".to_string(),
        "EUR".to_string(),
        "".to_string(),
        "stellar".to_string(),
        None,
    );
    assert!(payload.is_err());
}

#[test]
fn test_mint_payload_from_string() {
    let json = r#"{
        "value": "250.00",
        "currency": "GBP",
        "reference": "MYK_MINT_001",
        "chain": "stellar",
        "message": "Minting tokens"
    }"#;

    let payload: MintPayload = json.to_string().into();
    assert_eq!(payload.value, "250.00");
    assert_eq!(payload.currency, "GBP");
    assert_eq!(payload.reference, "MYK_MINT_001");
    assert_eq!(payload.chain, "stellar");
    assert_eq!(payload.message, Some("Minting tokens".to_string()));
}

#[test]
fn test_mint_payload_serialization_roundtrip() {
    let original = MintPayload::new(
        "100.00".to_string(),
        "EUR".to_string(),
        "MYK123456".to_string(),
        "stellar".to_string(),
        Some("Mint for deposit".to_string()),
    )
    .unwrap();

    let serialized: String = original.clone().into();
    let deserialized: MintPayload = serialized.into();

    assert_eq!(original, deserialized);
}

#[test]
fn test_mint_payload_serialization_without_optionals() {
    let payload = MintPayload::new(
        "100.00".to_string(),
        "EUR".to_string(),
        "MYK123456".to_string(),
        "stellar".to_string(),
        None,
    )
    .unwrap();

    let serialized = serde_json::to_string(&payload).unwrap();

    assert!(!serialized.contains("message"));

    let deserialized: MintPayload = serde_json::from_str(&serialized).unwrap();
    assert_eq!(payload, deserialized);
}

// Burn payload tests

#[test]
fn test_burn_payload_valid() {
    let payload = BurnPayload::new(
        "75.00".to_string(),
        "EUR".to_string(),
        "MYK654321".to_string(),
        "stellar".to_string(),
        Some("Burn for withdrawal".to_string()),
    );
    assert!(payload.is_ok());
}

#[test]
fn test_burn_payload_valid_without_message() {
    let payload = BurnPayload::new(
        "200.00".to_string(),
        "USD".to_string(),
        "MYK111222".to_string(),
        "stellar".to_string(),
        None,
    );
    assert!(payload.is_ok());
    let p = payload.unwrap();
    assert_eq!(p.value, "200.00");
    assert_eq!(p.currency, "USD");
    assert_eq!(p.reference, "MYK111222");
    assert_eq!(p.message, None);
}

#[test]
fn test_burn_payload_missing_value() {
    let payload = BurnPayload::new(
        "".to_string(),
        "EUR".to_string(),
        "MYK654321".to_string(),
        "stellar".to_string(),
        None,
    );
    assert!(payload.is_err());
}

#[test]
fn test_burn_payload_missing_currency() {
    let payload = BurnPayload::new(
        "75.00".to_string(),
        "".to_string(),
        "MYK654321".to_string(),
        "stellar".to_string(),
        None,
    );
    assert!(payload.is_err());
}

#[test]
fn test_burn_payload_missing_reference() {
    let payload = BurnPayload::new(
        "75.00".to_string(),
        "EUR".to_string(),
        "".to_string(),
        "stellar".to_string(),
        None,
    );
    assert!(payload.is_err());
}

#[test]
fn test_burn_payload_from_string() {
    let json = r#"{
        "value": "500.00",
        "currency": "USD",
        "reference": "MYK_BURN_001",
        "chain": "stellar",
        "message": "Burning tokens"
    }"#;

    let payload: BurnPayload = json.to_string().into();
    assert_eq!(payload.value, "500.00");
    assert_eq!(payload.currency, "USD");
    assert_eq!(payload.reference, "MYK_BURN_001");
    assert_eq!(payload.chain, "stellar");
    assert_eq!(payload.message, Some("Burning tokens".to_string()));
}

#[test]
fn test_burn_payload_serialization_roundtrip() {
    let original = BurnPayload::new(
        "75.00".to_string(),
        "EUR".to_string(),
        "MYK654321".to_string(),
        "stellar".to_string(),
        Some("Burn for withdrawal".to_string()),
    )
    .unwrap();

    let serialized: String = original.clone().into();
    let deserialized: BurnPayload = serialized.into();

    assert_eq!(original, deserialized);
}

#[test]
fn test_burn_payload_serialization_without_optionals() {
    let payload = BurnPayload::new(
        "75.00".to_string(),
        "EUR".to_string(),
        "MYK654321".to_string(),
        "stellar".to_string(),
        None,
    )
    .unwrap();

    let serialized = serde_json::to_string(&payload).unwrap();

    assert!(!serialized.contains("message"));

    let deserialized: BurnPayload = serde_json::from_str(&serialized).unwrap();
    assert_eq!(payload, deserialized);
}
