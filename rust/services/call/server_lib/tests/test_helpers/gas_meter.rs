use std::time::Duration;

use call_server_lib::gas_meter::Config as GasMeterConfig;
use derive_more::{Deref, DerefMut};
use server_utils::rpc::mock::Server as RpcServerMock;

#[derive(Deref, DerefMut)]
pub(crate) struct Server {
    #[deref]
    #[deref_mut]
    mock: RpcServerMock,
    time_to_live: Duration,
    api_key: Option<String>,
}

impl Server {
    pub(crate) async fn start(time_to_live: Duration, api_key: Option<String>) -> Self {
        let mock = RpcServerMock::start().await;
        Self {
            mock,
            time_to_live,
            api_key,
        }
    }

    pub(crate) fn as_gas_meter_config(&self) -> GasMeterConfig {
        GasMeterConfig::new(self.url(), self.time_to_live, self.api_key.clone())
    }
}
