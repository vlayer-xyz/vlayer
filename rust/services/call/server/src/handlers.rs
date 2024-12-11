use std::sync::Arc;

use call_engine::HostOutput;
use dashmap::DashMap;
use derive_more::{Deref, DerefMut};
use serde::Deserialize;
use v_call::types::CallHash;

use crate::error::AppError;

pub mod v_call;
pub mod v_get_proof_receipt;
pub mod v_versions;

pub type SharedState = Arc<State>;

#[derive(Deref, DerefMut, Default, Debug)]
pub struct State(DashMap<CallHash, Result<HostOutput, AppError>>);

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    token: Option<String>,
}
