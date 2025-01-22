use alloy_primitives::{BlockNumber, ChainId, B256};
use block_header::{EthBlockHeader, EvmBlockHeader};
use call_engine::{
    evm::{
        env::location::ExecutionLocation,
        input::{EvmInput, MultiEvmInput},
    },
    verifier::{chain_proof, guest_input, time_travel, zk_proof},
};
use mpt::KeccakMerkleTrie as MerkleTrie;

use super::*;

const CHAIN_ID: ChainId = 1;
const BLOCK_NUM: BlockNumber = 0;
const EXEC_LOCATION: ExecutionLocation = ExecutionLocation::new(CHAIN_ID, BLOCK_NUM);

const fn input_ok(_: &MultiEvmInput) -> guest_input::Result {
    Ok(())
}

const fn input_invalid(_: &MultiEvmInput) -> guest_input::Result {
    Err(guest_input::Error::TimeTravel(time_travel::Error::ChainProof(
        chain_proof::Error::Zk(zk_proof::Error::InvalidProof),
    )))
}

fn mock_header(state_root: B256) -> Box<dyn EvmBlockHeader> {
    let header = EthBlockHeader {
        number: BLOCK_NUM,
        state_root,
        ..Default::default()
    };
    Box::new(header)
}

fn mock_evm_input(header: Box<dyn EvmBlockHeader>, state_trie: MerkleTrie) -> EvmInput {
    EvmInput {
        header,
        state_trie,
        storage_tries: Default::default(),
        contracts: Default::default(),
        ancestors: Default::default(),
    }
}

fn mock_multi_evm_input(state_trie: MerkleTrie, state_root: B256) -> MultiEvmInput {
    let header = mock_header(state_root);
    let evm_input = mock_evm_input(header, state_trie);
    MultiEvmInput::from_entries([(EXEC_LOCATION, evm_input)])
}

mod verify_env {
    use block_header::Hashable;

    use super::*;

    #[tokio::test]
    async fn ok() {
        let state_trie = MerkleTrie::new();
        let state_root = state_trie.hash_slow();
        let multi_evm_input = mock_multi_evm_input(state_trie, state_root);
        _ = verify_input(input_ok, multi_evm_input).await;
    }

    #[tokio::test]
    #[should_panic(expected = "State root mismatch")]
    async fn state_root_mismatch() {
        let state_trie = MerkleTrie::new();
        let state_root = B256::ZERO; // invalid state root hash
        let multi_evm_input = mock_multi_evm_input(state_trie, state_root);
        _ = verify_input(input_ok, multi_evm_input).await;
    }

    #[tokio::test]
    #[should_panic(expected = "invalid guest input")]
    async fn zk_verification_failed() {
        let state_trie = MerkleTrie::new();
        let state_root = state_trie.hash_slow();
        let multi_evm_input = mock_multi_evm_input(state_trie, state_root);
        _ = verify_input(input_invalid, multi_evm_input).await;
    }
}
