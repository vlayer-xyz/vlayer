use block_header::EvmBlockHeader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Input {
    Initialize { block: Box<dyn EvmBlockHeader> },
}
