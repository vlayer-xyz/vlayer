use alloy_chains::Chain;
use alloy_primitives::{hex::ToHexExt, keccak256, ChainId, B256};
use alloy_rlp::RlpEncodable;
use call_engine::{evm::env::location::ExecutionLocation, Call as EngineCall};
use call_host::Call as HostCall;
use common::Hashable;
use derive_more::From;
use derive_new::new;
use serde::{Deserialize, Serialize};
use server_utils::{parse_address_field, parse_hex_field};

use crate::error::AppError;

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Call {
    pub to: String,
    pub data: String,
    pub gas_limit: u64,
}

impl Call {
    pub fn new(to: impl Into<String>, data: impl Into<String>, gas_limit: u64) -> Self {
        Self {
            to: to.into(),
            data: data.into(),
            gas_limit,
        }
    }
}

impl TryFrom<Call> for HostCall {
    type Error = AppError;

    fn try_from(value: Call) -> Result<Self, Self::Error> {
        Ok(Self {
            to: parse_address_field("to", value.to)?,
            data: parse_hex_field("data", value.data)?,
        })
    }
}

const fn mainnet_chain_id() -> ChainId {
    Chain::mainnet().id()
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CallContext {
    #[serde(default = "mainnet_chain_id")]
    pub chain_id: ChainId,
    pub gas_limit: u64,
}

#[derive(Serialize, Deserialize, Debug, From, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CallHash(B256);

impl std::fmt::Display for CallHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.encode_hex_with_prefix())
    }
}

impl From<(&ExecutionLocation, &EngineCall)> for CallHash {
    fn from((execution_location, call): (&ExecutionLocation, &EngineCall)) -> Self {
        CallHashData::new(execution_location, call)
            .hash_slow()
            .into()
    }
}

#[derive(new, RlpEncodable)]
pub struct CallHashData<'a> {
    execution_location: &'a ExecutionLocation,
    call: &'a EngineCall,
}

impl Hashable for CallHashData<'_> {
    fn hash_slow(&self) -> B256 {
        keccak256(alloy_rlp::encode(self))
    }
}

#[cfg(test)]
mod test {
    use call_host::Call as HostCall;

    use super::Call;
    use crate::error::AppError;

    const TO: &str = "0x7Ad53bbA1004e46dd456316912D55dBc5D311a03";
    const DATA: &str = "0x0000";
    const INVALID_ADDRESS: &str = "0x";

    #[tokio::test]
    async fn invalid_to_address() -> anyhow::Result<()> {
        let call = Call::new(INVALID_ADDRESS, DATA, 0);
        let actual_result: Result<HostCall, AppError> = call.try_into();

        assert!(matches!(
            actual_result,
            Err(AppError::FieldValidation(err)) if err.to_string() == "`to` Invalid string length `0x`"
        ));

        Ok(())
    }

    #[tokio::test]
    async fn invalid_data() -> anyhow::Result<()> {
        const INVALID_DATA: &str = "xx";
        let call = Call::new(TO, INVALID_DATA, 0);
        let actual_result: Result<HostCall, AppError> = call.try_into();

        assert!(matches!(
            actual_result,
            Err(AppError::FieldValidation(err)) if err.to_string() == "`data` Invalid hex prefix `xx`"
        ));

        Ok(())
    }
}
