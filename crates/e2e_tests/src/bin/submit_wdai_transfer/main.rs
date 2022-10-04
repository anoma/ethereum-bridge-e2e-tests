use test_runner::chain;
use test_runner::client;
use test_runner::env;
use test_runner::wallet;

mod test;

const TENDERMINT_RPC_ENV_VAR: &str = "ANOMA_LEDGER_ADDRESS";

#[tokio::main]
async fn main() {
    chain::join();

    let receiver_implicit_alias = wallet::random_alias("receiver-implicit");
    wallet::gen_address_or_die(&receiver_implicit_alias);

    let ledger_address = env::get_var_or_die(TENDERMINT_RPC_ENV_VAR);
    let client = client::Client::new(&ledger_address);

    client.get_xan_from_faucet(&receiver_implicit_alias);

    match test::run(&client).await {
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
