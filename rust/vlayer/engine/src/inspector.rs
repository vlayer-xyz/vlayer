use alloy_primitives::hex::decode;
use alloy_primitives::{address, Address, Bytes};
use ethers_core::types::U256;
use once_cell::sync::Lazy;
use revm::interpreter::{Gas, InstructionResult, InterpreterResult};
use revm::{
    interpreter::{CallInputs, CallOutcome},
    Database, EvmContext, Inspector,
};
use tracing::info;

// First 4 bytes of the call data is the selector id - the rest are arguments.
const SELECTOR_LEN: usize = 4;
const TRAVEL_CONTRACT_ADDR: Address = address!("1234567890AbcdEF1234567890aBcdef12345678");
const HOST_ADDR: Address = address!("e7f1725e7734ce288f8367e1bb143e90bb3f0512");
static SET_BLOCK_SELECTOR: Lazy<Vec<u8>> =
    Lazy::new(|| decode("87cea3ae").expect("Error decoding set_block function call"));
static SET_CHAIN_SELECTOR: Lazy<Vec<u8>> =
    Lazy::new(|| decode("1b44fd15").expect("Error decoding set_chain function call"));

#[derive(Clone, Debug, Default)]
pub struct SetInspector {
    set_block: Option<U256>,
    set_chain: Option<U256>,
}

impl SetInspector {
    fn return_number(number: U256) -> CallOutcome {
        info!("Intercepting the call. Returning number: {:?}", number);
        let mut output = [0; 32];
        number.to_big_endian(&mut output);

        CallOutcome {
            result: InterpreterResult {
                result: InstructionResult::Return,
                output: Bytes::copy_from_slice(&output),
                gas: Gas::new(0),
            },
            memory_offset: 0..0,
        }
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
            HOST_ADDR => {
                info!("Host contract called!");
            }
            TRAVEL_CONTRACT_ADDR => {
                let (selector, argument_bytes) = inputs.input.split_at(SELECTOR_LEN);
                let argument = U256::from_big_endian(argument_bytes);

                if selector == *SET_BLOCK_SELECTOR {
                    info!(
                        "Travel contract called with function: setBlock and argument: {:?}!",
                        argument
                    );
                    self.set_block = Some(argument);
                } else if selector == *SET_CHAIN_SELECTOR {
                    info!(
                        "Travel contract called with function: setChain and argument: {:?}!",
                        argument
                    );
                    self.set_chain = Some(argument);
                }
            }
            // If the call is not setBlock/setChain but setBlock/setChain is active, intercept the call.
            _ => {
                if let Some(block_number) = &self.set_block.take() {
                    return Some(SetInspector::return_number(*block_number));
                }
                if let Some(chain_id) = &self.set_chain.take() {
                    return Some(SetInspector::return_number(*chain_id));
                }
            }
        }

        None
    }

    fn call_end(
        &mut self,
        _context: &mut EvmContext<DB>,
        _inputs: &CallInputs,
        outcome: CallOutcome,
    ) -> CallOutcome {
        dbg!(outcome)
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
