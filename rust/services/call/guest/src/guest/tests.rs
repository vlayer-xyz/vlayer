use alloy_primitives::{B256, BlockHash, BlockNumber, ChainId};
use block_header::{EthBlockHeader, EvmBlockHeader};
use call_common::ExecutionLocation;
use call_engine::{
    evm::input::{EvmInput, MultiEvmInput},
    verifier::{time_travel, travel_call},
};
use mpt::KeccakMerkleTrie as MerkleTrie;

use super::*;

const CHAIN_ID: ChainId = 1;
const BLOCK_NUM: BlockNumber = 0;
const EXEC_LOCATION: ExecutionLocation = ExecutionLocation::new(CHAIN_ID, BLOCK_NUM);

fn time_travel_invalid_zk_proof(
    _: ChainId,
    _: Vec<(BlockNumber, BlockHash)>,
) -> time_travel::Result {
    Err(time_travel::Error::ChainProof(chain_common::verifier::Error::Zk(
        zk_proof::Error::InvalidProof,
    )))
}

const fn teleport_ok(_: &CachedEvmEnv<GuestDb>, _: ExecutionLocation) -> teleport::Result<()> {
    Ok(())
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
        multi_evm_input.assert_coherency();
    }

    #[tokio::test]
    #[should_panic(expected = "State root mismatch")]
    async fn state_root_mismatch() {
        let state_trie = MerkleTrie::new();
        let state_root = B256::ZERO; // invalid state root hash
        let multi_evm_input = mock_multi_evm_input(state_trie, state_root);
        multi_evm_input.assert_coherency();
    }

    #[tokio::test]
    async fn zk_verification_failed() {
        let state_trie = MerkleTrie::new();
        let state_root = state_trie.hash_slow();
        let multi_evm_input = mock_multi_evm_input(state_trie, state_root);
        let envs = create_envs_from_input(multi_evm_input);
        let cached_envs = CachedEvmEnv::from_envs(envs);
        let verifier = travel_call::Verifier::new(time_travel_invalid_zk_proof, teleport_ok);
        let verification_err = verifier
            .verify(&cached_envs, EXEC_LOCATION)
            .await
            .unwrap_err();
        assert!(matches!(
            verification_err,
            travel_call::Error::TimeTravel(time_travel::Error::ChainProof(
                chain_common::verifier::Error::Zk(zk_proof::Error::InvalidProof)
            ))
        ));
    }
}
