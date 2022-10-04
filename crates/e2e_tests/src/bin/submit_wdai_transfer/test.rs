use eyre::Result;

use namada::types::{
    address::Address, ethereum_events::testing::DAI_ERC20_ETH_ADDRESS, token::Amount,
};
use test_runner::{client, ethereum_events_endpoint};

pub(crate) async fn run(_client: &client::Client) -> Result<bool> {
    let amount = Amount::from(100);
    // this receiver is a random Namada address
    let receiver = Address::decode(
        "atest1v4ehgw36x5myzdfcg5myg3feg4p5vvzyxfrrgdpnxccnvvp5gdrrqse3gdz5zv2yxsenqs3ntgy89n",
    )
    .expect("Receiver address must be decodeable!");
    let asset = DAI_ERC20_ETH_ADDRESS;

    ethereum_events_endpoint::send_fake_transfer_to_namada(amount, receiver, asset, None).await?;

    Ok(true)
}
