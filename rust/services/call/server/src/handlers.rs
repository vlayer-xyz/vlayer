use std::{collections::HashMap, sync::Arc};

use call_engine::HostOutput;
use derive_more::{Deref, DerefMut};
use parking_lot::RwLock;
use v_call::types::CallHash;

pub mod v_call;
pub mod v_get_proof_receipt;
pub mod v_versions;

pub type SharedState = Arc<RwLock<State>>;

#[derive(Deref, DerefMut, Default, Debug)]
pub struct State(HashMap<CallHash, HostOutput>);
