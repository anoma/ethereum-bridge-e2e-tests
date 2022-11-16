# e2e_submit_fake_transfer

Tests that a Namada chain processes a fake wDAI transfer.

## Prerequisites

- `namada` binaries on `$PATH` (e.g. `namadac`, `namadaw`)
- a live Namada chain with a reachable fullnode that can be used for submitting transactions, that also has the fake Ethereum events endpoint enabled
- the network config for the chain is published somewhere so that the chain can be joined using `namadac utils join-network`
- a `config.toml` file, which may look something like the following for a locally running ledger (and with a network configs server running at `http://localhost:8123`)

```toml
chain_id = "dev.70637df1cdce6f442a8f501274"
network_configs_server = "http://localhost:8123"
ledger_address = "localhost:26657"
ethereum_events_endpoint = "http://localhost:3030/eth_events"
```

## Running

Using a built `e2e_submit_fake_transfer` binary:

```shell
e2e_submit_fake_transfer --config config.toml
```

Or, if running from withing the Cargo workspace:

```shell
cargo run --bin e2e_submit_fake_transfer -- --config config.toml
```
