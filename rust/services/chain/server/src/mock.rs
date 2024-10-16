use serde::Serialize;
use server_utils::RpcServerMock;

pub struct ChainProofServerMock {
    mock_server: RpcServerMock,
}

impl ChainProofServerMock {
    pub async fn start(params: impl Serialize, result: impl Serialize) -> Self {
        let mock_server = RpcServerMock::start("v_chain", true, params, result).await;

        ChainProofServerMock { mock_server }
    }

    pub fn url(&self) -> String {
        self.mock_server.url()
    }
}
