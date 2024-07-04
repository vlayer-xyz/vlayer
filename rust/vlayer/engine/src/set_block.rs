use alloy_primitives::{address, Address};
use revm::{
    interpreter::{CallInputs, CallOutcome},
    Database, EvmContext, Inspector,
};
use tracing::info;

const SET_BLOCK_CONTRACT_ADDR: Address = address!("1234567890AbcdEF1234567890aBcdef12345678");

#[derive(Clone, Debug, Default)]
pub struct SetBlockInspector {
    set_block: bool,
}

impl<DB: Database> Inspector<DB> for SetBlockInspector {
    fn call(
        &mut self,
        _context: &mut EvmContext<DB>,
        inputs: &mut CallInputs,
    ) -> Option<CallOutcome> {
        info!(
            "Address: {:?}, caller:{:?}, input:{:?}",
            inputs.bytecode_address, inputs.caller, inputs.input,
        );

        if self.set_block {
            info!("Need to change block!");
        }
        self.set_block = inputs.bytecode_address == SET_BLOCK_CONTRACT_ADDR;

        None
    }
}

mod test {
    use std::collections::HashSet;

    use alloy_primitives::{address, Address, Bytes, B256, U256};
    use revm::{db::{CacheDB, EmptyDB}, interpreter::{CallInputs, CallScheme, CallValue}, primitives::{AccountInfo, Bytecode, Env, SpecId}, ContextPrecompiles, EvmContext, InnerEvmContext, Inspector, JournaledState};

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

    fn create_cache_db_evm_context_with_balance(
        env: Box<Env>,
        mut db: CacheDB<EmptyDB>,
        balance: U256,
    ) -> EvmContext<CacheDB<EmptyDB>> {
        db.insert_account_info(
            MOCK_CALLER,
            AccountInfo {
                nonce: 0,
                balance,
                code_hash: B256::default(),
                code: None,
            },
        );
        create_cache_db_evm_context(env, db)
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
    fn no_set_block() {
        let env = Env::default();
        let mut cdb = CacheDB::new(EmptyDB::default());
        let bal = U256::from(3_000_000_000_u128);
        let by = Bytecode::new_raw(Bytes::from(vec![0x60, 0x00, 0x60, 0x00]));
        let contract = address!("dead10000000000000000000000000000001dead");
        cdb.insert_account_info(
            contract,
            AccountInfo {
                nonce: 0,
                balance: bal,
                code_hash: by.clone().hash_slow(),
                code: Some(by),
            },
        );
        let mut evm_context = create_cache_db_evm_context_with_balance(Box::new(env), cdb, bal);
        let mut call_inputs = create_mock_call_inputs(contract);

        let mut set_block_inspector = SetBlockInspector::default();
        set_block_inspector.call(&mut evm_context, &mut call_inputs);
        assert!(!set_block_inspector.set_block);
    }
}
