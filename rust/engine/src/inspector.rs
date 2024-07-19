use alloy_primitives::Bytes;
use ethers_core::types::U256;
use revm::interpreter::{Gas, InstructionResult, InterpreterResult};
use revm::{
    interpreter::{CallInputs, CallOutcome},
    Database, EvmContext, Inspector,
};

use crate::consts::U256_BYTES;

pub struct MockCallOutcome(CallOutcome);

impl From<Bytes> for MockCallOutcome {
    fn from(bytes: Bytes) -> Self {
        MockCallOutcome(CallOutcome {
            result: InterpreterResult {
                result: InstructionResult::Return,
                output: bytes,
                gas: Gas::new(0),
            },
            memory_offset: 0..0,
        })
    }
}

impl From<U256> for MockCallOutcome {
    fn from(number: U256) -> Self {
        let mut output = [0; U256_BYTES];
        number.to_big_endian(&mut output);
        MockCallOutcome::from(Bytes::copy_from_slice(&output))
    }
}

#[derive(Clone, Debug)]
pub struct SetInspector<DB: Database> {
    pub set_block: Option<U256>,
    pub set_chain: Option<U256>,

    callback: fn(
        &mut SetInspector<DB>,
        &mut EvmContext<&mut DB>,
        &mut CallInputs,
    ) -> Option<MockCallOutcome>,
}

impl<DB: Database> Default for SetInspector<DB> {
    fn default() -> Self {
        Self {
            set_block: None,
            set_chain: None,
            callback: |_, _, _| None,
        }
    }
}

impl<DB: Database> SetInspector<DB> {
    pub fn new(
        callback: fn(
            &mut Self,
            &mut EvmContext<&mut DB>,
            &mut CallInputs,
        ) -> Option<MockCallOutcome>,
    ) -> Self {
        Self {
            set_block: None,
            set_chain: None,
            callback,
        }
    }
}

impl<DB: Database> Inspector<&mut DB> for SetInspector<DB> {
    fn call(
        &mut self,
        context: &mut EvmContext<&mut DB>,
        inputs: &mut CallInputs,
    ) -> Option<CallOutcome> {
        match (self.callback)(self, context, inputs) {
            Some(outcome) => Some(outcome.0),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use std::convert::Infallible;

    use alloy_primitives::{address, hex::decode, Address, Bytes, U256};
    use once_cell::sync::Lazy;
    use revm::{
        db::{CacheDB, EmptyDB, EmptyDBTyped},
        interpreter::{CallInputs, CallScheme, CallValue},
        primitives::AccountInfo,
        EvmContext, Inspector,
    };

    use super::SetInspector;

    const MOCK_CALLER: Address = address!("0000000000000000000000000000000000000000");
    const TRAVEL_CONTRACT_ADDR: Address = address!("1234567890AbcdEF1234567890aBcdef12345678");
    static SET_BLOCK_SELECTOR: Lazy<Vec<u8>> =
        Lazy::new(|| decode("87cea3ae").expect("Error decoding set_block function call"));

    fn create_mock_call_inputs(to: Address, input: &[u8]) -> CallInputs {
        CallInputs {
            input: Bytes::copy_from_slice(input),
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

    fn inspector_call(addr: Address) -> SetInspector<CacheDB<EmptyDBTyped<Infallible>>> {
        let mut mock_db = CacheDB::new(EmptyDB::default());
        mock_db.insert_account_info(addr, AccountInfo::default());

        let mut evm_context = EvmContext::new(&mut mock_db);
        let mut call_inputs = create_mock_call_inputs(addr, &SET_BLOCK_SELECTOR);

        let mut set_block_inspector = SetInspector::default();
        set_block_inspector.call(&mut evm_context, &mut call_inputs);

        set_block_inspector
    }

    #[test]
    fn call_to_travel_contract() {
        let inspector = inspector_call(TRAVEL_CONTRACT_ADDR);
        assert!(inspector.set_block.is_some());
    }

    #[test]
    fn call_to_other_contract() {
        let other_contract = address!("0000000000000000000000000000000000000000");
        let inspector = inspector_call(other_contract);
        assert!(inspector.set_block.is_none());
    }
}
