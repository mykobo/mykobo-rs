# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`mykobo-rs` is a Rust client library for interacting with the MYKOBO suite of services. It provides HTTP clients for various MYKOBO services and supports message bus communication via both Kafka and AWS SQS.

## Build and Test Commands

### Build
```bash
cargo build              # Build the library
cargo check             # Fast compilation check without producing binary
```

### Testing
```bash
cargo test              # Run all tests
cargo test <test_name>  # Run specific test containing <test_name>
cargo test --package mykobo-rs  # Run tests for specific package
```

### Running a Single Test
```bash
# Run a specific test by its full name or partial match
cargo test test_customer_requests
cargo test identity::  # Run all tests in identity module
```

## Architecture

### Core Service Clients

The library is organized around service-specific HTTP clients that handle authentication and API communication:

- **IdentityServiceClient** (`src/identity/mod.rs`): Manages user profiles, KYC status, authentication tokens, and authorization checks. Handles automatic token acquisition and retry logic.

- **WalletServiceClient** (`src/wallets/mod.rs`): Handles wallet profile retrieval and wallet registration for users.

- **SumSubClient** (`src/sumsub/mod.rs`): Integrates with SumSub for KYC verification.

- **AnchorClients** (`src/anchor/`):
  - Stellar anchor integration (`stellar/mod.rs`)
  - Dapp anchor integration (`dapp/mod.rs`)

### Message Bus Architecture

The message bus module (`src/message_bus/`) supports two backends with a unified message model:

**Kafka** (`message_bus/kafka/`):
- `consumer.rs`: EventConsumer with automatic retry, backoff, and channel-based message forwarding
- `producer.rs`: EventProducer with SASL_SSL authentication and compression
- Both use KAFKA_API_KEY and KAFKA_API_SECRET environment variables

**SQS** (`message_bus/sqs/`):
- `mod.rs`: Functions for send_message, receive, delete_message, and client configuration
- Uses AWS SDK with QUEUE_ENDPOINT and AWS_REGION environment variables

**Unified Message Models** (`message_bus/models/`):
- `MessageBusMessage`: Core message structure with metadata and payload
- `EventType` and `InstructionType`: Enums for message classification
- Legacy `Metadata` type deprecated as of v0.0.28 (use `MetaData` instead)

### Model Organization

Models are split between request and response types in `src/models/`:
- `request/`: Request DTOs for identity, wallets, sumsub
- `response/`: Response DTOs including auth, identity, wallets, anchor (stellar/dapp)
- `error.rs`: ServiceError and message bus error types (KafkaError, SQSError)

### Authentication Pattern

Most service clients follow this pattern:
1. Store credentials and service token
2. Check if token is valid before requests
3. `attempt_token_acquisition()` handles token refresh with configurable retries
4. Requests use `generate_headers()` utility with optional service token and client identifier

### Testing Infrastructure

Tests are in `tests/` directory mirroring the src structure. Test files use:
- `wiremock` for HTTP mocking
- `fake` for test data generation
- `pretty_assertions` for better assertion output
- Helper `read_file()` function in `tests/lib.rs` for loading test fixtures

## Environment Variables

Required environment variables by service:
- Identity: `IDENTITY_SERVICE_HOST`, `IDENTITY_ACCESS_KEY`, `IDENTITY_SECRET_KEY`
- Wallets: `WALLET_HOST`
- SQS: `QUEUE_ENDPOINT`, `AWS_REGION` (optional, defaults to eu-west-1)
- Kafka: `KAFKA_API_KEY`, `KAFKA_API_SECRET`

## Key Patterns

### Service Client Instantiation
All service clients use `new(max_retries: i8)` and read configuration from environment variables.

### Error Handling
All service operations return `Result<T, ServiceError>` or `Result<T, KafkaError/SQSError>`. Use the `parse_response` utility for consistent HTTP response parsing.

### Message Bus Migration
When working with message bus code, note that as of v0.0.28:
- Use `models::MetaData` instead of deprecated `Metadata`
- Use `MessageBusMessage::create()` for new message construction
- Legacy `generate_meta_data()` function is deprecated
