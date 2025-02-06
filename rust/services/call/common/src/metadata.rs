use alloy_primitives::{BlockNumber, ChainId};
use call_precompiles::precompile::Tag;
use derive_new::new;

#[derive(Debug, Copy, Clone)]
pub enum Metadata {
    Precompile(Precompile),
    StartChain(ChainId),
    SetChain(ExecutionLocation),
    SetBlock(ExecutionLocation),
}

#[derive(new, Debug, Copy, Clone)]
pub struct Precompile {
    pub tag: Tag,
    pub calldata_length: usize,
}

#[derive(new, Debug, Copy, Clone)]
pub struct ExecutionLocation {
    pub chain_id: ChainId,
    pub block_number: BlockNumber,
}
