use crate::db::{state::StateDb, wrap_state::WrapStateDb};
use vlayer_engine::evm::{
    block_header::EvmBlockHeader,
    env::{EvmEnv, MultiEvmEnv},
    input::{EvmInput, MultiEvmInput},
};

pub struct ValidatedEvmInput<H>(EvmInput<H>);

impl<H: EvmBlockHeader + Clone> From<EvmInput<H>> for ValidatedEvmInput<H> {
    fn from(input: EvmInput<H>) -> Self {
        input.validate_state_root();
        input.validate_ancestors();

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

pub struct ValidatedMultiEvmInput<H>(MultiEvmInput<H>);

impl<H: EvmBlockHeader + Clone> From<MultiEvmInput<H>> for ValidatedMultiEvmInput<H> {
    fn from(input: MultiEvmInput<H>) -> Self {
        let validated = input
            .0
            .into_iter()
            .map(|(location, input)| (location, ValidatedEvmInput::from(input).0))
            .collect();
        ValidatedMultiEvmInput(MultiEvmInput(validated))
    }
}

impl<H: EvmBlockHeader + Clone> From<ValidatedMultiEvmInput<H>> for MultiEvmEnv<WrapStateDb, H> {
    fn from(input: ValidatedMultiEvmInput<H>) -> Self {
        let envs = input
            .0
             .0
            .into_iter()
            .map(|(location, input)| (location, EvmEnv::from(ValidatedEvmInput(input))))
            .collect();
        MultiEvmEnv(envs)
    }
}
