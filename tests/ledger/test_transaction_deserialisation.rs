use crate::read_file;
use bigdecimal::BigDecimal;
use mykobo_rs::ledger::models::{
    ComplianceEventsResponse, TransactionResponse, TransactionListResponse,
    TransactionStatusesResponse,
};
use pretty_assertions::assert_eq;
use std::str::FromStr;
use mykobo_rs::ledger::models::response::TransactionDetailsResponse;

#[test]
fn test_deserialise_transaction_list() {
    let content = read_file("tests/ledger/fixtures/transaction_list.json");
    let result = serde_json::from_str::<TransactionListResponse>(&content);

    assert!(result.is_ok());
    let transaction_list = result.unwrap();

    // Verify pagination
    assert_eq!(transaction_list.page, 1);
    assert_eq!(transaction_list.limit, 10);

    // Verify we have 2 transactions
    assert_eq!(transaction_list.transactions.len(), 2);

    // Verify first transaction
    let first_tx = &transaction_list.transactions[0];
    assert_eq!(
        first_tx.id,
        "urn:tx:366a0c79997641a59d6298e29bfcec17".to_string()
    );
    assert_eq!(
        first_tx.external_reference,
        Some("770e1636-bf8e-4bf6-bf02-850b827da63a".to_string())
    );
    assert_eq!(first_tx.source, "ANCHOR_MYKOBO".to_string());
    assert_eq!(first_tx.reference, "MYK1724666903".to_string());
    assert_eq!(first_tx.transaction_type, "DEPOSIT".to_string());
    assert_eq!(first_tx.status, "PENDING_CHAIN".to_string());
    assert_eq!(first_tx.incoming_currency, "EUR".to_string());
    assert_eq!(first_tx.outgoing_currency, "EURC".to_string());
    assert_eq!(
        first_tx.requested_amount,
        BigDecimal::from_str("20").unwrap()
    );
    assert_eq!(
        first_tx.expected_amount_in,
        BigDecimal::from_str("20").unwrap()
    );
    assert_eq!(first_tx.amount_out, BigDecimal::from_str("17.65").unwrap());
    assert_eq!(first_tx.amount_in, BigDecimal::from_str("20").unwrap());
    assert_eq!(first_tx.fee, BigDecimal::from_str("2.35").unwrap());
    assert_eq!(
        first_tx.payer,
        Some("urn:usrp:b26734fce23e450b87368b22cf56e091".to_string())
    );
    assert_eq!(first_tx.payee, None);
    assert_eq!(first_tx.requester_first_name, "UNKNOWN".to_string());
    assert_eq!(first_tx.requester_last_name, "UNKNOWN".to_string());
    assert_eq!(first_tx.originating_ip_address, None);

    // Verify second transaction
    let second_tx = &transaction_list.transactions[1];
    assert_eq!(
        second_tx.id,
        "urn:tx:477b1d8aaa8752b6ae7399f3acfded28".to_string()
    );
    assert_eq!(second_tx.external_reference, None);
    assert_eq!(second_tx.source, "WALLET_SERVICE".to_string());
    assert_eq!(second_tx.transaction_type, "WITHDRAWAL".to_string());
    assert_eq!(second_tx.status, "COMPLETED".to_string());
    assert_eq!(
        second_tx.requested_amount,
        BigDecimal::from_str("50.5").unwrap()
    );
    assert_eq!(
        second_tx.payee,
        Some("urn:usrp:c37845gdf34f561c98479c33dg67f192".to_string())
    );
    assert_eq!(second_tx.payer, None);
    assert_eq!(second_tx.requester_first_name, "John".to_string());
    assert_eq!(second_tx.requester_last_name, "Doe".to_string());
    assert_eq!(
        second_tx.originating_ip_address,
        Some("192.168.1.100".to_string())
    );
}

#[test]
fn test_deserialise_transaction_by_external_id() {
    let content = read_file("tests/ledger/fixtures/transaction_by_external_id.json");
    let result = serde_json::from_str::<TransactionResponse>(&content);

    assert!(result.is_ok());
    let transaction = result.unwrap();

    assert_eq!(
        transaction.id,
        "urn:tx:366a0c79997641a59d6298e29bfcec17".to_string()
    );
    assert_eq!(
        transaction.external_reference,
        Some("770e1636-bf8e-4bf6-bf02-850b827da63a".to_string())
    );
    assert_eq!(transaction.source, "ANCHOR_MYKOBO".to_string());
    assert_eq!(transaction.reference, "MYK1724666903".to_string());
    assert_eq!(transaction.transaction_type, "DEPOSIT".to_string());
    assert_eq!(transaction.status, "PENDING_CHAIN".to_string());
    assert_eq!(transaction.incoming_currency, "EUR".to_string());
    assert_eq!(transaction.outgoing_currency, "EURC".to_string());

    // Verify BigDecimal amounts
    assert_eq!(
        transaction.requested_amount,
        BigDecimal::from_str("20").unwrap()
    );
    assert_eq!(
        transaction.expected_amount_in,
        BigDecimal::from_str("20").unwrap()
    );
    assert_eq!(
        transaction.amount_out,
        BigDecimal::from_str("17.65").unwrap()
    );
    assert_eq!(transaction.amount_in, BigDecimal::from_str("20").unwrap());
    assert_eq!(transaction.fee, BigDecimal::from_str("2.35").unwrap());

    assert_eq!(
        transaction.payer,
        Some("urn:usrp:b26734fce23e450b87368b22cf56e091".to_string())
    );
    assert_eq!(transaction.payee, None);
    assert_eq!(transaction.requester_first_name, "UNKNOWN".to_string());
    assert_eq!(transaction.requester_last_name, "UNKNOWN".to_string());
    assert_eq!(transaction.originating_ip_address, None);
}

