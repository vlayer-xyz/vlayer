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
    pub gas_meter_user_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, From, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CallHash(B256);

impl std::fmt::Display for CallHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.encode_hex_with_prefix())
    }
}

#[derive(new, RlpEncodable)]
pub struct CallHashData {
    execution_location: ExecutionLocation,
    call: EngineCall,
}

impl Hashable for CallHashData {
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
        let call = Call {
            to: INVALID_ADDRESS.to_string(),
            data: DATA.to_string(),
        };
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
        let call = Call {
            to: TO.to_string(),
            data: INVALID_DATA.to_string(),
        };
        let actual_result: Result<HostCall, AppError> = call.try_into();

        assert!(matches!(
            actual_result,
            Err(AppError::FieldValidation(err)) if err.to_string() == "`data` Invalid hex prefix `xx`"
        ));

        Ok(())
    }
}
