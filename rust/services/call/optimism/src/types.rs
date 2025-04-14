use alloy_eips::NumHash;
use alloy_primitives::{B256, keccak256};
use common::Hashable;
use rpc::OutputResponse;
use serde::{Deserialize, Serialize};

pub mod rpc;

/// This is a subset of OutputResponse that is passed between Host and Guest
/// * It's smaller and contains only the data needed to compute the hash
/// * It does not use `alloy_serde::quantity` which uses `seq` which is not available on `risc0` version of `serde`
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SequencerOutput {
    pub version: B256,
    pub state_root: B256,
    pub withdrawal_storage_root: B256,
    pub l2_block: NumHash,
}

impl SequencerOutput {
    pub fn new(state_root: B256, withdrawal_storage_root: B256, l2_block: NumHash) -> Self {
        Self {
            version: B256::default(),
            state_root,
            withdrawal_storage_root,
            l2_block,
        }
    }
}

impl From<OutputResponse> for SequencerOutput {
    fn from(output: OutputResponse) -> Self {
        let l2_block = output.block_ref.l2_block_info;
        Self {
            version: output.version,
            state_root: output.state_root,
            withdrawal_storage_root: output.withdrawal_storage_root,
            l2_block: NumHash::new(l2_block.number, l2_block.hash),
        }
    }
}

impl Hashable for SequencerOutput {
    fn hash_slow(&self) -> B256 {
        let payload: Vec<u8> = [
            self.version.to_vec(),
            self.state_root.to_vec(),
            self.withdrawal_storage_root.to_vec(),
            self.l2_block.hash.to_vec(),
        ]
        .concat();

        keccak256(payload)
    }
}

#[cfg(test)]
mod hash_slow {
    use alloy_primitives::hex;
    use lazy_static::lazy_static;

    use super::*;

    lazy_static! {
        static ref STATE_ROOT: B256 =
            B256::from(hex!("b96b23e8db3147cf46b80eda0b97e6612cbdcec43128d5bd81a8360093cfcf17"));
        static ref WITHDRAWAL_STORAGE_ROOT: B256 =
            B256::from(hex!("1e346b4b9774c44851b6e75760e09da0495f0b9124282e0f652df80d9a876b44"));
        static ref FINALIZED_L2_HASH: B256 =
            B256::from(hex!("4cd86d480704aef6106fcd200a26f2d6e6025f1032dd9b6ae09af85198973cd9"));
        static ref OUTPUT: SequencerOutput = SequencerOutput::new(
            *STATE_ROOT,
            *WITHDRAWAL_STORAGE_ROOT,
            NumHash::new(3, *FINALIZED_L2_HASH)
        );
    }

    #[test]
    fn hash_slow_test() {
        let output = OUTPUT.clone();

        let hash = output.hash_slow();

        let expected_hash =
            B256::from(hex!("39e47bbb42c1043f6f05950f4af9dd673a521c31001cb87e47c2642040580f54"));
        assert_eq!(hash, expected_hash);
    }
}
