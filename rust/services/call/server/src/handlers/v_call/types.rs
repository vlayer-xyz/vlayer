use alloy_chains::Chain;
use alloy_primitives::{hex::ToHexExt, keccak256, ChainId, B256, U256};
use alloy_rlp::RlpEncodable;
use alloy_sol_types::SolValue;
use call_engine::{
    evm::env::location::ExecutionLocation, Call as EngineCall, HostOutput, Proof, Seal,
};
use call_host::{encodable_receipt, Call as HostCall, Error as HostError};
use common::Hashable;
use derive_more::From;
use derive_new::new;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use server_utils::{parse_address_field, parse_hex_field};

use crate::error::AppError;

#[derive(Deserialize, Serialize, Debug)]
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

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CallContext {
    #[serde(default = "mainnet_chain_id")]
    pub chain_id: ChainId,
}

pub struct CallResult {
    hash: CallHash,
    proof: Proof,
    evm_call_result: Vec<u8>,
}

impl CallResult {
    pub fn try_new(hash: CallHash, host_output: HostOutput) -> Result<Self, HostError> {
        let HostOutput {
            guest_output,
            seal,
            proof_len,
            call_guest_id,
            ..
        } = host_output;

        let proof = Proof {
            length: U256::from(proof_len),
            seal: decode_seal(seal)?,
            callGuestId: call_guest_id.into(),
            // Intentionally set to 0. These fields will be updated with the correct values by the prover script, based on the verifier ABI.
            callAssumptions: guest_output.call_assumptions,
        };
        Ok(Self {
            hash,
            proof,
            evm_call_result: guest_output.evm_call_result,
        })
    }

    pub fn to_json(&self) -> Value {
        json!({
            "hash": self.hash,
            "evm_call_result": self.evm_call_result.encode_hex_with_prefix(),
            "proof": {
                "seal": {
                    "verifierSelector": self.proof.seal.verifierSelector,
                    "seal": self.proof.seal.seal,
                    "mode": Into::<u8>::into(self.proof.seal.mode),
                },
                "callGuestId": self.proof.callGuestId.encode_hex_with_prefix(),
                "length": u256_to_number(self.proof.length),
                "callAssumptions": {
                    "functionSelector": self.proof.callAssumptions.functionSelector,
                    "proverContractAddress": self.proof.callAssumptions.proverContractAddress,
                    "settleBlockNumber": u256_to_number(self.proof.callAssumptions.settleBlockNumber),
                    "settleBlockHash": self.proof.callAssumptions.settleBlockHash,
                }
            },
        })
    }
}

#[derive(Serialize, Deserialize, Debug, From)]
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

fn decode_seal(seal: Vec<u8>) -> Result<Seal, encodable_receipt::Error> {
    Ok(Seal::abi_decode(&seal, true)?)
}

fn u256_to_number(value: U256) -> u64 {
    u64::try_from(value).expect("Expected value to fit into u64")
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
