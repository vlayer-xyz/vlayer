use std::collections::HashMap;

use alloy_primitives::{BlockHash, BlockNumber, ChainId};
use call_engine::{
    engine::Engine,
    evm::{
        env::{cached::CachedEvmEnv, location::ExecutionLocation},
        input::MultiEvmInput,
    },
    io::{Call, GuestOutput},
    CallAssumptions,
};
use chain_types::ChainProof;

use crate::db::wrap_state::WrapStateDb;

pub struct Guest {
    start_execution_location: ExecutionLocation,
    evm_envs: CachedEvmEnv<WrapStateDb>,
    #[warn(dead_code)]
    chain_id_to_blocks: HashMap<ChainId, HashMap<BlockNumber, BlockHash>>,
    #[warn(dead_code)]
    chain_id_to_chain_proof: HashMap<ChainId, ChainProof>,
}

// struct ChainProofsData {
//     map: HashMap<ChainId, HashMap<ChainId, ChainProof>>,
// }

// fn assert_chain_coherency(&self) {
//     for (chain_id, blocks) in &self.chain_id_to_blocks {
//         mpt.get()
//     }
// }

impl Guest {
    #[must_use]
    pub fn new(
        multi_evm_input: MultiEvmInput,
        start_execution_location: ExecutionLocation,
        chain_id_to_chain_proof: HashMap<ChainId, ChainProof>,
    ) -> Self {
        multi_evm_input.assert_coherency();
        let chain_id_to_blocks = multi_evm_input.group_blocks_by_chain();
        let multi_evm_env = multi_evm_input.into();
        let evm_envs = CachedEvmEnv::from_envs(multi_evm_env);

        Guest {
            evm_envs,
            chain_id_to_blocks,
            start_execution_location,
            chain_id_to_chain_proof,
        }
    }

    pub fn run(self, call: &Call) -> GuestOutput {
        let evm_call_result = Engine::new(&self.evm_envs)
            .call(call, self.start_execution_location)
            .unwrap();
        let start_evm_env = self
            .evm_envs
            .get(self.start_execution_location)
            .expect("cannot get start evm env");
        let call_assumptions =
            CallAssumptions::new(start_evm_env.header(), call.to, call.selector());

        GuestOutput {
            evm_call_result,
            call_assumptions,
        }
    }
}
