use crate::db::{state::StateDb, wrap_state::WrapStateDb};
use alloy_primitives::{FixedBytes, Sealable, Sealed};
use revm::primitives::HashMap;
use vlayer_engine::{
    chain::spec::ChainSpec,
    engine::Engine,
    ethereum::EthBlockHeader,
    evm::{
        block_header::EvmBlockHeader,
        env::{EvmEnv, ExecutionLocation},
        input::{EvmInput, MultiEvmInput},
    },
    io::{Call, GuestOutput},
    ExecutionCommitment,
};

pub struct Guest {
    env: EvmEnv<WrapStateDb, EthBlockHeader>,
}

impl Guest {
    pub fn new(
        multi_evm_input: MultiEvmInput<EthBlockHeader>,
        start_execution_location: ExecutionLocation,
    ) -> Self {
        let start_evm_input = multi_evm_input
            .get(&start_execution_location)
            .expect("cannot get start evm input")
            .to_owned(); // TODO: Remove clone and convert this object into MultiEnv

        validate_evm_input(&start_evm_input);
        let header = start_evm_input.header.clone().seal_slow();
        let block_hashes = get_block_hashes(&start_evm_input, &header);
        let db = WrapStateDb::new(StateDb::new(
            start_evm_input.state_trie,
            start_evm_input.storage_tries,
            start_evm_input.contracts,
            block_hashes,
        ));

        let chain_spec = ChainSpec::try_from_config(start_execution_location.chain_id)
            .expect("cannot get chain spec");
        let env = EvmEnv::new(db, header)
            .with_chain_spec(&chain_spec)
            .expect("cannot set chain spec");

        Guest { env }
    }

    pub fn run(&mut self, call: Call) -> GuestOutput {
        let evm_call_result = Engine::default().call(&call, &mut self.env).unwrap();
        let execution_commitment =
            ExecutionCommitment::new(self.env.header(), call.to, call.selector());

        GuestOutput {
            evm_call_result,
            execution_commitment,
        }
    }
}

fn validate_evm_input(evm_input: &EvmInput<EthBlockHeader>) {
    // verify that the state root matches the state trie
    let state_root = evm_input.state_trie.hash_slow();
    assert_eq!(
        evm_input.header.state_root(),
        &state_root,
        "State root mismatch"
    );

    // seal the header to compute its block hash
    let header = evm_input.header.clone().seal_slow();
    // validate that ancestor headers form a valid chain
    let mut previous_header = header.inner();
    for ancestor in &evm_input.ancestors {
        let ancestor_hash = ancestor.hash_slow();
        assert_eq!(
            previous_header.parent_hash(),
            &ancestor_hash,
            "Invalid chain: block {} is not the parent of block {}",
            ancestor.number(),
            previous_header.number()
        );
        previous_header = ancestor;
    }
}

fn get_block_hashes(
    evm_input: &EvmInput<EthBlockHeader>,
    header: &Sealed<EthBlockHeader>,
) -> HashMap<u64, FixedBytes<32>> {
    let mut block_hashes = HashMap::with_capacity(evm_input.ancestors.len() + 1);
    block_hashes.insert(header.number(), header.seal());
    for ancestor in &evm_input.ancestors {
        let ancestor_hash = ancestor.hash_slow();
        block_hashes.insert(ancestor.number(), ancestor_hash);
    }

    block_hashes
}
