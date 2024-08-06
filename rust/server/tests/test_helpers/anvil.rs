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
use serde_json::json;
use std::{sync::Arc, time::Duration};

abigen!(ExampleProver, "./testdata/ExampleProver.json",);

#[derive(Default)]
pub(crate) struct TestHelper {
    client: Option<Arc<SignerMiddleware<Provider<Http>, Wallet<ecdsa::SigningKey>>>>,
    anvil: Option<AnvilInstance>,
    pub(crate) block_number: u32,
}

pub(crate) async fn test_helper() -> TestHelper {
    let mut test_helper = TestHelper::default();
    test_helper.setup_anvil().await;
    test_helper.deploy_test_contract().await;
    test_helper.set_block_nr().await;
    test_helper
}

impl TestHelper {
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

    async fn set_block_nr(&mut self) {
        let req = json!({
            "jsonrpc": "2.0",
            "method": "eth_blockNumber",
            "params": [],
            "id": 0
        });

        let response = reqwest::Client::new()
            .post(self.anvil().endpoint())
            .json(&req)
            .send()
            .await
            .unwrap();

        let body = response.text().await.unwrap();
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        let result = json["result"].clone();
        let result = result.as_str().unwrap();
        self.block_number = u32::from_str_radix(&result[2..], 16).unwrap();
    }
}
