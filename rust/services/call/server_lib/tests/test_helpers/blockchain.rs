use std::{sync::Arc, time::Duration};

use derive_more::Deref;
use ethers::{
    contract::abigen,
    core::{
        k256::ecdsa,
        utils::{self, AnvilInstance},
    },
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer, Wallet},
};

abigen!(ExampleProver, "./testdata/ExampleProver.json");

pub(crate) type Contract =
    ExampleProver<SignerMiddleware<Provider<Http>, Wallet<ecdsa::SigningKey>>>;

#[derive(Deref)]
pub(crate) struct Client(Arc<SignerMiddleware<Provider<Http>, Wallet<ecdsa::SigningKey>>>);

impl Client {
    pub(crate) fn new(provider: Provider<Http>, wallet: Wallet<ecdsa::SigningKey>) -> Self {
        Client(Arc::new(SignerMiddleware::new(provider, wallet)))
    }

    pub(crate) async fn deploy_contract(&self) -> Contract {
        ExampleProver::deploy(self.0.clone(), ())
            .unwrap()
            .send()
            .await
            .unwrap()
    }
}

#[derive(Deref)]
pub(crate) struct Anvil(AnvilInstance);

impl Anvil {
    pub(crate) fn start() -> Self {
        Self(utils::Anvil::new().chain_id(11_155_111_u64).spawn())
    }

    pub(crate) fn setup_client(&self) -> Client {
        let wallet: LocalWallet = self.keys()[0].clone().into();
        let provider = Provider::<Http>::try_from(self.endpoint())
            .unwrap()
            .interval(Duration::from_millis(10_u64));
        Client::new(provider, wallet.with_chain_id(self.chain_id()))
    }
}
