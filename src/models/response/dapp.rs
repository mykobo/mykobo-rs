use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Transaction type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum TransactionType {
    Deposit,
    Withdraw,
}

/// Transaction status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionStatus {
    PendingPayer,
    PendingPayee,
    Completed,
    Failed,
    Cancelled,
}

/// Transaction source enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionSource {
    AnchorSolana,
    AnchorStellar,
}

/// Model for storing transaction records sent to the ledger.
///
/// This stores a local copy of all transactions created through the dApp
/// before they are sent to the ledger service.
///
/// Note: The 'dapp' schema is used for PostgreSQL in production.
/// For SQLite (used in tests), no schema is specified as SQLite doesn't support schemas.
/// The test configuration overrides this by using SQLite's in-memory database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Primary key
    pub id: String,

    /// Transaction identifiers
    pub reference: String,
    pub idempotency_key: String,

    /// Transaction details
    pub transaction_type: TransactionType,
    pub status: TransactionStatus,
    pub incoming_currency: String,
    pub outgoing_currency: String,
    /// Stored as string to preserve precision (equivalent to Numeric(20, 6))
    pub value: String,
    /// Stored as string to preserve precision (equivalent to Numeric(20, 6))
    pub fee: String,

    /// User information
    pub payer_id: Option<String>,
    pub payee_id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub wallet_address: String,

    /// Source and metadata
    pub source: TransactionSource,
    /// IPv4 or IPv6 address
    pub ip_address: Option<String>,

    /// Message queue tracking
    /// SQS Message ID
    pub message_id: Option<String>,
    pub queue_sent_at: Option<DateTime<Utc>>,

    /// Blockchain transaction hash (Solana signature)
    pub tx_hash: Option<String>,

    /// Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Transaction {
    /// Creates a new Transaction with default values
    pub fn new(
        reference: String,
        idempotency_key: String,
        transaction_type: TransactionType,
        status: TransactionStatus,
        incoming_currency: String,
        outgoing_currency: String,
        value: String,
        fee: String,
        wallet_address: String,
        source: TransactionSource
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            reference,
            idempotency_key,
            transaction_type,
            status,
            incoming_currency,
            outgoing_currency,
            value,
            fee,
            payer_id: None,
            payee_id: None,
            first_name: None,
            last_name: None,
            wallet_address,
            source,
            ip_address: None,
            message_id: None,
            queue_sent_at: None,
            tx_hash: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Updates the transaction status and refreshes the updated_at timestamp
    pub fn update_status(&mut self, status: TransactionStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    /// Sets the blockchain transaction hash
    pub fn set_tx_hash(&mut self, tx_hash: String) {
        self.tx_hash = Some(tx_hash);
        self.updated_at = Utc::now();
    }

    /// Sets the message queue information
    pub fn set_queue_info(&mut self, message_id: String) {
        self.message_id = Some(message_id);
        self.queue_sent_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Sets the payer information
    pub fn set_payer(&mut self, payer_id: String, first_name: Option<String>, last_name: Option<String>) {
        self.payer_id = Some(payer_id);
        self.first_name = first_name;
        self.last_name = last_name;
        self.updated_at = Utc::now();
    }

    /// Sets the payee information
    pub fn set_payee(&mut self, payee_id: String) {
        self.payee_id = Some(payee_id);
        self.updated_at = Utc::now();
    }

    /// Sets the IP address
    pub fn set_ip_address(&mut self, ip_address: String) {
        self.ip_address = Some(ip_address);
        self.updated_at = Utc::now();
    }
}

impl Default for Transaction {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            reference: String::new(),
            idempotency_key: String::new(),
            transaction_type: TransactionType::Deposit,
            status: TransactionStatus::PendingPayer,
            incoming_currency: String::new(),
            outgoing_currency: String::new(),
            value: String::from("0.0"),
            fee: String::from("0.0"),
            payer_id: None,
            payee_id: None,
            first_name: None,
            last_name: None,
            wallet_address: String::new(),
            source: TransactionSource::AnchorSolana,
            ip_address: None,
            message_id: None,
            queue_sent_at: None,
            tx_hash: None,
            created_at: now,
            updated_at: now,
        }
    }
}
