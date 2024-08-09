use async_trait::async_trait;
use serde::Deserialize;

#[async_trait]
pub trait JsonRpcHandler {
    type Params: for<'de> Deserialize<'de>;
    type Config;
    type Output;
    type Error;
    async fn call(params: Self::Params, config: Self::Config) -> Result<Self::Output, Self::Error>;
}

pub mod v_call;
