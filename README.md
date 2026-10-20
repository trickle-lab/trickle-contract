# Trickle Contract

Soroban smart contract powering Trickle — a streaming payments protocol on Stellar. Instead of paying in a lump sum, senders lock funds that vest continuously to a recipient over a set duration.

## Functions
- `create_stream(sender, recipient, token, amount, duration)` — locks funds, starts the stream
- `balance(stream_id)` — amount currently available to withdraw
- `withdraw(stream_id)` — recipient claims vested funds
- `cancel(stream_id)` — sender cancels early; recipient keeps vested amount, sender gets the rest back

Every state-changing call emits an event so off-chain systems (like [trickle-backend](https://github.com/trickle-lab/trickle-backend)) can track activity without polling storage directly.

## Status
Deployed and tested live on Stellar testnet. Core logic (create/withdraw/cancel/balance) works end to end, and the unit tests run in CI with `cargo test`.

## Setup
See [CONTRIBUTING.md](./CONTRIBUTING.md).

![Trickle architecture](docs/architecture.png)

## Part of Trickle
- [trickle-contract](https://github.com/trickle-lab/trickle-contract) (this repo)
- [trickle-backend](https://github.com/trickle-lab/trickle-backend)
- [trickle-frontend](https://github.com/trickle-lab/trickle-frontend)

## License
Apache License 2.0
## Live on Stellar testnet

- Contract: [`CBM2KGHZZOHJ44GSHJWHY4YKTKOH3VUBBWZCXIG2NT2ONGBP6ANJMPGR`](https://stellar.expert/explorer/testnet/contract/CBM2KGHZZOHJ44GSHJWHY4YKTKOH3VUBBWZCXIG2NT2ONGBP6ANJMPGR)
- Example `create_stream`: [transaction](https://stellar.expert/explorer/testnet/tx/de875169007aaf2ef6c9658d40a1028b8abb3936a8a85580139608e52afb7172)
- Example `withdraw`: [transaction](https://stellar.expert/explorer/testnet/tx/20a3c054ff0ea38cd1f1b46106ea4b2d840bd7a3b935aff52ef3ccd69bc031d3)
- Example `cancel`: [transaction](https://stellar.expert/explorer/testnet/tx/349c982c38cf8e83f0b339b3975d81a8463b2bffdb946ebb14003e4c3b3898e0)
