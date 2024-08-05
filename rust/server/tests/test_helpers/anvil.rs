use ethers::{
    contract::abigen,
    core::utils::{Anvil, AnvilInstance},
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
};
use std::{sync::Arc, time::Duration};

abigen!(ExampleProver, "./testdata/ExampleProver.json",);

pub(crate) async fn setup_anvil() -> AnvilInstance {
    let anvil = Anvil::new().spawn();
    let wallet: LocalWallet = anvil.keys()[0].clone().into();
    let provider = Provider::<Http>::try_from(anvil.endpoint())
        .unwrap()
        .interval(Duration::from_millis(10u64));
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(anvil.chain_id()),
    ));
    ExampleProver::deploy(client.clone(), ())
        .unwrap()
        .send()
        .await
        .unwrap();
    anvil
}
