use mykobo_rs::anchor::models::{
    DappTransaction as Transaction, TransactionSource, TransactionStatus, TransactionType,
};

#[test]
fn test_new_transaction() {
    let tx = Transaction::new(
        "REF-123".to_string(),
        "IDEMPOTENCY-123".to_string(),
        TransactionType::Deposit,
        TransactionStatus::PendingPayer,
        "USD".to_string(),
        "USDC".to_string(),
        "100.50".to_string(),
        "0.25".to_string(),
        "wallet-address-123".to_string(),
        TransactionSource::AnchorSolana,
    );

    assert_eq!(tx.reference, "REF-123");
    assert_eq!(tx.idempotency_key, "IDEMPOTENCY-123");
    assert_eq!(tx.transaction_type, TransactionType::Deposit);
    assert_eq!(tx.status, TransactionStatus::PendingPayer);
    assert_eq!(tx.value, "100.50");
    assert_eq!(tx.fee, "0.25");
    assert!(tx.payer_id.is_none());
    assert!(tx.tx_hash.is_none());
}

#[test]
fn test_update_status() {
    let mut tx = Transaction::default();
    let original_time = tx.updated_at;

    std::thread::sleep(std::time::Duration::from_millis(10));
    tx.update_status(TransactionStatus::Completed);

    assert_eq!(tx.status, TransactionStatus::Completed);
    assert!(tx.updated_at > original_time);
}

#[test]
fn test_set_tx_hash() {
    let mut tx = Transaction::default();
    tx.set_tx_hash("solana-signature-hash".to_string());

    assert_eq!(tx.tx_hash, Some("solana-signature-hash".to_string()));
}

#[test]
fn test_set_queue_info() {
    let mut tx = Transaction::default();
    tx.set_queue_info("sqs-message-id-123".to_string());

    assert_eq!(tx.message_id, Some("sqs-message-id-123".to_string()));
    assert!(tx.queue_sent_at.is_some());
}

#[test]
fn test_set_payer() {
    let mut tx = Transaction::default();
    tx.set_payer(
        "payer-123".to_string(),
        Some("John".to_string()),
        Some("Doe".to_string()),
    );

    assert_eq!(tx.payer_id, Some("payer-123".to_string()));
    assert_eq!(tx.first_name, Some("John".to_string()));
    assert_eq!(tx.last_name, Some("Doe".to_string()));
}

#[test]
fn test_serialization() {
    let tx = Transaction::default();
    let serialized = serde_json::to_string(&tx).unwrap();
    let deserialized: Transaction = serde_json::from_str(&serialized).unwrap();

    assert_eq!(tx.id, deserialized.id);
    assert_eq!(tx.status, deserialized.status);
}

#[test]
fn test_deserialization() {
    let payload = r#"
          {
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "reference": "TX_2024_03_15_ABC123",
            "external_reference": "a8b9c1d2-e3f4-5678-90ab-cdef12345678",
            "idempotency_key": "idempotency_2024_03_15_xyz789",
            "transaction_type": "WITHDRAW",
            "status": "COMPLETED",
            "incoming_currency": "USDC",
            "outgoing_currency": "USD",
            "value": "250.00",
            "fee": "5.50",
            "payer_id": null,
            "payee_id": "user_abc123xyz",
            "first_name": "John",
            "last_name": "Doe",
            "wallet_address": "7xKWv8QRt9YZN3pM5cD2jFqH4sX6wL8aB1vN9mT5rP3k",
            "source": "ANCHOR_SOLANA",
            "tx_hash": "5wHzKFxC2jL9mN8pQ3rT6vX4bY1cD7eF9gH2iJ5kM8nP0qR3sT6vW9xY1zA4bC7dE",
            "created_at": "2024-03-15T10:30:45.123456Z",
            "updated_at": "2024-03-15T10:32:18.654321Z",
            "message_id": "b2e5f8a1-c3d4-5678-90ab-def123456789",
            "queue_sent_at": "2024-03-15T10:30:46.789012Z"
          }
        "#;

    let transaction: Transaction = serde_json::from_str(payload).unwrap();
    assert_eq!(
        transaction.wallet_address,
        "7xKWv8QRt9YZN3pM5cD2jFqH4sX6wL8aB1vN9mT5rP3k"
    );
    assert_eq!(transaction.source, TransactionSource::AnchorSolana);
    assert_eq!(transaction.status, TransactionStatus::Completed);
    assert_eq!(transaction.transaction_type, TransactionType::Withdraw);
    assert_eq!(transaction.incoming_currency, "USDC");
    assert_eq!(transaction.outgoing_currency, "USD");
    assert_eq!(transaction.value, "250.00");
    assert_eq!(transaction.fee, "5.50");
    assert!(transaction
        .payee_id
        .is_some_and(|id| id == "user_abc123xyz"));
    assert!(transaction.first_name.is_some_and(|name| name == "John"));
    assert!(transaction.last_name.is_some_and(|name| name == "Doe"));
    assert!(transaction
        .message_id
        .is_some_and(|id| id == "b2e5f8a1-c3d4-5678-90ab-def123456789"));
}

