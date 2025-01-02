use std::sync::Arc;

use alloy_primitives::{hex::ToHexExt, U256};
use alloy_sol_types::SolValue;
use async_trait::async_trait;
use call_engine::{HostOutput, Proof, Seal};
use dashmap::DashMap;
use derive_more::{Deref, DerefMut};
use derive_new::new;
use jsonrpsee::{proc_macros::rpc, types::error::ErrorObjectOwned, Extensions};
use serde::{Deserialize, Serialize, Serializer};
use v_call::types::{Call, CallContext, CallHash};
use v_get_proof_receipt::types::CallResult;
use v_versions::Versions;

use crate::{config::Config, error::AppError, ser::ProofDTO};

pub mod v_call;
pub mod v_get_proof_receipt;
pub mod v_versions;

pub type UserToken = String;

#[derive(Clone, Debug, Deserialize)]
pub struct QueryParams {
    token: Option<UserToken>,
}

#[rpc(server)]
#[async_trait]
pub trait Rpc {
    #[method(name = "v_call", with_extensions)]
    async fn v_call(&self, call: Call, ctx: CallContext) -> Result<CallHash, AppError>;

    #[method(name = "v_getProofReceipt")]
    async fn v_get_proof_receipt(&self, hash: CallHash) -> Result<CallResult, ErrorObjectOwned>;

    #[method(name = "v_versions")]
    async fn v_versions(&self) -> Result<Versions, AppError>;
}

#[derive(Deref, DerefMut, Default)]
pub struct Proofs(DashMap<CallHash, ProofStatus>);

pub enum ProofStatus {
    /// Proof task has just been queued
    Queued,
    /// Waiting for chain service to generate proof for the start execution location
    WaitingForChainProof,
    /// Preflight computation in progress
    Preflight,
    /// Proof is being generated
    Proving,
    /// Proof generation finished
    Ready(Result<ProofReceipt, AppError>),
}

#[derive(new, Clone, Serialize)]
pub struct ProofReceipt {
    data: RawData,
    metrics: Metrics,
}

#[derive(new, Clone, Serialize)]
pub struct Metrics {
    gas: u64,
    cycles: u64,
    times: Times,
}

#[derive(new, Clone, Serialize)]
pub struct Times {
    preflight: u64,
    proving: u64,
}

#[derive(Serialize, Clone)]
pub struct RawData {
    #[serde(with = "ProofDTO")]
    proof: Proof,
    #[serde(serialize_with = "ser_evm_call_result")]
    evm_call_result: Vec<u8>,
}

fn ser_evm_call_result<S>(evm_call_result: &[u8], state: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    state.serialize_str(&evm_call_result.encode_hex_with_prefix())
}

impl TryFrom<HostOutput> for RawData {
    type Error = seal::Error;

    fn try_from(value: HostOutput) -> Result<Self, Self::Error> {
        let HostOutput {
            guest_output,
            seal,
            proof_len,
            call_guest_id,
            ..
        } = value;

        let proof = Proof {
            length: U256::from(proof_len),
            seal: decode_seal(&seal)?,
            callGuestId: call_guest_id.into(),
            // Intentionally set to 0. These fields will be updated with the correct values by the prover script, based on the verifier ABI.
            callAssumptions: guest_output.call_assumptions,
        };
        Ok(Self {
            proof,
            evm_call_result: guest_output.evm_call_result,
        })
    }
}

fn decode_seal(seal: &[u8]) -> Result<Seal, seal::Error> {
    Ok(Seal::abi_decode(seal, true)?)
}

pub type SharedConfig = Arc<Config>;
pub type SharedProofs = Arc<Proofs>;

#[derive(Clone)]
pub struct State {
    config: Arc<Config>,
    proofs: Arc<Proofs>,
}

impl State {
    pub fn new(cfg: Config) -> Self {
        let config = Arc::new(cfg);
        let proofs = Arc::new(Proofs::default());
        Self { config, proofs }
    }
}

#[async_trait]
impl RpcServer for State {
    async fn v_call(
        &self,
        extensions: &Extensions,
        call: Call,
        ctx: CallContext,
    ) -> Result<CallHash, AppError> {
        let params = extensions
            .get::<QueryParams>()
            .expect("query params should be extracted in the handler");
        v_call::v_call(self.config.clone(), self.proofs.clone(), params.clone(), call, ctx).await
    }

    async fn v_get_proof_receipt(&self, hash: CallHash) -> Result<CallResult, ErrorObjectOwned> {
        v_get_proof_receipt::v_get_proof_receipt(&self.proofs, hash)
    }

    async fn v_versions(&self) -> Result<Versions, AppError> {
        v_versions::v_versions(&self.config)
    }
}
