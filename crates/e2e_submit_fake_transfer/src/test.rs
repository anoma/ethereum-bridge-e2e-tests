use crate::utils::TestRunner;
use crate::{configuration, utils::step::StepResult};
use eyre::{eyre, Context, Result};
use namada::types::address::Address;
use namada::types::ethereum_events::EthAddress;
use namada::types::{ethereum_events::testing::DAI_ERC20_ETH_ADDRESS, token::Amount};
use namada_cli::{namadac, namadaw, Output};
use std::time::Duration;

const ETH_BRIDGE_ADDR: &str =
    "atest1v9hx7w36g42ysgzzwf5kgem9ypqkgerjv4ehxgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpq8f99ew";

pub async fn setup(chain_id: &str, network_configs_server: Option<&str>) -> Result<TestRunner> {
    let mut runner = crate::utils::TestRunner::default();

    runner
        .execute_step("Join the network", async {
            match namadac::utils::join_network(chain_id, network_configs_server)
                .await
            {
                Ok(Output { raw, .. }) => Ok(StepResult::succeeded().with_debug_logs(
                    format!("{:#?}", raw),
                )),
                Err(error) => match error {
                    namada_cli::NamadaError::Recognized { reason: namadac::utils::JoinNetworkErrorReason::ChainDirectoryAlreadyExists(output) } => {
                        Ok(StepResult::skipped("chain directory already exists").with_debug_logs(
                            format!("{:#?}", output),
                        ))
                    }
                    namada_cli::NamadaError::Recognized { reason: namadac::utils::JoinNetworkErrorReason::ConnectionRefused(_) } => {
                        // TODO: better retry strategy
                        tracing::info!("Couldn't join network, will attempt again in 10 seconds");
                        tokio::time::sleep(Duration::from_secs(10)).await;
                        match namadac::utils::join_network(chain_id, network_configs_server).await {
                            Ok(Output { raw, .. }) => Ok(StepResult::succeeded().with_debug_logs(
                                format!("{:#?}", raw),
                            )),
                            Err(error) => Err(eyre!(
                                "Failed to join the network, could not connect to download the release: {:#?}",
                                error
                            ))
                        }
                    }
                    namada_cli::NamadaError::Unrecognized {output} => Err(eyre!(
                        "Failed to join the network (unrecognized output) (custom network configs server = {:?}): {:#?}",
                        network_configs_server,
                        output
                    )),
                    namada_cli::NamadaError::Io { source } => Err(eyre!(
                        "Failed to join the network due to IO error (custom network configs server = {:?}): {:#?}",
                        network_configs_server,
                        source
                    )),
                },
            }
        })
        .await?;

    runner.log_info(format!("Joined chain {}", chain_id));
    Ok(runner)
}

pub async fn test(cfg: configuration::Config) -> Result<bool> {
    let mut runner = setup(&cfg.chain_id, cfg.network_configs_server.as_deref()).await?;

    let alias = generate_alias(&mut runner).await?;

    runner.log_info(format!("Generated address for alias {}", alias));

    let receiver = get_address_for_alias(&mut runner, alias).await?;

    let receiver_bech32m = receiver.to_string();

    runner.log_info(format!("Got address {}", receiver_bech32m));

    let events_endpoint_client =
        ethereum_bridge::events_endpoint::Client::new(&cfg.ethereum_events_endpoint);
    let amount = Amount::from(100);

    send_fake_erc20_transfer(
        &mut runner,
        &events_endpoint_client,
        amount,
        DAI_ERC20_ETH_ADDRESS,
        receiver,
    )
    .await?;

    let sleep_time = Duration::from_secs(4);
    runner.log_info(format!(
        "Briefly sleeping {sleep_time:?} while waiting for event to be processed"
    ));
    // TODO: explicitly await a Tendermint event indicating the event was processed, rather than sleeping
    tokio::time::sleep(sleep_time).await;

    assert_wrapped_erc20_balance(
        &mut runner,
        Some(&cfg.ledger_address),
        receiver_bech32m.as_str(),
        &DAI_ERC20_ETH_ADDRESS,
        amount,
    )
    .await?;

    Ok(true)
}

