use vlayer_engine::evm::{
    env::{EvmEnv, MultiEvmEnv},
    input::{EvmInput, MultiEvmInput},
};

use crate::db::{state::StateDb, wrap_state::WrapStateDb};

pub struct ValidatedEvmInput(EvmInput);

impl From<EvmInput> for ValidatedEvmInput {
    fn from(input: EvmInput) -> Self {
        input.validate_state_root();
        input.validate_ancestors();

        ValidatedEvmInput(input)
    }
}

impl From<ValidatedEvmInput> for EvmEnv<WrapStateDb> {
    fn from(input: ValidatedEvmInput) -> Self {
        let input = input.0;
        let block_hashes = input.block_hashes();
        let db = WrapStateDb::new(StateDb::new(
            input.state_trie,
            input.storage_tries,
            input.contracts,
            block_hashes,
        ));

        EvmEnv::new(db, input.header)
    }
}

pub struct ValidatedMultiEvmInput(MultiEvmInput);

impl From<MultiEvmInput> for ValidatedMultiEvmInput {
    fn from(input: MultiEvmInput) -> Self {
        let validated = input
            .into_iter()
            .map(|(location, input)| (location, ValidatedEvmInput::from(input).0))
            .collect();
        ValidatedMultiEvmInput(validated)
    }
}

impl From<ValidatedMultiEvmInput> for MultiEvmEnv<WrapStateDb> {
    fn from(input: ValidatedMultiEvmInput) -> Self {
        input
            .0
            .into_iter()
            .map(|(location, input)| {
                let chain_spec = &location.chain_id.try_into().expect("cannot get chain spec");
                (
                    location,
                    EvmEnv::from(ValidatedEvmInput(input))
                        .with_chain_spec(chain_spec)
                        .unwrap(),
                )
            })
            .collect()
    }
}
