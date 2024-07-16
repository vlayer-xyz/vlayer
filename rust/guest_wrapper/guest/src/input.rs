use crate::db::{state::StateDb, wrap_state::WrapStateDb};
use vlayer_engine::evm::{
    block_header::EvmBlockHeader,
    env::{EvmEnv, MultiEvmEnv},
    input::{EvmInput, MultiEvmInput},
};

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
        let header = input.header;

        EvmEnv::new(db, header)
    }
}

pub struct ValidatedMultiEvmInput(MultiEvmInput);

impl From<MultiEvmInput> for ValidatedMultiEvmInput {
    fn from(input: MultiEvmInput) -> Self {
        let validated = input
            .0
            .into_iter()
            .map(|(location, input)| (location, ValidatedEvmInput::from(input).0))
            .collect();
        ValidatedMultiEvmInput(MultiEvmInput(validated))
    }
}

impl From<ValidatedMultiEvmInput> for MultiEvmEnv<WrapStateDb> {
    fn from(input: ValidatedMultiEvmInput) -> Self {
        let envs = input
            .0
             .0
            .into_iter()
            .map(|(location, input)| (location, EvmEnv::from(ValidatedEvmInput(input))))
            .collect();
        MultiEvmEnv(envs)
    }
}
