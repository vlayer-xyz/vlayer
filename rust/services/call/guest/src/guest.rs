use alloy_primitives::{BlockNumber, ChainId, B256};
use block_header::Hashable;
use call_engine::{
    evm::{
        env::{cached::CachedEvmEnv, location::ExecutionLocation},
        input::MultiEvmInput,
    },
    io::{Call, GuestOutput},
    travel_call_executor::TravelCallExecutor,
    CallAssumptions,
};
use chain_client::Client as ChainProofClient;
use chain_common::{ChainProof, GuestVerifier};
use risc0_zkvm::{sha::Digest, Receipt};

use crate::db::wrap_state::WrapStateDb;

pub struct Guest {
    start_execution_location: ExecutionLocation,
    evm_envs: CachedEvmEnv<WrapStateDb>,
}

struct VerifiedEnv(CachedEvmEnv<WrapStateDb>);

impl Guest {
    #[must_use]
    fn new(multi_evm_input: MultiEvmInput, start_execution_location: ExecutionLocation) -> Self {
        let multi_evm_env = multi_evm_input.into();
        let evm_envs = CachedEvmEnv::from_envs(multi_evm_env);

        Guest {
            evm_envs,
            start_execution_location,
        }
    }

    #[allow(clippy::unused_async)]
    pub async fn run(self, call: &Call) -> GuestOutput {
        let evm_call_result = TravelCallExecutor::new(&self.evm_envs)
            .call(call, self.start_execution_location)
            .unwrap();
        let start_evm_env = self
            .evm_envs
            .get(self.start_execution_location)
            .expect("cannot get start evm env");
        let call_assumptions =
            CallAssumptions::new(start_evm_env.header(), call.to, call.selector());

        GuestOutput {
            evm_call_result: evm_call_result.output,
            call_assumptions,
        }
    }
}

pub struct ChainProofVerifier {
    verifier: Box<dyn GuestVerifier>,
    elf_id: Digest,
}

impl ChainProofVerifier {
    #[must_use]
    pub fn new(verifier: impl GuestVerifier, elf_id: Digest) -> Self {
        Self {
            verifier: Box::new(verifier),
            elf_id,
        }
    }

    pub fn verify(&self, proof: &ChainProof) {
        self.verifier.verify(self.elf_id, &proof.proof);
        let receipt: Receipt =
            bincode::deserialize(&proof.proof).expect("proof deserialization failed");
        let (proven_root, elf_id): (B256, Digest) = receipt
            .journal
            .decode()
            .expect("journal deserialization failed");
        assert_eq!(elf_id, self.elf_id, "elf_id mismatch in proof");
        assert_eq!(proven_root, proof.block_trie.hash_slow(), "root hash mismatch in proof");
    }
}

pub struct GuestBuilder {
    proof_client: Box<dyn ChainProofClient>,
    proof_verifier: ChainProofVerifier,
}

impl GuestBuilder {
    #[must_use]
    pub fn new(proof_client: impl ChainProofClient, proof_verifier: ChainProofVerifier) -> Self {
        Self {
            proof_client: Box::new(proof_client),
            proof_verifier,
        }
    }

    /// Verify the input and build guest. Panics if input is invalid
    #[must_use]
    pub async fn build_guest(
        &self,
        multi_evm_input: MultiEvmInput,
        start_execution_location: ExecutionLocation,
    ) -> Guest {
        self.assert_coherency(&multi_evm_input).await;
        Guest::new(multi_evm_input, start_execution_location)
    }

    async fn get_chain_proof(
        &self,
        chain_id: ChainId,
        block_numbers: Vec<BlockNumber>,
    ) -> ChainProof {
        self.proof_client
            .get_chain_proof(chain_id, block_numbers)
            .await
            .unwrap()
    }

    async fn assert_coherency(&self, multi_evm_input: &MultiEvmInput) {
        multi_evm_input.assert_coherency();

        for (chain_id, blocks) in multi_evm_input.blocks_by_chain() {
            let block_numbers = blocks.iter().map(|(block_num, _)| *block_num).collect();
            let chain_proof = self.get_chain_proof(chain_id, block_numbers).await;
            self.proof_verifier.verify(&chain_proof);
            for (block_number, block_hash) in blocks {
                let trie_block_hash = chain_proof
                    .block_trie
                    .get(block_number)
                    .expect("block hash not found");
                assert_eq!(trie_block_hash, block_hash, "block hash mismatch");
            }
        }
    }
}
