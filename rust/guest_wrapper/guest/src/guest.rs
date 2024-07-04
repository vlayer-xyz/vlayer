use crate::db::{state::StateDb, wrap_state::WrapStateDb};
use alloy_primitives::{FixedBytes, Sealable, Sealed};
use revm::primitives::HashMap;
use vlayer_engine::{
    chain::spec::ChainSpec,
    config::SEPOLIA_ID,
    engine::Engine,
    ethereum::EthBlockHeader,
    evm::{block_header::EvmBlockHeader, env::EvmEnv, input::EvmInput},
    io::{Call, GuestOutput},
};

pub struct Guest {
    db: WrapStateDb,
    header: EthBlockHeader,
}

impl Guest {
    pub fn new(evm_input: EvmInput<EthBlockHeader>) -> Self {
        validate_evm_input(&evm_input);
        let header = evm_input.header.clone().seal_slow();
        let block_hashes = get_block_hashes(&evm_input, &header);
        let db = WrapStateDb::new(StateDb::new(
            evm_input.state_trie,
            evm_input.storage_tries,
            evm_input.contracts,
            block_hashes,
        ));

        Guest {
            db,
            header: header.inner().clone(),
        }
    }

    pub fn run(&mut self, call: Call) -> GuestOutput {
        let chain_spec = ChainSpec::try_from_config(SEPOLIA_ID).expect("cannot get chain spec");
        let mut env = EvmEnv::new(&mut self.db, self.header.clone().seal_slow())
            .with_chain_spec(&chain_spec)
            .expect("cannot set chain spec");

        let evm_call_result = Engine::default().call(&call, &mut env).unwrap();
        let function_selector = get_selector(&call);
        let execution_commitment = self.header.execution_commitment(call.to, function_selector);

        GuestOutput {
            evm_call_result,
            execution_commitment,
        }
    }
}

fn get_selector(call: &Call) -> [u8; 4] {
    call.data[0..4]
        .try_into()
        .expect("cannot extract function selector from call data")
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
