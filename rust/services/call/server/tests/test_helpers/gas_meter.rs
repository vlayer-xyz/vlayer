use call_server::gas_meter::AllocateGas;
use derive_more::Deref;
use serde::Serialize;
use server_utils::{RpcMethod, RpcServerMock};

#[derive(Deref)]
pub struct ServerMock(RpcServerMock);

impl ServerMock {
    pub async fn start(params: impl Serialize, result: impl Serialize) -> ServerMock {
        let mock_server =
            RpcServerMock::start(AllocateGas::METHOD_NAME, true, params, result).await;
        Self(mock_server)
    }
}
