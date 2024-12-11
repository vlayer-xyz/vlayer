use std::sync::Arc;

use call_engine::HostOutput;
use dashmap::DashMap;
use derive_more::{Deref, DerefMut};
use v_call::types::CallHash;

use crate::error::AppError;

pub mod v_call;
pub mod v_get_proof_receipt;
pub mod v_versions;

pub type SharedState = Arc<State>;

#[derive(Deref, DerefMut, Default, Debug)]
pub struct State(DashMap<CallHash, ProofStatus>);

#[allow(dead_code)] // To be used in the future
#[derive(Debug)]
pub enum ProofStatus {
    WaitingForChainProof,
    Preflight,
    Proving,
    Ready(Result<HostOutput, AppError>),
}
