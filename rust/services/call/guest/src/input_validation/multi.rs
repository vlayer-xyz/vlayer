use crate::db::wrap_state::WrapStateDb;
use call_engine::evm::{
    env::{cached::MultiEvmEnv, EvmEnv},
    input::MultiEvmInput,
};
use std::cell::RefCell;
use std::rc::Rc;

use super::single::ValidatedEvmInput;

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
        RefCell::new(
            input
                .0
                .into_iter()
                .map(|(location, input)| {
                    let chain_spec = &location.chain_id.try_into().expect("cannot get chain spec");
                    (
                        location,
                        Rc::new(
                            EvmEnv::from(ValidatedEvmInput(input))
                                .with_chain_spec(chain_spec)
                                .unwrap(),
                        ),
                    )
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod multi_evm_env_from_input {

    use super::*;
    use alloy_chains::Chain;
    use as_any::Downcast;
    use call_engine::{
        block_header::eth::EthBlockHeader,
        config::MAINNET_MERGE_BLOCK_NUMBER,
        evm::{env::location::ExecutionLocation, input::EvmInput},
    };
    use mpt::EMPTY_ROOT_HASH;

    #[test]
    fn success() -> anyhow::Result<()> {
        let location = ExecutionLocation {
            chain_id: Chain::mainnet().id(),
            block_number: MAINNET_MERGE_BLOCK_NUMBER,
        };
        let expected_header = EthBlockHeader {
            state_root: EMPTY_ROOT_HASH,
            number: location.block_number,
            ..Default::default()
        };
        let input = MultiEvmInput::from([(
            location,
            EvmInput {
                header: Box::new(expected_header.clone()),
                state_trie: Default::default(),
                storage_tries: Default::default(),
                contracts: Default::default(),
                ancestors: Default::default(),
            },
        )]);

        let validated_input = ValidatedMultiEvmInput::from(input);
        let multi_evm_env = MultiEvmEnv::from(validated_input);
        let multi_evm_env = multi_evm_env.borrow();
        let actual_header = multi_evm_env
            .get(&location)
            .unwrap()
            .header()
            .downcast_ref::<EthBlockHeader>()
            .unwrap();

        assert_eq!(expected_header, *actual_header);
        Ok(())
    }
}
