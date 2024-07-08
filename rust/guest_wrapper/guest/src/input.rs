use crate::db::{state::StateDb, wrap_state::WrapStateDb};
use vlayer_engine::evm::{block_header::EvmBlockHeader, env::EvmEnv, input::EvmInput};

pub struct ValidatedEvmInput<H>(EvmInput<H>);

impl<H: EvmBlockHeader + Clone> From<EvmInput<H>> for ValidatedEvmInput<H> {
    fn from(input: EvmInput<H>) -> Self {
        // verify that the state root matches the state trie
        let state_root = input.state_trie.hash_slow();
        assert_eq!(
            input.header.state_root(),
            &state_root,
            "State root mismatch"
        );

        // seal the header to compute its block hash
        let header = input.header.clone().seal_slow();
        // validate that ancestor headers form a valid chain
        let mut previous_header = header.inner();
        for ancestor in &input.ancestors {
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

        ValidatedEvmInput(input)
    }
}

impl<H: EvmBlockHeader + Clone> From<ValidatedEvmInput<H>> for EvmEnv<WrapStateDb, H> {
    fn from(input: ValidatedEvmInput<H>) -> Self {
        let input = input.0;
        let header = input.header.clone().seal_slow();
        let block_hashes = input.block_hashes();
        let db = WrapStateDb::new(StateDb::new(
            input.state_trie,
            input.storage_tries,
            input.contracts,
            block_hashes,
        ));

        EvmEnv::new(db, header)
    }
}
