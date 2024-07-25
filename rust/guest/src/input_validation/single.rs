use crate::db::{state::StateDb, wrap_state::WrapStateDb};
use vlayer_engine::evm::{env::EvmEnv, input::EvmInput};

pub struct ValidatedEvmInput(pub(crate) EvmInput);

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
