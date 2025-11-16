# Message Bus Documentation

This document provides comprehensive documentation for the MYKOBO message bus system, including all instruction and event types, their corresponding payloads, and usage examples.

## Table of Contents

- [Overview](#overview)
- [Quick Reference](#quick-reference)
- [Message Structure](#message-structure)
- [Instruction Types](#instruction-types)
- [Event Types](#event-types)
- [Payload Variants](#payload-variants)
- [Usage Examples](#usage-examples)
- [Best Practices](#best-practices)

## Overview

The MYKOBO message bus system provides a unified way to send instructions and events between services. Messages are validated to ensure type safety and data integrity.

### Key Features

- **Type-safe messaging**: Strict validation ensures instruction/event types match their payloads
- **Flexible deserialization**: All payloads support `From<String>` for easy JSON conversion
- **Raw payload support**: Send arbitrary data without strict typing when needed
- **Validation**: Required field validation at creation time
- **Metadata tracking**: Built-in support for idempotency, timestamps, and source tracking
- **Comprehensive coverage**: 6 instruction types and 8 event types fully supported

## Quick Reference

### Instruction Types Summary

| Type | Constant | Payload | Purpose |
|------|----------|---------|---------|
| Payment | `PAYMENT` | `PaymentPayload` | Record ledger payments |
| StatusUpdate | `STATUS_UPDATE` | `StatusUpdatePayload` | Update transaction/operation status |
| Correction | `CORRECTION` | `CorrectionPayload` | Correct transaction amounts |
| Transaction | `TRANSACTION` | `TransactionPayload` | Create/update complete transaction records |
| BankPaymentRequest | `BANK_PAYMENT_REQUEST` | `BankPaymentRequestPayload` | Request banking gateway payments |
| ChainPayment | `CHAIN_PAYMENT` | `ChainPaymentPayload` | Update blockchain payment status |

### Event Types Summary

| Type | Constant | Payload | Purpose |
|------|----------|---------|---------|
| NewTransaction | `NEW_TRANSACTION` | `NewTransactionEventPayload` | Notify new transaction created |
| TransactionStatusUpdate | `TRANSACTION_STATUS_UPDATE` | `TransactionStatusEventPayload` | Notify transaction status changed |
| Payment | `PAYMENT` | `PaymentEventPayload` | Notify payment received (bank/chain) |
| NewProfile | `NEW_PROFILE` | `ProfileEventPayload` | Notify new profile created |
| NewUser | `NEW_USER` | `NewUserEventPayload` | Notify new user account created |
| KycEvent | `KYC_EVENT` | `KycEventPayload` | Notify KYC status changes |
| PasswordResetRequested | `PASSWORD_RESET_REQUESTED` | `PasswordResetEventPayload` | Notify password reset requested |
| VerificationRequested | `VERIFICATION_REQUESTED` | `VerificationRequestedEventPayload` | Notify email verification requested |

## Message Structure

Every message bus message consists of two main components:

### MetaData

```rust
pub struct MetaData {
    pub source: String,              // Service/component sending the message
    pub created_at: String,          // ISO 8601 timestamp
    pub token: String,               // Service authentication token
    pub idempotency_key: String,     // Unique identifier for deduplication
    pub instruction_type: Option<InstructionType>,  // Set for instructions
    pub event: Option<EventType>,    // Set for events
}
```

**Validation Rules:**
- Either `instruction_type` OR `event` must be set (not both, not neither)
- All string fields must be non-empty
- Automatically generated if using `MessageBusMessage::create()`

### Payload

The payload contains the actual message data. It must match the specified instruction or event type.

```rust
pub enum Payload {
    // Instruction payloads
    Payment(PaymentPayload),
    StatusUpdate(StatusUpdatePayload),
    Correction(CorrectionPayload),
    Transaction(TransactionPayload),
    BankPaymentRequest(BankPaymentRequestPayload),
    ChainPayment(ChainPaymentPayload),

    // Event payloads
    NewTransaction(NewTransactionEventPayload),
    TransactionStatus(TransactionStatusEventPayload),
    PaymentEvent(PaymentEventPayload),
    Profile(ProfileEventPayload),
    NewUser(NewUserEventPayload),
    Kyc(KycEventPayload),
    PasswordReset(PasswordResetEventPayload),
    VerificationRequested(VerificationRequestedEventPayload),

    // Generic payload
    Raw(String),
}
```

## Instruction Types

Instructions are commands sent to services to perform specific actions. There are 6 instruction types in total.

### 1. Payment (`PAYMENT`)

**Purpose:** Ledger payment instruction for recording payments

**Payload:** `PaymentPayload`

```rust
pub struct PaymentPayload {
    pub external_reference: String,      // External payment reference ID
    pub payer_name: Option<String>,      // Name of the payer
    pub currency: String,                // Currency code (e.g., "EUR", "USD")
    pub value: String,                   // Payment amount as string
    pub source: String,                  // Payment source (e.g., "BANK_MODULR")
    pub reference: String,               // Internal reference ID
    pub bank_account_number: Option<String>, // Bank account number if applicable
}
```

**Required Fields:** `external_reference`, `currency`, `value`, `source`, `reference`

**Example:**
```rust
let payload = PaymentPayload::new(
    "P763763453G".to_string(),
    "EUR".to_string(),
    "123.00".to_string(),
    "BANK_MODULR".to_string(),
    "MYK123344545".to_string(),
    Some("John Doe".to_string()),
    Some("GB123266734836738787454".to_string()),
)?;

let message = MessageBusMessage::create(
    "BANKING_SERVICE".to_string(),
    Payload::Payment(payload),
    "service.token.here".to_string(),
    Some(InstructionType::Payment),
    None,
    None,
)?;
```

**JSON Example:**
```json
{
  "external_reference": "P763763453G",
  "currency": "EUR",
  "value": "123.00",
  "source": "BANK_MODULR",
  "reference": "MYK123344545",
  "payer_name": "John Doe",
  "bank_account_number": "GB123266734836738787454"
}
```

---

### 2. StatusUpdate (`STATUS_UPDATE`)

**Purpose:** Update the status of a transaction or operation

**Payload:** `StatusUpdatePayload`

```rust
pub struct StatusUpdatePayload {
    pub reference: String,              // Reference to the item being updated
    pub status: String,                 // New status value
    pub message: Option<String>,        // Optional status update message/reason
    pub transaction_id: Option<String>, // Optional associated transaction ID
}
```

**Required Fields:** `reference`, `status`

**Example:**
```rust
let payload = StatusUpdatePayload::new(
    "REF123".to_string(),
    "COMPLETED".to_string(),
    Some("Payment processed successfully".to_string()),
    Some("TXN456".to_string()),
)?;

// Or without message
let payload = StatusUpdatePayload::new(
    "REF456".to_string(),
    "PENDING".to_string(),
    None,
    None,
)?;
```

**JSON Example:**
```json
{
  "reference": "REF123",
  "status": "COMPLETED",
  "message": "Payment processed successfully",
  "transaction_id": "TXN456"
}
```

---

### 3. Correction (`CORRECTION`)

**Purpose:** Correct a previously submitted transaction amount or details

**Payload:** `CorrectionPayload`

```rust
pub struct CorrectionPayload {
    pub reference: String,   // Reference to the item being corrected
    pub value: String,       // Corrected amount
    pub message: String,     // Reason for correction
    pub currency: String,    // Currency code
    pub source: String,      // Source of the correction
}
```

**Required Fields:** All fields are required

**Example:**
```rust
let payload = CorrectionPayload::new(
    "REF123".to_string(),
    "50.00".to_string(),
    "Corrected amount due to pricing error".to_string(),
    "USD".to_string(),
    "BANK_XYZ".to_string(),
)?;
```

**JSON Example:**
```json
{
  "reference": "REF123",
  "value": "50.00",
  "message": "Corrected amount due to pricing error",
  "currency": "USD",
  "source": "BANK_XYZ"
}
```

---

### 4. Transaction (`TRANSACTION`)

**Purpose:** Create or update a complete transaction record

**Payload:** `TransactionPayload`

```rust
pub struct TransactionPayload {
    pub external_reference: String,
    pub source: String,
    pub reference: String,
    pub first_name: String,
    pub last_name: String,
    pub transaction_type: TransactionType,  // DEPOSIT or WITHDRAW
    pub status: String,
    pub incoming_currency: String,
    pub outgoing_currency: String,
    pub value: String,
    pub fee: String,
    pub payer: Option<String>,    // Required for DEPOSIT
    pub payee: Option<String>,    // Required for WITHDRAW
}
```

**Required Fields:** All fields except `payer` and `payee` (conditional)

**Validation Rules:**
- `payer` is required when `transaction_type` is `DEPOSIT`
- `payee` is required when `transaction_type` is `WITHDRAW`

**Example:**
```rust
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
    Some("Bank Account 123".to_string()),
    None,
)?;
```

**JSON Example:**
```json
{
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
}
```

---

### 5. BankPaymentRequest (`BANK_PAYMENT_REQUEST`)

**Purpose:** Banking gateway payment request instruction

**Payload:** `BankPaymentRequestPayload`

```rust
pub struct BankPaymentRequestPayload {
    pub reference: String,     // Payment reference
    pub value: String,         // Payment amount
    pub currency: String,      // Currency code
    pub profile_id: String,    // User profile ID
    pub message: Option<String>, // Optional payment message
}
```

**Required Fields:** `reference`, `value`, `currency`, `profile_id`

**Example:**
```rust
let payload = BankPaymentRequestPayload::new(
    "BANK_REF123".to_string(),
    "500.00".to_string(),
    "GBP".to_string(),
    "PROF456".to_string(),
    Some("Bank transfer".to_string()),
)?;
```

**JSON Example:**
```json
{
  "reference": "BANK_REF123",
  "value": "500.00",
  "currency": "GBP",
  "profile_id": "PROF456",
  "message": "Bank transfer"
}
```

---

### 6. ChainPayment (`CHAIN_PAYMENT`)

**Purpose:** Blockchain payment update for anchors requiring chain confirmation

**Payload:** `ChainPaymentPayload`

```rust
pub struct ChainPaymentPayload {
    pub chain: String,                    // Blockchain name (e.g., "STELLAR", "ETHEREUM")
    pub hash: String,                     // Transaction hash
    pub reference: String,                // Internal reference
    pub status: String,                   // Payment status
    pub transaction_id: Option<String>,   // Optional transaction ID
}
```

**Required Fields:** `chain`, `hash`, `reference`, `status`

**Example:**
```rust
let payload = ChainPaymentPayload::new(
    "STELLAR".to_string(),
    "0x123abc".to_string(),
    "REF123".to_string(),
    "CONFIRMED".to_string(),
    Some("TXN456".to_string()),
)?;
```

**JSON Example:**
```json
{
  "chain": "ETHEREUM",
  "hash": "0xabc123def456",
  "reference": "REF321",
  "status": "PENDING",
  "transaction_id": "TXN789"
}
```


## Event Types

Events are notifications about things that have happened in the system. There are 8 event types in total.

### 1. NewTransaction (`NEW_TRANSACTION`)

**Purpose:** Notify that a new transaction has been created

**Payload:** `NewTransactionEventPayload`

```rust
pub struct NewTransactionEventPayload {
    pub created_at: String,          // ISO 8601 timestamp
    pub kind: TransactionType,       // DEPOSIT or WITHDRAW
    pub reference: String,           // Transaction reference
    pub source: String,              // Source service
}
```

**Required Fields:** All fields

**Example:**
```rust
let payload = NewTransactionEventPayload::new(
    "2021-01-01T00:00:00Z".to_string(),
    TransactionType::Deposit,
    "TXN123".to_string(),
    "BANKING_SERVICE".to_string(),
)?;
```

**JSON Example:**
```json
{
  "created_at": "2021-01-01T00:00:00Z",
  "kind": "DEPOSIT",
  "reference": "TXN123",
  "source": "BANKING_SERVICE"
}
```

---

### 2. TransactionStatusUpdate (`TRANSACTION_STATUS_UPDATE`)

**Purpose:** Notify that a transaction status has changed

**Payload:** `TransactionStatusEventPayload`

```rust
pub struct TransactionStatusEventPayload {
    pub reference: String,    // Transaction reference
    pub status: String,       // New status
}
```

**Required Fields:** All fields

**Example:**
```rust
let payload = TransactionStatusEventPayload::new(
    "TXN123".to_string(),
    "COMPLETED".to_string(),
)?;
```

**JSON Example:**
```json
{
  "reference": "TXN123",
  "status": "COMPLETED"
}
```

---

### 3. Payment (`PAYMENT`)

**Purpose:** Notify of a payment event (bank or chain payment)

**Payload:** `PaymentEventPayload`

```rust
pub struct PaymentEventPayload {
    pub external_reference: String,   // External payment reference
    pub reference: Option<String>,    // Internal reference (optional)
    pub source: String,               // Payment source
}
```

**Required Fields:** `external_reference`, `source`

**Example:**
```rust
let payload = PaymentEventPayload::new(
    "PAY123".to_string(),
    "BANK_XYZ".to_string(),
    Some("REF123".to_string()),
)?;
```

**JSON Example:**
```json
{
  "external_reference": "PAY123",
  "source": "BANK_XYZ",
  "reference": "REF123"
}
```

---

### 4. NewProfile (`NEW_PROFILE`)

**Purpose:** Notify that a new user profile has been created

**Payload:** `ProfileEventPayload`

```rust
pub struct ProfileEventPayload {
    pub title: String,        // Profile title/name
    pub identifier: String,   // Unique profile identifier
}
```

**Required Fields:** All fields

**Example:**
```rust
let payload = ProfileEventPayload::new(
    "New User".to_string(),
    "USER123".to_string(),
)?;
```

**JSON Example:**
```json
{
  "title": "New User",
  "identifier": "USER123"
}
```

---

### 5. NewUser (`NEW_USER`)

**Purpose:** Notify that a new user account has been created

**Payload:** `NewUserEventPayload`

```rust
pub struct NewUserEventPayload {
    pub title: String,        // User title/name
    pub identifier: String,   // Unique user identifier
}
```

**Required Fields:** All fields

**Example:**
```rust
let payload = NewUserEventPayload::new(
    "User Profile".to_string(),
    "USER456".to_string(),
)?;
```

**JSON Example:**
```json
{
  "title": "New User Registration",
  "identifier": "USER789"
}
```

---

### 6. KycEvent (`KYC_EVENT`)

**Purpose:** Notify of KYC (Know Your Customer) status changes

**Payload:** `KycEventPayload`

```rust
pub struct KycEventPayload {
    pub title: String,
    pub identifier: String,
    pub review_status: Option<String>,
    pub review_result: Option<String>,  // Required if review_status is "completed"
}
```

**Required Fields:** `title`, `identifier`

**Validation Rules:**
- If `review_status` is "completed", then `review_result` must be provided

**Example:**
```rust
let payload = KycEventPayload::new(
    "KYC Review".to_string(),
    "KYC123".to_string(),
    Some("completed".to_string()),
    Some("approved".to_string()),
)?;
```

**JSON Example:**
```json
{
  "title": "KYC Review",
  "identifier": "KYC123",
  "review_status": "completed",
  "review_result": "approved"
}
```

---

### 7. PasswordResetRequested (`PASSWORD_RESET_REQUESTED`)

**Purpose:** Notify that a user has requested a password reset

**Payload:** `PasswordResetEventPayload`

```rust
pub struct PasswordResetEventPayload {
    pub to: String,         // Email address
    pub subject: String,    // Email subject
}
```

**Required Fields:** All fields

**Example:**
```rust
let payload = PasswordResetEventPayload::new(
    "user@example.com".to_string(),
    "Reset Your Password".to_string(),
)?;
```

**JSON Example:**
```json
{
  "to": "user@example.com",
  "subject": "Reset Your Password"
}
```

---

### 8. VerificationRequested (`VERIFICATION_REQUESTED`)

**Purpose:** Notify that email verification has been requested

**Payload:** `VerificationRequestedEventPayload`

```rust
pub struct VerificationRequestedEventPayload {
    pub to: String,         // Email address
    pub subject: String,    // Email subject
}
```

**Required Fields:** All fields

**Example:**
```rust
let payload = VerificationRequestedEventPayload::new(
    "user@example.com".to_string(),
    "Verify Your Email".to_string(),
)?;
```

**JSON Example:**
```json
{
  "to": "user@example.com",
  "subject": "Verify Your Email"
}
```

---

## Payload Variants

All message payloads are defined as variants of the `Payload` enum. The system supports:
- **6 Instruction Payloads**: For commands and operations
- **8 Event Payloads**: For notifications and state changes
- **1 Generic Payload**: Raw string data for custom use cases

### Raw Payload

The `Raw` payload variant allows sending arbitrary string data without strict type validation.

**Purpose:** Send custom or legacy data that doesn't fit into defined payload types

**Features:**
- Bypasses payload type validation
- Still requires valid metadata (instruction_type or event)
- Useful for migration or custom integrations

**Example:**
```rust
let custom_data = r#"{"custom_field": "value", "another_field": 123}"#;
let message = MessageBusMessage::create(
    "CUSTOM_SERVICE".to_string(),
    Payload::Raw(custom_data.to_string()),
    "service.token".to_string(),
    Some(InstructionType::Payment),  // Can use any type
    None,
    None,
)?;
```

---

## Usage Examples

### Creating a Message with the Builder Pattern

```rust
use mykobo_rs::message_bus::models::{
    MessageBusMessage, InstructionType, Payload,
    PaymentPayload,
};

// Create the payload
let payload = PaymentPayload::new(
    "EXT001".to_string(),
    "USD".to_string(),
    "100.00".to_string(),
    "BANK_SOURCE".to_string(),
    "REF001".to_string(),
    Some("John Doe".to_string()),
    None,
)?;

// Create the complete message
let message = MessageBusMessage::create(
    "PAYMENT_SERVICE".to_string(),
    Payload::Payment(payload),
    "service.auth.token".to_string(),
    Some(InstructionType::Payment),
    None,
    Some("custom-idempotency-key".to_string()),
)?;

// Serialize to JSON
let json = serde_json::to_string(&message)?;
```

### Deserializing from JSON String

All payload types support `From<String>` for easy deserialization:

```rust
let json = r#"{
    "external_reference": "P123",
    "currency": "EUR",
    "value": "100.00",
    "source": "BANK",
    "reference": "REF123"
}"#;

// Direct conversion
let payload: PaymentPayload = json.to_string().into();

// Or with error handling
let payload: PaymentPayload = serde_json::from_str(json)?;
```

### Creating Event Messages

```rust
use mykobo_rs::message_bus::models::{
    MessageBusMessage, EventType, Payload,
    NewTransactionEventPayload, TransactionType,
};

let payload = NewTransactionEventPayload::new(
    chrono::Utc::now().to_rfc3339(),
    TransactionType::Deposit,
    "TXN789".to_string(),
    "LEDGER_SERVICE".to_string(),
)?;

let message = MessageBusMessage::create(
    "LEDGER_SERVICE".to_string(),
    Payload::NewTransaction(payload),
    "service.token".to_string(),
    None,
    Some(EventType::NewTransaction),
    None,
)?;
```

### Handling Multiple Message Types

```rust
fn process_message(message: MessageBusMessage) -> Result<(), Box<dyn std::error::Error>> {
    match message.payload {
        Payload::Payment(payload) => {
            println!("Processing payment: {}", payload.reference);
            // Handle payment logic
        },
        Payload::StatusUpdate(payload) => {
            println!("Status update: {} -> {}", payload.reference, payload.status);
            // Handle status update
        },
        Payload::NewTransaction(payload) => {
            println!("New transaction: {}", payload.reference);
            // Handle new transaction event
        },
        Payload::Raw(data) => {
            println!("Raw data: {}", data);
            // Handle custom data
        },
        _ => {
            println!("Unhandled message type");
        }
    }
    Ok(())
}
```

---

## Best Practices

### 1. Use Type-Safe Payloads

Always prefer typed payloads over `Raw` when possible:

```rust
// Good ✓
let payload = PaymentPayload::new(...)?;
let message = MessageBusMessage::create(
    source,
    Payload::Payment(payload),
    token,
    Some(InstructionType::Payment),
    None,
    None,
)?;

// Avoid ✗
let raw_json = r#"{"amount": "100"}"#;
let message = MessageBusMessage::create(
    source,
    Payload::Raw(raw_json.to_string()),
    token,
    Some(InstructionType::Payment),
    None,
    None,
)?;
```

### 2. Provide Idempotency Keys

For critical operations, always provide custom idempotency keys:

```rust
let message = MessageBusMessage::create(
    source,
    payload,
    token,
    instruction_type,
    event_type,
    Some(format!("payment-{}-{}", user_id, timestamp)),  // Custom key
)?;
```

### 3. Validate Before Sending

Use the `validate()` method to catch errors early:

```rust
let message = MessageBusMessage::new(metadata, payload)?;
// Validation happens automatically, will return error if invalid
```

### 4. Handle Deserialization Errors

The `From<String>` implementation will panic on invalid JSON. For production code, use explicit error handling:

```rust
// Better for production
let payload: PaymentPayload = serde_json::from_str(json_str)
    .map_err(|e| format!("Failed to deserialize: {}", e))?;
```

### 5. Match Payload Types to Message Types

The validation system enforces type matching. Don't try to circumvent it:

```rust
// This will fail validation ✗
let payment_payload = PaymentPayload::new(...)?;
let message = MessageBusMessage::create(
    source,
    Payload::Payment(payment_payload),
    token,
    Some(InstructionType::StatusUpdate),  // Wrong type!
    None,
    None,
)?; // Returns ValidationError
```

### 6. Use Meaningful References

Always provide clear, traceable reference IDs:

```rust
// Good ✓
reference: format!("PAY-{}-{}", user_id, timestamp)

// Avoid ✗
reference: "ABC123"
```

### 7. Include Optional Context

When available, include optional fields for better traceability:

```rust
let payload = PaymentPayload::new(
    external_ref,
    currency,
    value,
    source,
    reference,
    Some(payer_name),           // Include when available
    Some(account_number),        // Include when available
)?;
```

---

## Error Handling

### Validation Errors

```rust
use mykobo_rs::message_bus::models::ValidationError;

match MessageBusMessage::create(...) {
    Ok(message) => {
        // Send message
    },
    Err(ValidationError { class_name, fields }) => {
        eprintln!("Validation failed for {}: {:?}", class_name, fields);
    }
}
```

### Common Validation Failures

1. **Missing required fields**: Empty strings in required fields
2. **Type mismatch**: Payload doesn't match instruction/event type
3. **Missing instruction/event**: Neither `instruction_type` nor `event` is set
4. **Both set**: Both `instruction_type` and `event` are set
5. **Conditional requirements**: e.g., `review_result` missing when `review_status` is "completed"

---

## Migration Guide

### From Legacy Metadata to MetaData

If migrating from the deprecated `Metadata` type:

```rust
// Old (deprecated as of v0.0.28)
use mykobo_rs::message_bus::models::Metadata;

// New
use mykobo_rs::message_bus::models::MetaData;
```

### From generate_meta_data() to MetaData::new()

```rust
// Old (deprecated)
let metadata = generate_meta_data(...);

// New
let metadata = MetaData::new(
    source,
    created_at,
    token,
    idempotency_key,
    instruction_type,
    event,
)?;
```

---

## Type Reference

### TransactionType

```rust
pub enum TransactionType {
    Deposit,   // "DEPOSIT"
    Withdraw,  // "WITHDRAW"
}
```

### InstructionType

```rust
pub enum InstructionType {
    Payment,              // "PAYMENT"
    StatusUpdate,         // "STATUS_UPDATE"
    Correction,           // "CORRECTION"
    Transaction,          // "TRANSACTION"
    BankPaymentRequest,   // "BANK_PAYMENT_REQUEST"
    ChainPayment,         // "CHAIN_PAYMENT"
}
```

### EventType

```rust
pub enum EventType {
    NewTransaction,           // "NEW_TRANSACTION"
    TransactionStatusUpdate,  // "TRANSACTION_STATUS_UPDATE"
    Payment,                  // "PAYMENT"
    NewProfile,              // "NEW_PROFILE"
    NewUser,                 // "NEW_USER"
    VerificationRequested,   // "VERIFICATION_REQUESTED"
    PasswordResetRequested,  // "PASSWORD_RESET_REQUESTED"
    KycEvent,                // "KYC_EVENT"
}
```

### Constructor Parameter Order

When using `new()` methods, parameters are ordered as they appear in the struct definition. Common patterns:

**Instruction Payloads:**
- `PaymentPayload::new(external_reference, currency, value, source, reference, payer_name?, bank_account_number?)`
- `StatusUpdatePayload::new(reference, status, message?, transaction_id?)`
- `CorrectionPayload::new(reference, value, message, currency, source)`
- `BankPaymentRequestPayload::new(reference, value, currency, profile_id, message?)`
- `ChainPaymentPayload::new(chain, hash, reference, status, transaction_id?)`

**Event Payloads:**
- `NewTransactionEventPayload::new(created_at, kind, reference, source)`
- `TransactionStatusEventPayload::new(reference, status)`
- `PaymentEventPayload::new(external_reference, source, reference?)`
- `ProfileEventPayload::new(title, identifier)`
- `NewUserEventPayload::new(title, identifier)`
- `KycEventPayload::new(title, identifier, review_status?, review_result?)`
- `PasswordResetEventPayload::new(to, subject)`
- `VerificationRequestedEventPayload::new(to, subject)`

**Note:** `?` indicates optional parameters

---

## Testing

### Unit Testing Messages

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payment_message_creation() {
        let payload = PaymentPayload::new(
            "EXT001".to_string(),
            "USD".to_string(),
            "100.00".to_string(),
            "BANK".to_string(),
            "REF001".to_string(),
            None,
            None,
        ).unwrap();

        let message = MessageBusMessage::create(
            "TEST_SERVICE".to_string(),
            Payload::Payment(payload),
            "test.token".to_string(),
            Some(InstructionType::Payment),
            None,
            None,
        );

        assert!(message.is_ok());
    }
}
```

---

---

## Validation Matrix

The following table shows which payload types are valid for each instruction/event type:

### Instruction Type Validation

| InstructionType | Valid Payload | Required Fields |
|----------------|---------------|-----------------|
| `Payment` | `Payload::Payment(PaymentPayload)` | external_reference, currency, value, source, reference |
| `StatusUpdate` | `Payload::StatusUpdate(StatusUpdatePayload)` | reference, status |
| `Correction` | `Payload::Correction(CorrectionPayload)` | reference, value, message, currency, source |
| `Transaction` | `Payload::Transaction(TransactionPayload)` | All fields (conditional: payer for DEPOSIT, payee for WITHDRAW) |
| `BankPaymentRequest` | `Payload::BankPaymentRequest(BankPaymentRequestPayload)` | reference, value, currency, profile_id |
| `ChainPayment` | `Payload::ChainPayment(ChainPaymentPayload)` | chain, hash, reference, status |

### Event Type Validation

| EventType | Valid Payload | Required Fields |
|-----------|---------------|-----------------|
| `NewTransaction` | `Payload::NewTransaction(NewTransactionEventPayload)` | created_at, kind, reference, source |
| `TransactionStatusUpdate` | `Payload::TransactionStatus(TransactionStatusEventPayload)` | reference, status |
| `Payment` | `Payload::PaymentEvent(PaymentEventPayload)` | external_reference, source |
| `NewProfile` | `Payload::Profile(ProfileEventPayload)` | title, identifier |
| `NewUser` | `Payload::NewUser(NewUserEventPayload)` | title, identifier |
| `KycEvent` | `Payload::Kyc(KycEventPayload)` | title, identifier (review_result required if review_status="completed") |
| `PasswordResetRequested` | `Payload::PasswordReset(PasswordResetEventPayload)` | to, subject |
| `VerificationRequested` | `Payload::VerificationRequested(VerificationRequestedEventPayload)` | to, subject |

**Special Cases:**
- `Raw` payloads bypass type validation and can be used with any instruction or event type
- You cannot specify both an instruction_type and event in the same message
- At least one of instruction_type or event must be specified

---

## Support and Contribution

For issues, questions, or contributions, please refer to the main project repository.

**Version:** 0.1.0
**Last Updated:** 2025-11-16
**Total Message Types:** 6 Instructions + 8 Events = 14 Types
**Total Payload Variants:** 15 (14 typed + 1 raw)
