// We write emv env conversion tests here so we can use WrapStateDb which is defined in this crate
#[cfg(test)]
mod evm_env_conversions {
    use alloy_chains::Chain;
    use as_any::Downcast;
    use block_header::EthBlockHeader;
    use call_engine::evm::{
        env::{cached::MultiEvmEnv, location::ExecutionLocation, EvmEnv},
        input::{EvmInput, MultiEvmInput},
    };
    use chain::MAINNET_MERGE_BLOCK_NUMBER;
    use mpt::EMPTY_ROOT_HASH;

    use crate::db::wrap_state::WrapStateDb;

    #[test]
    fn converts_evm_input_into_evm_env_successfully() {
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

        let evm_env: EvmEnv<WrapStateDb> = input.into();
        let actual_header = evm_env.header().downcast_ref::<EthBlockHeader>().unwrap();

        assert_eq!(expected_header, *actual_header);
    }

    #[test]
    fn converts_multi_evm_input_into_multi_evm_env_successfully() -> anyhow::Result<()> {
        let location = ExecutionLocation {
            chain_id: Chain::mainnet().id(),
            block_number: MAINNET_MERGE_BLOCK_NUMBER,
        };
        let expected_header = EthBlockHeader {
            state_root: EMPTY_ROOT_HASH,
            number: location.block_number,
            ..Default::default()
        };
        let multi_evm_input = MultiEvmInput::from_entries([(
            location,
            EvmInput {
                header: Box::new(expected_header.clone()),
                state_trie: Default::default(),
                storage_tries: Default::default(),
                contracts: Default::default(),
                ancestors: Default::default(),
            },
        )]);

        let multi_evm_env: MultiEvmEnv<WrapStateDb> = multi_evm_input.into();
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
