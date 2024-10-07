use alloy_primitives::B256;
use block_header::EvmBlockHeader;
use chain_engine::BlockTrie;
pub use chain_engine::Input;
use mpt::MerkleTrie;
use risc0_zkp::core::digest::Digest;

fn initialize(elf_id: Digest, block: &dyn EvmBlockHeader) -> (B256, Digest) {
    let mut trie = BlockTrie::new();
    trie.insert(block.number(), &block.hash_slow());
    (trie.hash_slow(), elf_id)
}

fn append_prepend(
    elf_id: Digest,
    prepend_blocks: impl DoubleEndedIterator<Item = Box<dyn EvmBlockHeader>>,
    append_blocks: impl Iterator<Item = Box<dyn EvmBlockHeader>>,
    mut old_leftmost_block: Box<dyn EvmBlockHeader>,
    mut mpt: BlockTrie,
) -> (B256, Digest) {
    for block in append_blocks {
        mpt = append(mpt, &*block);
    }
    for block in prepend_blocks.rev() {
        mpt = prepend(mpt, &*old_leftmost_block);
        old_leftmost_block = block;
    }
    (mpt.hash_slow(), elf_id)
}

fn append(mut mpt: BlockTrie, new_rightmost_block: &dyn EvmBlockHeader) -> BlockTrie {
    let parent_block_idx = new_rightmost_block.number() - 1;
    let parent_block_hash = mpt
        .get(parent_block_idx)
        .expect("failed to get parent block hash");
    assert_eq!(parent_block_hash, new_rightmost_block.parent_hash(), "block hash mismatch");
    mpt.insert(new_rightmost_block.number(), &new_rightmost_block.hash_slow());
    mpt
}

fn prepend(mut mpt: BlockTrie, old_leftmost_block: &dyn EvmBlockHeader) -> BlockTrie {
    let old_leftmost_block_hash = mpt
        .get(old_leftmost_block.number())
        .expect("failed to get old leftmost block hash");
    assert_eq!(old_leftmost_block_hash, old_leftmost_block.hash_slow(), "block hash mismatch");
    mpt.insert(old_leftmost_block.number() - 1, old_leftmost_block.parent_hash());
    mpt
}

pub fn main(input: Input) -> (B256, Digest) {
    match input {
        Input::Initialize { elf_id, block } => initialize(elf_id, &*block),
        Input::AppendPrepend {
            elf_id,
            prepend_blocks,
            append_blocks,
            old_leftmost_block,
            mpt_nodes,
        } => {
            let mpt = MerkleTrie::from_rlp_nodes(mpt_nodes.into_vec())
                .expect("failed to construct MPT from RLP nodes");
            append_prepend(
                elf_id,
                prepend_blocks.into_iter(),
                append_blocks.into_iter(),
                old_leftmost_block,
                mpt.into(),
            )
        }
    }
}
