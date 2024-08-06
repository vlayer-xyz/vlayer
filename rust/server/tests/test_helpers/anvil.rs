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
    client: Option<Arc<SignerMiddleware<Provider<Http>, Wallet<ecdsa::SigningKey>>>>,
    anvil: Option<AnvilInstance>,
}

pub(crate) async fn test_helper() -> _TestHelper {
    let mut test_helper = _TestHelper::default();
    test_helper.setup_anvil().await;
    test_helper.deploy_test_contract().await;
    test_helper
}

impl _TestHelper {
    pub(crate) fn anvil(&self) -> &AnvilInstance {
        self.anvil.as_ref().unwrap()
    }

    fn client(&self) -> Arc<SignerMiddleware<Provider<Http>, Wallet<ecdsa::SigningKey>>> {
        self.client.as_ref().unwrap().clone()
    }

    async fn setup_anvil(&mut self) {
        self.anvil = Some(Anvil::new().spawn());
        let wallet: LocalWallet = self.anvil().keys()[0].clone().into();
        let provider = Provider::<Http>::try_from(self.anvil().endpoint())
            .unwrap()
            .interval(Duration::from_millis(10u64));
        let client = Arc::new(SignerMiddleware::new(
            provider,
            wallet.with_chain_id(self.anvil().chain_id()),
        ));
        self.client = Some(client.clone());
    }

    async fn deploy_test_contract(&self) {
        ExampleProver::deploy(self.client(), ())
            .unwrap()
            .send()
            .await
            .unwrap();
    }
}
