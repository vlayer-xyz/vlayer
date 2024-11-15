use alloy_primitives::ChainId;
use provider::BlockTag;

pub struct ExecutionLocation {
    pub chain_id: ChainId,
    pub block_tag: BlockTag,
}

impl<C, B> From<(C, B)> for ExecutionLocation
where
    C: Into<ChainId>,
    B: Into<BlockTag>,
{
    fn from((chain_id, block_tag): (C, B)) -> Self {
        ExecutionLocation {
            chain_id: chain_id.into(),
            block_tag: block_tag.into(),
        }
    }
}
