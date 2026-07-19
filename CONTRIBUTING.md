# Contributing to Trickle Contracts

## Setup
1. Install Rust and the wasm32v1-none target:
   rustup target add wasm32v1-none
2. Install stellar-cli:
   cargo install --locked stellar-cli

## Building
cd contracts/trickle-stream
cargo build --target wasm32v1-none --release

## Testing
cargo test is currently blocked by an upstream bug in soroban-env-host's test utilities
(unrelated to this contract's code). Until resolved, verify changes by deploying to testnet
and invoking functions directly:
stellar contract deploy --wasm target/wasm32v1-none/release/trickle_stream.wasm --source <your-identity> --network testnet

## Pull Requests
- Branch from main
- Keep PRs focused on a single change
- Describe what you tested and how, since automated tests are currently limited