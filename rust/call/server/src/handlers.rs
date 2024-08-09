use std::error::Error;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

#[async_trait]
pub trait JsonRpcHandler {
    type Params: DeserializeOwned;
    type Config;
    async fn call(params: Self::Params, config: Self::Config)
        -> Result<impl Serialize, impl Error>;
}

pub mod v_call;
