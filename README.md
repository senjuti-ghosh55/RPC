# Stellar Legacy Bridge Connector

A robust Spring Boot backend application designed to act as a bridge, allowing older enterprise banking systems to interface seamlessly with the Stellar network.

## Architecture

This project is composed of two primary layers:
1. **Spring Boot Backend**: Provides REST APIs to legacy systems, ensuring idempotency using JPA and network resilience using Spring Retry.
2. **Soroban Smart Contract**: Located in the `smart-contract/` directory, it provides on-chain logic to map legacy banking identifiers to native Stellar addresses.

## Stellar & Soroban Integration

The project integrates with the Stellar network utilizing the `java-stellar-sdk`. Furthermore, a Rust-based Soroban smart contract is included to handle identity mappings on-chain. 

### Smart Contract Structure
The `smart-contract` directory contains the Soroban smart contract source code natively written in Rust.

- `smart-contract/Cargo.toml`: Package configuration and dependencies (`soroban-sdk`).
- `smart-contract/src/lib.rs`: The on-chain registry contract that permits mapping legacy bank IDs (Strings) to Stellar Addresses securely.

### Building the Contract
To compile the smart contract into a WebAssembly (.wasm) file:

```bash
cd smart-contract
cargo build --target wasm32-unknown-unknown --release
```

## Running the Application
Ensure you have Java 17 and Maven installed, then run the Spring Boot application using:
```bash
mvn spring-boot:run
```
