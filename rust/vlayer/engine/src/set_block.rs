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

#[cfg(test)]
mod test {
    use alloy_primitives::{address, Address, Bytes, U256};
    use revm::{
        db::{CacheDB, EmptyDB},
        interpreter::{CallInputs, CallScheme, CallValue},
        primitives::AccountInfo,
        EvmContext, Inspector,
    };

    use super::{SetBlockInspector, SET_BLOCK_CONTRACT_ADDR};

    const MOCK_CALLER: Address = address!("0000000000000000000000000000000000000000");

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

    fn inspector_call(addr: Address) -> SetBlockInspector {
        let mut mock_db = CacheDB::new(EmptyDB::default());
        mock_db.insert_account_info(addr, AccountInfo::default());

        let mut evm_context = EvmContext::new(mock_db);
        let mut call_inputs = create_mock_call_inputs(addr);

        let mut set_block_inspector = SetBlockInspector::default();
        set_block_inspector.call(&mut evm_context, &mut call_inputs);

        set_block_inspector
    }

    #[test]
    fn call_to_set_block_contract() {
        let inspector = inspector_call(SET_BLOCK_CONTRACT_ADDR);
        assert!(inspector.set_block);
    }

    #[test]
    fn call_to_other_contract() {
        let other_contract = address!("0000000000000000000000000000000000000000");
        let inspector = inspector_call(other_contract);
        assert!(!inspector.set_block);
    }
}
