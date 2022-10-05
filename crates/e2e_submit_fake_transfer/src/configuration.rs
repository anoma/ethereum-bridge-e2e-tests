use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub chain_id: String,
    #[serde(default)]
    pub network_configs_server: Option<String>,
    #[serde(default = "default_ethereum_events_endpoint")]
    pub ethereum_events_endpoint: String,
    #[serde(default = "default_ledger_address")]
    pub ledger_address: String,
}

fn default_ethereum_events_endpoint() -> String {
    "http://localhost:3030/eth_events".to_owned()
}

fn default_ledger_address() -> String {
    "localhost:26657".to_owned()
}
