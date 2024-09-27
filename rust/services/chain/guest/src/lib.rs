use block_header::EvmBlockHeader;
pub use chain_engine::Input;

pub struct Guest {}

impl Guest {
    pub fn initialize(_block: &dyn EvmBlockHeader) -> Box<[u8]> {
        Box::new([])
    }
}
