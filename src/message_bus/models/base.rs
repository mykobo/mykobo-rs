use serde::{Deserialize, Serialize};
use std::fmt;

/// Enum for message instruction types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InstructionType {
    Payment, // ledger payment instruction
    StatusUpdate,
    Correction,
    Transaction,
    BankPaymentRequest, // banking gateway payment request instruction
    ChainPayment,       // this is for anchors that require an update from the chain
    UpdateProfile,      // profile update instruction
    Mint,               // mint instruction - to convert FIAT to Crypto asset
    Burn,               // burn instruction - to convert Crypto asset to FIAT
}

impl fmt::Display for InstructionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_value(self)
                .ok()
                .and_then(|v| v.as_str().map(String::from))
                .unwrap_or_default()
        )
    }
}

/// Enum for transaction types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
    Deposit,
    Withdraw,
    Transfer,
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_value(self)
                .ok()
                .and_then(|v| v.as_str().map(String::from))
                .unwrap_or_default()
        )
    }
}

/// Enum for payment direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[derive(Default)]
pub enum PaymentDirection {
    #[default]
    Inbound,
    Outbound,
    Both,
}

impl fmt::Display for PaymentDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_value(self)
                .ok()
                .and_then(|v| v.as_str().map(String::from))
                .unwrap_or_default()
        )
    }
}

impl From<String> for PaymentDirection {
    fn from(value: String) -> Self {
        let normalized = format!("\"{}\"", value.trim_matches('"').to_uppercase());
        serde_json::from_str(&normalized)
            .expect("Failed to deserialize PaymentDirection from String")
    }
}

/// Enum for event types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventType {
    NewTransaction,
    TransactionStatusUpdate,
    Payment,
    BankPayment,
    NewProfile,
    NewUser,
    VerificationRequested,
    PasswordResetRequested,
    KycEvent,
    AddressOnboarded,
    RelayInitiated,
    RelayCompleted,
    RelayOnboarded,
    RelayStuckDepositing,
    RelayStuckBridging,
    RelayStuckForwarding,
    RelayFailed,
    #[serde(rename = "CIRCLE_API_5XX_BURST")]
    CircleApi5xxBurst,
    WebhookReprocessorBacklog,
    MintCompleted,
    BurnCompleted,
    MintHeld,
    BurnHeld,
    MintHeldAlert,
    BurnHeldAlert,
    CustomerNotifyFailed,
    MintInfo,
    BurnInfo,
    TransactionFailedAlert,
    TransactionHeldAlert,
    DepositInitiated,
    DepositCompleted,
    DepositFailed,
    WithdrawInitiated,
    WithdrawCompleted,
    WithdrawFailed,
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_value(self)
                .ok()
                .and_then(|v| v.as_str().map(String::from))
                .unwrap_or_default()
        )
    }
}

impl EventType {
    pub const ALL: &'static [EventType] = &[
        EventType::NewTransaction,
        EventType::TransactionStatusUpdate,
        EventType::Payment,
        EventType::BankPayment,
        EventType::NewProfile,
        EventType::NewUser,
        EventType::VerificationRequested,
        EventType::PasswordResetRequested,
        EventType::KycEvent,
        EventType::AddressOnboarded,
        EventType::RelayInitiated,
        EventType::RelayCompleted,
        EventType::RelayOnboarded,
        EventType::RelayStuckDepositing,
        EventType::RelayStuckBridging,
        EventType::RelayStuckForwarding,
        EventType::RelayFailed,
        EventType::CircleApi5xxBurst,
        EventType::WebhookReprocessorBacklog,
        EventType::MintCompleted,
        EventType::BurnCompleted,
        EventType::MintHeld,
        EventType::BurnHeld,
        EventType::MintHeldAlert,
        EventType::BurnHeldAlert,
        EventType::CustomerNotifyFailed,
        EventType::MintInfo,
        EventType::BurnInfo,
        EventType::TransactionFailedAlert,
        EventType::TransactionHeldAlert,
        EventType::DepositInitiated,
        EventType::DepositCompleted,
        EventType::DepositFailed,
        EventType::WithdrawInitiated,
        EventType::WithdrawCompleted,
        EventType::WithdrawFailed,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NewTransaction => "NEW_TRANSACTION",
            Self::TransactionStatusUpdate => "TRANSACTION_STATUS_UPDATE",
            Self::Payment => "PAYMENT",
            Self::BankPayment => "BANK_PAYMENT",
            Self::NewProfile => "NEW_PROFILE",
            Self::NewUser => "NEW_USER",
            Self::VerificationRequested => "VERIFICATION_REQUESTED",
            Self::PasswordResetRequested => "PASSWORD_RESET_REQUESTED",
            Self::KycEvent => "KYC_EVENT",
            Self::AddressOnboarded => "ADDRESS_ONBOARDED",
            Self::RelayInitiated => "RELAY_INITIATED",
            Self::RelayCompleted => "RELAY_COMPLETED",
            Self::RelayOnboarded => "RELAY_ONBOARDED",
            Self::RelayStuckDepositing => "RELAY_STUCK_DEPOSITING",
            Self::RelayStuckBridging => "RELAY_STUCK_BRIDGING",
            Self::RelayStuckForwarding => "RELAY_STUCK_FORWARDING",
            Self::RelayFailed => "RELAY_FAILED",
            Self::CircleApi5xxBurst => "CIRCLE_API_5XX_BURST",
            Self::WebhookReprocessorBacklog => "WEBHOOK_REPROCESSOR_BACKLOG",
            Self::MintCompleted => "MINT_COMPLETED",
            Self::BurnCompleted => "BURN_COMPLETED",
            Self::MintHeld => "MINT_HELD",
            Self::BurnHeld => "BURN_HELD",
            Self::MintHeldAlert => "MINT_HELD_ALERT",
            Self::BurnHeldAlert => "BURN_HELD_ALERT",
            Self::CustomerNotifyFailed => "CUSTOMER_NOTIFY_FAILED",
            Self::MintInfo => "MINT_INFO",
            Self::BurnInfo => "BURN_INFO",
            Self::TransactionFailedAlert => "TRANSACTION_FAILED_ALERT",
            Self::TransactionHeldAlert => "TRANSACTION_HELD_ALERT",
            Self::DepositInitiated => "DEPOSIT_INITIATED",
            Self::DepositCompleted => "DEPOSIT_COMPLETED",
            Self::DepositFailed => "DEPOSIT_FAILED",
            Self::WithdrawInitiated => "WITHDRAW_INITIATED",
            Self::WithdrawCompleted => "WITHDRAW_COMPLETED",
            Self::WithdrawFailed => "WITHDRAW_FAILED",
        }
    }
}

impl TryFrom<&str> for EventType {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::ALL.iter().find(|e| e.as_str() == s).copied().ok_or_else(|| s.to_string())
    }
}

/// Validation error for required fields
#[derive(Debug, Clone, thiserror::Error)]
pub struct ValidationError {
    pub class_name: String,
    pub fields: Vec<String>,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} missing required fields: {}",
            self.class_name,
            self.fields.join(", ")
        )
    }
}

/// Validate that required string fields are not empty or whitespace only
pub fn validate_required_fields(
    fields: &[(&str, &str)],
    class_name: &str,
) -> Result<(), ValidationError> {
    let missing_fields: Vec<String> = fields
        .iter()
        .filter(|(_, value)| value.trim().is_empty())
        .map(|(name, _)| name.to_string())
        .collect();

    if !missing_fields.is_empty() {
        return Err(ValidationError {
            class_name: class_name.to_string(),
            fields: missing_fields,
        });
    }

    Ok(())
}
