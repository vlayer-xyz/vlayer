#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use alloy_primitives::{address, Address, Bytes, U256};
    use revm::{
        db::{CacheDB, EmptyDB},
        interpreter::{CallInputs, CallScheme, CallValue},
        primitives::{AccountInfo, Env, SpecId},
        ContextPrecompiles, EvmContext, InnerEvmContext, Inspector, JournaledState,
    };

    use crate::set_block::SetBlockInspector;

    const MOCK_CALLER: Address = address!("0000000000000000000000000000000000000000");

    fn create_cache_db_evm_context(
        env: Box<Env>,
        db: CacheDB<EmptyDB>,
    ) -> EvmContext<CacheDB<EmptyDB>> {
        EvmContext {
            inner: InnerEvmContext {
                env,
                journaled_state: JournaledState::new(SpecId::CANCUN, HashSet::new()),
                db,
                error: Ok(()),
            },
            precompiles: ContextPrecompiles::default(),
        }
    }

    fn create_mock_call_inputs(to: Address) -> CallInputs {
        CallInputs {
            input: Bytes::new(),
            gas_limit: 0,
            bytecode_address: to,
            target_address: to,
            caller: MOCK_CALLER,
            value: CallValue::Transfer(U256::ZERO),
            scheme: CallScheme::Call,
            is_eof: false,
            is_static: false,
            return_memory_offset: 0..0,
        }
    }

    #[test]
    fn dont_set_block() {
        let mut mock_db = CacheDB::new(EmptyDB::default());
        let not_set_block_contract = address!("dead10000000000000000000000000000001dead");
        mock_db.insert_account_info(not_set_block_contract, AccountInfo::default());
        let mut evm_context = create_cache_db_evm_context(Box::new(Env::default()), mock_db);
        let mut call_inputs = create_mock_call_inputs(not_set_block_contract);

        let mut set_block_inspector = SetBlockInspector::default();
        set_block_inspector.call(&mut evm_context, &mut call_inputs);

        assert!(!set_block_inspector.set_block);
    }

    #[test]
    fn set_block() {
        let mut mock_db = CacheDB::new(EmptyDB::default());
        let not_set_block_contract = address!("1234567890AbcdEF1234567890aBcdef12345678");
        mock_db.insert_account_info(not_set_block_contract, AccountInfo::default());
        let mut evm_context = create_cache_db_evm_context(Box::new(Env::default()), mock_db);
        let mut call_inputs = create_mock_call_inputs(not_set_block_contract);

        let mut set_block_inspector = SetBlockInspector::default();
        set_block_inspector.call(&mut evm_context, &mut call_inputs);

        assert!(set_block_inspector.set_block);
    }
}