async fn generate_alias(runner: &mut TestRunner) -> Result<String> {
    let alias = runner
        .execute_step_and_return("Generate a new address in the wallet", async {
            match namadaw::address::gen(true).await {
                Ok(Output { raw, parsed }) => Ok(StepResult::succeeded()
                    .with_debug_logs(format!("{:#?}", raw))
                    .returning(parsed)),
                Err(error) => match error {
                    namada_cli::NamadaError::Recognized { reason } => {
                        Ok(StepResult::failed("failed for some reason")
                            .with_debug_logs(format!("{:#?}", reason)))
                    }
                    namada_cli::NamadaError::Unrecognized { output } => Err(eyre!(
                        "Failed to join the network (unrecognized output): {:#?}",
                        output
                    )),
                    namada_cli::NamadaError::Io { source } => Err(eyre!(
                        "Failed to join the network due to IO error: {:#?}",
                        source
                    )),
                },
            }
        })
        .await?
        .unwrap();
    Ok(alias)
}

async fn get_address_for_alias(runner: &mut TestRunner, alias: impl AsRef<str>) -> Result<Address> {
    let addr = runner
        .execute_step_and_return(
            format!("Get the address for alias {}", alias.as_ref()),
            async {
                match namadaw::address::find(alias.as_ref()).await {
                    Ok(Output { raw, parsed }) => Ok(StepResult::succeeded()
                        .with_debug_logs(format!("{:#?}", raw))
                        .returning(parsed)),
                    Err(error) => match error {
                        namada_cli::NamadaError::Recognized { reason } => {
                            Ok(StepResult::failed("failed for some reason")
                                .with_debug_logs(format!("{:#?}", reason)))
                        }
                        namada_cli::NamadaError::Unrecognized { output } => Err(eyre!(
                            "Failed to join the network (unrecognized output): {:#?}",
                            output
                        )),
                        namada_cli::NamadaError::Io { source } => Err(eyre!(
                            "Failed to join the network due to IO error: {:#?}",
                            source
                        )),
                    },
                }
            },
        )
        .await?
        .unwrap();
    Ok(addr)
}

async fn send_fake_erc20_transfer(
    runner: &mut TestRunner,
    client: &ethereum_bridge::events_endpoint::Client,
    amount: Amount,
    asset: EthAddress,
    receiver: Address,
) -> Result<()> {
    runner
        .execute_step(
            format!("Transfer some wDAI ({:?}) to the receiver", &amount),
            async {
                client
                    .send_fake_transfer_to_namada(amount, asset, receiver, None)
                    .await
                    .map(|response| {
                        StepResult::succeeded().with_debug_logs(format!("{:#?}", response))
                    })
                    .wrap_err_with(|| "Failed to send the transfer")
            },
        )
        .await
}

async fn assert_wrapped_erc20_balance(
    runner: &mut TestRunner,
    ledger_address: Option<&str>,
    owner_bech32m: &str,
    erc20: &EthAddress,
    expected: Amount,
) -> Result<()> {
    runner
        .execute_step(
            format!("Check the wDAI balance of the receiver is {:?}", &expected),
            async {
                match namadac::balance_of_multitoken_token_for_owner(
                    ledger_address,
                    owner_bech32m,
                    ETH_BRIDGE_ADDR,
                    &format!("ERC20/{}", erc20.to_canonical()),
                )
                .await
                {
                    Ok(Output { raw, parsed }) => {
                        if parsed == expected {
                            Ok(StepResult::succeeded().with_debug_logs(format!("{:#?}", raw)))
                        } else {
                            Ok(StepResult::failed(format!(
                                "got amount {:?} but expected amount {:?}",
                                parsed, expected
                            ))
                            .with_debug_logs(format!("{:#?}", raw)))
                        }
                    }
                    Err(error) => match error {
                        namada_cli::NamadaError::Recognized {
                            reason: namadac::BalanceErrorReason::UnknownTokenAddress(output),
                        } => Ok(StepResult::failed("Token address wasn't recognized")
                            .with_debug_logs(format!("{:#?}", output))),
                        // TODO: retry for a bit here, maybe the event hasn't been processed yet
                        namada_cli::NamadaError::Recognized {
                            reason: namadac::BalanceErrorReason::NoBalanceFound(output),
                        } => Ok(StepResult::failed("No balance found")
                            .with_debug_logs(format!("{:#?}", output))),
                        namada_cli::NamadaError::Unrecognized { output } => Err(eyre!(
                            "Failed to join the network (unrecognized output): {:#?}",
                            output
                        )),
                        namada_cli::NamadaError::Io { source } => Err(eyre!(
                            "Failed to join the network due to IO error: {:#?}",
                            source
                        )),
                    },
                }
            },
        )
        .await
}
