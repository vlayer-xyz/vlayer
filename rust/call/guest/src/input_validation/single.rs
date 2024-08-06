use crate::db::{state::StateDb, wrap_state::WrapStateDb};
use call_engine::evm::{env::EvmEnv, input::EvmInput};

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

#[cfg(test)]
mod evm_env_from_input {

    use super::*;
    use as_any::Downcast;
    use call_engine::block_header::eth::EthBlockHeader;
    use mpt::EMPTY_ROOT_HASH;

    #[test]
    fn success() {
        let expected_header = EthBlockHeader {
            state_root: EMPTY_ROOT_HASH,
            ..Default::default()
        };
        let input = EvmInput {
            header: Box::new(expected_header.clone()),
            state_trie: Default::default(),
            storage_tries: Default::default(),
            contracts: Default::default(),
            ancestors: Default::default(),
        };

        let validated_input = ValidatedEvmInput::from(input);
        let evm_env = EvmEnv::from(validated_input);
        let actual_header = evm_env.header().downcast_ref::<EthBlockHeader>().unwrap();

        assert_eq!(expected_header, *actual_header);
    }
}
