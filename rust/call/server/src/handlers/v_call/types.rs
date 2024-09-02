use crate::error::AppError;
use alloy_chains::Chain;
use alloy_primitives::hex::ToHexExt;
use alloy_primitives::{Address, ChainId, FixedBytes};
use call_engine::io::HostOutput;
use call_host::Call as HostCall;
use serde::{Deserialize, Serialize};
use server_utils::{parse_address_field, parse_hex_field};

const SELECTOR_LEN: usize = 4;
const HASH_LEN: usize = 32;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Call {
    to: String,
    data: String,
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

fn mainnet_chain_id() -> ChainId {
    Chain::mainnet().id()
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CallContext {
    #[serde(default = "mainnet_chain_id")]
    pub chain_id: ChainId,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CallResult {
    evm_call_result: String,
    function_selector: FixedBytes<SELECTOR_LEN>,
    prover_contract_address: Address,
    seal: String,
    block_no: u64,
    block_hash: FixedBytes<HASH_LEN>,
}

impl From<HostOutput> for CallResult {
    fn from(host_output: HostOutput) -> Self {
        Self {
            evm_call_result: host_output
                .guest_output
                .evm_call_result
                .encode_hex_with_prefix(),
            function_selector: host_output
                .guest_output
                .execution_commitment
                .functionSelector,
            prover_contract_address: host_output
                .guest_output
                .execution_commitment
                .proverContractAddress,
            seal: host_output.seal.encode_hex_with_prefix(),
            block_no: u64::try_from(
                host_output
                    .guest_output
                    .execution_commitment
                    .settleBlockNumber,
            )
            .unwrap(),
            block_hash: host_output
                .guest_output
                .execution_commitment
                .settleBlockHash,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Call;
    use crate::error::AppError;
    use call_host::Call as HostCall;

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
