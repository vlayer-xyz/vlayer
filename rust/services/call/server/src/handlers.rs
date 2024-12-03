use std::{collections::HashMap, sync::Arc};

use call_engine::HostOutput;
use parking_lot::Mutex;
use v_call::types::CallHash;

pub mod v_call;
pub mod v_get_proof_receipt;
pub mod v_versions;

pub type SharedState = Arc<Mutex<State>>;

#[derive(Default, Debug)]
pub struct State {
    hashes: HashMap<CallHash, HostOutput>,
}
