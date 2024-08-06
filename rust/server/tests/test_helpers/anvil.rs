use ethers::{
    contract::abigen,
    core::{
        k256::ecdsa,
        utils::{Anvil, AnvilInstance},
    },
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer, Wallet},
};
use std::{sync::Arc, time::Duration};

abigen!(ExampleProver, "./testdata/ExampleProver.json",);

pub(crate) struct _TestHelper {
    pub _client: Arc<SignerMiddleware<Provider<Http>, Wallet<ecdsa::SigningKey>>>,
    pub anvil: AnvilInstance,
}

pub(crate) async fn _test_helper() -> _TestHelper {
    let (_client, anvil) = setup_anvil().await;
    let test_helper = _TestHelper { _client, anvil };
    test_helper
}

async fn setup_anvil() -> (
    Arc<SignerMiddleware<Provider<Http>, Wallet<ecdsa::SigningKey>>>,
    AnvilInstance,
) {
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
    (client, anvil)
}
