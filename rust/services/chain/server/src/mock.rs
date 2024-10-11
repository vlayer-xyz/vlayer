use httpmock::Mock;
use serde::Serialize;
use server_utils::RpcServerMock;

pub struct ChainProofServerMock {
    mock_server: RpcServerMock,
}

impl ChainProofServerMock {
    pub async fn start() -> Self {
        let mock_server = RpcServerMock::start("chain_proof").await;

        ChainProofServerMock { mock_server }
    }

    pub fn url(&self) -> String {
        self.mock_server.url()
    }

    pub async fn mock(&self, params: impl Serialize, result: impl Serialize) -> Mock {
        self.mock_server.mock(true, params, result).await
    }
}
