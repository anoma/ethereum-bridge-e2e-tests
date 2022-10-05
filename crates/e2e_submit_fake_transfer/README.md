# e2e_submit_fake_transfer

Use the `RUST_LOG` environment variable to set the log level.

This test should be able to be run against any Namada chain, so long as everything is configured correctly. e.g.

```shell
./e2e_submit_fake_transfer --config config.toml
```

where `config.toml` may look something like the following for a locally running ledger

```toml
chain_id = "dev.70637df1cdce6f442a8f501274"
network_configs_server = "http://localhost:8123"
ledger_address = "localhost:26657"
ethereum_events_endpoint = "http://localhost:3030/eth_events"
```
