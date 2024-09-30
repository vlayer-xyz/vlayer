use alloy_rlp::encode_fixed_size;
use block_header::EvmBlockHeader;
pub use chain_engine::Input;
use mpt::MerkleTrie;
use risc0_zkp::core::digest::Digest;
use serde::Serialize;

pub fn initialize(elf_id: Digest, block: &dyn EvmBlockHeader) -> impl Serialize {
    let mut mpt = MerkleTrie::new();
    let encoded_block_num = encode_fixed_size(&block.number());
    mpt.insert(encoded_block_num, block.hash_slow())
        .expect("insert block number");

    (mpt.hash_slow(), elf_id)
}
