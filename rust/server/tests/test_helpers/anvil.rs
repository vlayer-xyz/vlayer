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

#[derive(Default)]
pub(crate) struct _TestHelper {
    pub _client: Option<Arc<SignerMiddleware<Provider<Http>, Wallet<ecdsa::SigningKey>>>>,
    anvil: Option<AnvilInstance>,
}

pub(crate) async fn test_helper() -> _TestHelper {
    let mut test_helper = _TestHelper::default();
    test_helper._setup().await;
    test_helper
}

impl _TestHelper {
    pub(crate) async fn _setup(&mut self) {
        let anvil = self.setup_anvil().await;
        self.anvil = Some(anvil);
    }

    pub(crate) fn anvil(&self) -> &AnvilInstance {
        self.anvil.as_ref().unwrap()
    }

    async fn setup_anvil(&mut self) -> AnvilInstance {
        let anvil = Anvil::new().spawn();
        let wallet: LocalWallet = anvil.keys()[0].clone().into();
        let provider = Provider::<Http>::try_from(anvil.endpoint())
            .unwrap()
            .interval(Duration::from_millis(10u64));
        let client = Arc::new(SignerMiddleware::new(
            provider,
            wallet.with_chain_id(anvil.chain_id()),
        ));
        self._client = Some(client.clone());
        ExampleProver::deploy(client, ())
            .unwrap()
            .send()
            .await
            .unwrap();
        anvil
    }
}
