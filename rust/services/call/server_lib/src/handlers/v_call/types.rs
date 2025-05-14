use alloy_chains::Chain;
use alloy_primitives::{B256, ChainId, hex::ToHexExt, keccak256};
use alloy_rlp::RlpEncodable;
use call_common::ExecutionLocation;
use call_engine::Call as EngineCall;
use call_host::{BuilderError, Call as HostCall};
use common::Hashable;
use derive_more::From;
use derive_new::new;
use jsonrpsee::types::error::{self as jrpcerror, ErrorObjectOwned};
use serde::{Deserialize, Serialize};
use server_utils::{FieldValidationError, parse_address_field, parse_hex_field};

use crate::gas_meter::Error as GasMeterError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid field: {0}")]
    FieldValidation(#[from] FieldValidationError),
    #[error("Gas meter: {0}")]
    GasMeter(#[from] GasMeterError),
    #[error("Host builder: {0}")]
    HostBuilder(#[from] BuilderError),
}

impl From<Error> for ErrorObjectOwned {
    fn from(error: Error) -> Self {
        match error {
            Error::FieldValidation(..) => ErrorObjectOwned::owned::<()>(
                jrpcerror::INVALID_PARAMS_CODE,
                error.to_string(),
                None,
            ),
            Error::HostBuilder(..) | Error::GasMeter(..) => ErrorObjectOwned::owned::<()>(
                jrpcerror::INTERNAL_ERROR_CODE,
                error.to_string(),
                None,
            ),
        }
    }
}

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

    pub fn parse_and_validate(self, max_calldata_size: usize) -> Result<HostCall> {
        let call = HostCall {
            to: parse_address_field("to", self.to)?,
            data: parse_hex_field("data", self.data)?,
            gas_limit: self.gas_limit,
        };

        if call.data.len() > max_calldata_size {
            return Err(FieldValidationError::LengthLimit {
                field: "data".to_string(),
                length: call.data.len(),
                limit: max_calldata_size,
            }
            .into());
        }

        Ok(call)
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

#[derive(new, RlpEncodable, Debug)]
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
    use super::*;

    const TO: &str = "0x7Ad53bbA1004e46dd456316912D55dBc5D311a03";
    const DATA: &str = "0x0000";
    const INVALID_ADDRESS: &str = "0x";
    const MAX_CALLDATA_SIZE: usize = 2;

    #[tokio::test]
    async fn invalid_to_address() -> anyhow::Result<()> {
        let call = Call::new(INVALID_ADDRESS, DATA, 0);
        let actual_result = call.parse_and_validate(MAX_CALLDATA_SIZE);

        assert!(matches!(
            actual_result,
            Err(Error::FieldValidation(err)) if err.to_string() == "`to` Invalid string length `0x`"
        ));

        Ok(())
    }

    #[tokio::test]
    async fn invalid_data() -> anyhow::Result<()> {
        const INVALID_DATA: &str = "xx";
        let call = Call::new(TO, INVALID_DATA, 0);
        let actual_result = call.parse_and_validate(MAX_CALLDATA_SIZE);

        assert!(matches!(
            actual_result,
            Err(Error::FieldValidation(err)) if err.to_string() == "`data` Invalid hex prefix `xx`"
        ));

        Ok(())
    }

    #[tokio::test]
    async fn calldata_length_limit() -> anyhow::Result<()> {
        const LONG_DATA: &str = "0x00";
        let call = Call::new(TO, LONG_DATA, 0);
        let actual_result = call.parse_and_validate(0);

        assert!(matches!(
            actual_result,
            Err(Error::FieldValidation(err)) if err.to_string() == "`data` is too long `1` > `0`"
        ));

        Ok(())
    }
}
