use alloy_primitives::{Address, B256, Selector, U256};
use call_engine::{CallAssumptions, Proof, ProofMode, Seal};
use serde::{Serialize, Serializer};

#[derive(Serialize)]
#[serde(remote = "Seal")]
#[allow(non_snake_case)]
pub struct SealDTO {
    verifierSelector: Selector,
    seal: [B256; 8],
    #[serde(serialize_with = "ser_proof_mode")]
    mode: ProofMode,
}

#[derive(Serialize)]
#[serde(remote = "CallAssumptions")]
#[allow(non_snake_case)]
pub struct CallAssumptionsDTO {
    proverContractAddress: Address,
    functionSelector: Selector,
    settleChainId: U256,
    settleBlockNumber: U256,
    settleBlockHash: B256,
}

#[derive(Serialize)]
#[serde(remote = "Proof")]
#[allow(non_snake_case)]
pub struct ProofDTO {
    #[serde(with = "SealDTO")]
    seal: Seal,
    callGuestId: B256,
    #[serde(serialize_with = "ser_length")]
    length: U256,
    #[serde(with = "CallAssumptionsDTO")]
    callAssumptions: CallAssumptions,
}

#[allow(clippy::trivially_copy_pass_by_ref, clippy::panic)]
fn ser_proof_mode<S>(mode: &ProofMode, state: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let as_u8: u8 = match mode {
        ProofMode::GROTH16 => 0,
        ProofMode::FAKE => 1,
        _ => panic!("unexpected enum variant for ProofMode"),
    };
    state.serialize_u8(as_u8)
}

#[allow(clippy::expect_used)]
fn ser_length<S>(length: &U256, state: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    state.serialize_u64(
        u64::try_from(length)
            .expect("failed to serialize length field of Proof. Value must fit into u64"),
    )
}
