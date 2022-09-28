use test_runner::chain;
use test_runner::client;
use test_runner::env;
use test_runner::wallet;

mod test;

const TENDERMINT_RPC_ENV_VAR: &str = "ANOMA_LEDGER_ADDRESS";

fn main() {
    chain::join();

    let tx_sender_implicit_alias = wallet::random_alias("tx-sender-implicit");

    let ledger_address = env::get_var_or_die(TENDERMINT_RPC_ENV_VAR);
    let current_dir = std::env::current_dir().unwrap();

    let tx_modify_erc20_supply_path = format!(
        "{}/wasm/tx_modify_erc20_supply.wasm",
        current_dir.to_string_lossy()
    );

    let client = client::Client::new(&ledger_address);

    wallet::gen_address_or_die(&tx_sender_implicit_alias);
    client.get_xan_from_faucet(&tx_sender_implicit_alias);

    match test::run(
        &client,
        &tx_sender_implicit_alias,
        &tx_modify_erc20_supply_path,
    ) {
        Ok(passed) => {
            println!("Test passed!");
            if passed {
                std::process::exit(0);
            } else {
                std::process::exit(2);
            }
        }
        Err(err) => {
            eprintln!("Error while running test: {:?}", err);
            std::process::exit(1)
        }
    };
}