#[test]
fn test_deserialise_transaction_compliance_events() {
    let content = read_file("tests/ledger/fixtures/transaction_compliance_events.json");
    let result = serde_json::from_str::<ComplianceEventsResponse>(&content);

    assert!(result.is_ok());
    let compliance_events = result.unwrap();

    // Verify all compliance checks are present and true
    assert_eq!(compliance_events.len(), 6);
    assert_eq!(compliance_events.get("NAME_VERIFIER"), Some(&true));
    assert_eq!(
        compliance_events.get("FINAL_APPROVAL_VERIFIER"),
        Some(&true)
    );
    assert_eq!(compliance_events.get("GEOLOCATION_VERIFIER"), Some(&true));
    assert_eq!(
        compliance_events.get("ADDRESS_SCREENING_VERIFIER"),
        Some(&true)
    );
    assert_eq!(compliance_events.get("KYC_VERIFIER"), Some(&true));
    assert_eq!(
        compliance_events.get("INCOMING_PAYMENT_IBAN_VERIFIER"),
        Some(&true)
    );
}

#[test]
fn test_deserialise_transaction_statuses() {
    let content = read_file("tests/ledger/fixtures/transaction_statuses.json");
    let result = serde_json::from_str::<TransactionStatusesResponse>(&content);

    assert!(result.is_ok());
    let statuses = result.unwrap();

    // Verify we have 2 statuses
    assert_eq!(statuses.len(), 2);
    assert_eq!(statuses[0], "HELD".to_string());
    assert_eq!(statuses[1], "HOLD_RELEASED".to_string());
}

#[test]
fn test_deserialise_transaction_with_null_values() {
    // Test that we can handle transactions with null/None values properly
    let json = r#"{
        "id": "urn:tx:test123",
        "external_reference": null,
        "source": "TEST_SOURCE",
        "reference": "TEST_REF",
        "transaction_type": "TRANSFER",
        "status": "PENDING",
        "incoming_currency": "USD",
        "outgoing_currency": "EUR",
        "requested_amount": 100.00,
        "expected_amount_in": 100.00,
        "amount_out": 95.00,
        "amount_in": 100.00,
        "fee": 5.00,
        "payer": null,
        "payee": null,
        "created_at": "2024-01-01T00:00:00",
        "updated_at": null,
        "requester_first_name": "Test",
        "requester_last_name": "User",
        "originating_ip_address": null
    }"#;

    let result = serde_json::from_str::<TransactionResponse>(json);
    assert!(result.is_ok());

    let transaction = result.unwrap();
    assert_eq!(transaction.external_reference, None);
    assert_eq!(transaction.payer, None);
    assert_eq!(transaction.payee, None);
    assert_eq!(transaction.updated_at, None);
    assert_eq!(transaction.originating_ip_address, None);
}

#[test]
fn test_deserialise_empty_compliance_events() {
    let json = "{}";
    let result = serde_json::from_str::<ComplianceEventsResponse>(json);
    assert!(result.is_ok());
    let compliance_events = result.unwrap();
    assert_eq!(compliance_events.len(), 0);
}

#[test]
fn test_deserialise_empty_transaction_statuses() {
    let json = "[]";
    let result = serde_json::from_str::<TransactionStatusesResponse>(json);
    assert!(result.is_ok());
    let statuses = result.unwrap();
    assert_eq!(statuses.len(), 0);
}


#[test]
fn test_deserialise_transaction_details() {
    let content = read_file("tests/ledger/fixtures/transaction_details_by_reference.json");
    let result = serde_json::from_str::<TransactionDetailsResponse>(&content);

    let details = result.unwrap();
    assert_eq!(details.transaction.id, "urn:tx:963214b4d55d47cdb8b0fc1e4d9c9641".to_string());
    assert_eq!(details.events.len(), 5);
}