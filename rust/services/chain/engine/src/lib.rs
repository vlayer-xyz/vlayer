use alloy_primitives::FixedBytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    pub root_hash: FixedBytes<32>,
}
