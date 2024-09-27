use block_header::EvmBlockHeader;
pub use chain_engine::Input;
use serde::Serialize;

pub struct Guest {}

impl Guest {
    pub fn initialize(block: &dyn EvmBlockHeader) -> impl Serialize {
        block.number()
    }
}
