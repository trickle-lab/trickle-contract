# Trickle Contract

Soroban smart contract powering Trickle — a streaming payments protocol on Stellar. Instead of paying in a lump sum, senders lock funds that vest continuously to a recipient over a set duration.

## Functions
- `create_stream(sender, recipient, token, amount, duration)` — locks funds, starts the stream
- `balance(stream_id)` — amount currently available to withdraw
- `withdraw(stream_id)` — recipient claims vested funds
- `cancel(stream_id)` — sender cancels early; recipient keeps vested amount, sender gets the rest back

Every state-changing call emits an event so off-chain systems (like [trickle-backend](https://github.com/trickle-lab/trickle-backend)) can track activity without polling storage directly.

## Status
Deployed and tested live on Stellar testnet. Core logic (create/withdraw/cancel/balance) is working end-to-end. `cargo test` is currently blocked by an upstream `soroban-env-host` dependency conflict — see open issues.

## Setup
See [CONTRIBUTING.md](./CONTRIBUTING.md).

## Part of Trickle
- [trickle-contract](https://github.com/trickle-lab/trickle-contract) (this repo)
- [trickle-backend](https://github.com/trickle-lab/trickle-backend)
- [trickle-frontend](https://github.com/trickle-lab/trickle-frontend)

## License
Apache License 2.0