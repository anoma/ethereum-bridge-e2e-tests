use eyre::Result;
use namada::ledger::eth_bridge::storage::wrapped_erc20s;
use namada::types::ethereum_events::EthAddress;
use namada_tx_prelude::*;

const TX_NAME: &str = "tx_modify_erc20_supply";

fn log(msg: &str) {
    log_string(format!("[{}] {}", TX_NAME, msg))
}

pub const DAI_ERC20_ETH_ADDRESS: EthAddress = EthAddress([
    107, 23, 84, 116, 232, 144, 148, 196, 77, 169, 139, 149, 78, 237, 234, 196, 149, 39, 29, 15,
]);

fn wdai_supply_key() -> Key {
    let wdai = wrapped_erc20s::Keys::from(&DAI_ERC20_ETH_ADDRESS);
    wdai.supply()
}

#[transaction]
fn apply_tx(tx_data: Vec<u8>) {
    if let Err(err) = apply_tx_aux(tx_data) {
        log(&format!("ERROR: {:?}", err));
        panic!("{:?}", err)
    }
}

fn apply_tx_aux(tx_data: Vec<u8>) -> Result<()> {
    log_string(format!("apply_tx called with data: {:#?}", tx_data));

    // attempt to write garbage to the wDAI supply key
    write(wdai_supply_key().to_string(), "blah");

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    use namada_tests::tx::*;

    #[test]
    fn test_attempts_to_write_to_supply_key() {
        // The environment must be initialized first
        tx_host_env::init();

        let tx_data = vec![];
        apply_tx(tx_data);

        let env = tx_host_env::take();
        assert_eq!(
            env.all_touched_storage_keys(),
            BTreeSet::from_iter(vec![wdai_supply_key().into()])
        );
    }
}
