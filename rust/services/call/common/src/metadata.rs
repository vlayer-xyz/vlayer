use alloy_primitives::ChainId;
use call_precompiles::precompile::Tag;
use derive_new::new;

use super::ExecutionLocation;

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
