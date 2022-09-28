use eyre::Result;

use test_runner::client;

pub(crate) fn run(
    client: &client::Client,
    tx_sender_alias: &str,
    tx_modify_erc20_supply_path: &str,
) -> Result<bool> {
    client.tx(tx_modify_erc20_supply_path, tx_sender_alias, None);

    // TODO: check `namadac` output indicates that the tx was rejected by the #EthBridge VP
    // TODO: check the wDAI supply key wasn't modified using client.query_bytes()

    Ok(true)
}
