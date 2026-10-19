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
cd contracts/trickle-stream
cargo test

Keep Cargo.lock committed. It records a known-good set of dependency versions: the test utilities in
soroban-env-host have broken before on newer ed25519-dalek and curve25519-dalek releases. If `cargo test`
stops compiling after a `cargo update`, restore Cargo.lock from git.


## Pull Requests
- Branch from main
- Keep PRs focused on a single change
- Add or update tests for contract changes and make sure `cargo test` passes
