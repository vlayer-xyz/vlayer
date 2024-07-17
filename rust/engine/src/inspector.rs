use alloy_primitives::hex::decode;
use alloy_primitives::{address, Address, Bytes};
use ethers_core::types::U256;
use once_cell::sync::Lazy;
use revm::interpreter::{Gas, InstructionResult, Interpreter, InterpreterResult, OpCode};
use revm::{
    interpreter::{CallInputs, CallOutcome},
    Database, EvmContext, Inspector,
};
use tracing::info;

use crate::consts::U256_BYTES;

// First 4 bytes of the call data is the selector id - the rest are arguments.
const SELECTOR_LEN: usize = 4;
const TRAVEL_CONTRACT_ADDR: Address = address!("76dc9aa45aa006a0f63942d8f9f21bd4537972a3");
static SET_BLOCK_SELECTOR: Lazy<Vec<u8>> =
    Lazy::new(|| decode("87cea3ae").expect("Error decoding set_block function call"));
static SET_CHAIN_SELECTOR: Lazy<Vec<u8>> =
    Lazy::new(|| decode("ffbc5638").expect("Error decoding set_chain function call"));

#[derive(Clone, Debug, Default)]
pub struct SetInspector {
    set_block: Option<U256>,
    set_chain: Option<U256>,
}

struct MockCallOutcome(CallOutcome);

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

impl<DB: Database> Inspector<DB> for SetInspector {
    fn call(
        &mut self,
        _context: &mut EvmContext<DB>,
        inputs: &mut CallInputs,
    ) -> Option<CallOutcome> {
        info!(
            "Address: {:?}, caller:{:?}, input:{:?}",
            inputs.bytecode_address, inputs.caller, inputs.input,
        );

        match inputs.bytecode_address {
            TRAVEL_CONTRACT_ADDR => {
                let (selector, argument_bytes) = inputs.input.split_at(SELECTOR_LEN);
                let argument = U256::from_big_endian(argument_bytes);

                if selector == *SET_BLOCK_SELECTOR {
                    info!(
                        "Travel contract called with function: setBlock and argument: {:?}!",
                        argument
                    );
                    self.set_block = Some(argument);
                    return Some(MockCallOutcome::from(U256::zero()).0)
                } else if selector == *SET_CHAIN_SELECTOR {
                    info!(
                        "Travel contract called with function: setChain and argument: {:?}!",
                        argument
                    );
                    self.set_chain = Some(argument);
                    return Some(MockCallOutcome::from(U256::zero()).0)
                }
            }
            // If the call is not setBlock/setChain but setBlock/setChain is active, intercept the call.
            _ => {
                if let Some(block_number) = &self.set_block.take() {
                    info!(
                        "Intercepting the call. Returning last block number: {:?}",
                        *block_number
                    );
                    return Some(MockCallOutcome::from(*block_number).0);
                }
                if let Some(chain_id) = &self.set_chain.take() {
                    info!(
                        "Intercepting the call. Returning last chain id: {:?}",
                        *chain_id
                    );
                    return Some(MockCallOutcome::from(*chain_id).0);
                }
            }
        }

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

    use super::{SetInspector, SET_BLOCK_SELECTOR, TRAVEL_CONTRACT_ADDR};

    const MOCK_CALLER: Address = address!("0000000000000000000000000000000000000000");

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

    fn inspector_call(addr: Address) -> SetInspector {
        let mut mock_db = CacheDB::new(EmptyDB::default());
        mock_db.insert_account_info(addr, AccountInfo::default());

        let mut evm_context = EvmContext::new(mock_db);
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