#[test]
fn test_solana_deposit_deserialization() {
    let payload = r#"
        {
            "created_at": "2025-11-17T20:10:29.815872+00:00",
            "fee": "0.250000",
            "first_name": "Kwabena",
            "id": "29147822-396e-4817-9b64-931c92a05f46",
            "idempotency_key": "e1d3bdea-77e0-4f18-9c30-a7569b1e3c6f",
            "incoming_currency": "EUR",
            "last_name": "Aning",
            "message_id": "888ce30a-b5b1-4aff-9ff1-5be91c2e5f07",
            "outgoing_currency": "EURC",
            "payee_id": null,
            "payer_id": "urn:usrp:a891c5585c604b7aa2fd77410d5dc8dc",
            "queue_sent_at": "2025-11-17T20:10:29.960538+00:00",
            "reference": "MYK1763410229",
            "source": "ANCHOR_SOLANA",
            "status": "PENDING_ANCHOR",
            "transaction_type": "DEPOSIT",
            "tx_hash": null,
            "updated_at": "2025-11-17T20:10:29.962143+00:00",
            "value": "20.000000",
            "wallet_address": "B2JAtKctzWLt4cegWpqBjRqABZDxSSBCNCXPP7Kyk24J"
        }
        "#;

    let transaction: Transaction = serde_json::from_str(payload).unwrap();

    // Verify basic transaction details
    assert_eq!(transaction.id, "29147822-396e-4817-9b64-931c92a05f46");
    assert_eq!(transaction.reference, "MYK1763410229");
    assert_eq!(transaction.idempotency_key, "e1d3bdea-77e0-4f18-9c30-a7569b1e3c6f");

    // Verify transaction type and status
    assert_eq!(transaction.transaction_type, TransactionType::Deposit);
    assert_eq!(transaction.status, TransactionStatus::PendingAnchor);

    // Verify currency and amounts
    assert_eq!(transaction.incoming_currency, "EUR");
    assert_eq!(transaction.outgoing_currency, "EURC");
    assert_eq!(transaction.value, "20.000000");
    assert_eq!(transaction.fee, "0.250000");

    // Verify user information
    assert_eq!(transaction.payer_id, Some("urn:usrp:a891c5585c604b7aa2fd77410d5dc8dc".to_string()));
    assert_eq!(transaction.payee_id, None);
    assert_eq!(transaction.first_name, Some("Kwabena".to_string()));
    assert_eq!(transaction.last_name, Some("Aning".to_string()));

    // Verify Solana-specific fields
    assert_eq!(transaction.wallet_address, "B2JAtKctzWLt4cegWpqBjRqABZDxSSBCNCXPP7Kyk24J");
    assert_eq!(transaction.source, TransactionSource::AnchorSolana);
    assert_eq!(transaction.tx_hash, None);

    // Verify message queue tracking
    assert_eq!(transaction.message_id, Some("888ce30a-b5b1-4aff-9ff1-5be91c2e5f07".to_string()));
    assert!(transaction.queue_sent_at.is_some());

    // Verify timestamps
    assert!(transaction.created_at.to_string().starts_with("2025-11-17"));
    assert!(transaction.updated_at.to_string().starts_with("2025-11-17"));
}
