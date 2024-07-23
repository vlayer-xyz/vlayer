use alloy_primitives::hex::decode;
use alloy_primitives::{address, b256, Address, Bytes, B256, U256};
use once_cell::sync::Lazy;
use revm::interpreter::{Gas, InstructionResult, InterpreterResult};
use revm::{
    interpreter::{CallInputs, CallOutcome},
    Database, EvmContext, Inspector,
};
use tracing::info;

use crate::consts::U256_BYTES;
use crate::evm::env::ExecutionLocation;

// First 4 bytes of the call data is the selector id - the rest are arguments.
const SELECTOR_LEN: usize = 4;
// The length of an argument in call data is 32 bytes.
const ARG_LEN: usize = 32;
/// This is calculated as:
/// `address(bytes20(uint160(uint256(keccak256('vlayer.traveler')))))`
pub const TRAVEL_CONTRACT_ADDR: Address = address!("76dC9aa45aa006A0F63942d8F9f21Bd4537972A3");

/// `keccak256(abi.encodePacked(TRAVEL_CONTRACT_ADDR))`
pub const TRAVEL_CONTRACT_HASH: B256 =
    b256!("262498cb66e1ee19d92574a1083e664489e446c94e8cfeb3eefe00a30be92891");

static SET_BLOCK_SELECTOR: Lazy<Vec<u8>> =
    Lazy::new(|| decode("87cea3ae").expect("Error decoding set_block function call"));
static SET_CHAIN_SELECTOR: Lazy<Vec<u8>> =
    Lazy::new(|| decode("ffbc5638").expect("Error decoding set_chain function call"));

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
        let output: [u8; U256_BYTES] = number.to_be_bytes();
        MockCallOutcome::from(Bytes::copy_from_slice(&output))
    }
}

#[derive(Clone, Debug)]
pub struct TravelInspector {
    start_chain_id: u64,
    pub location: Option<ExecutionLocation>,
    callback: fn(location: ExecutionLocation, inputs: &mut CallInputs) -> Option<MockCallOutcome>,
}

impl Default for TravelInspector {
    fn default() -> Self {
        Self {
            start_chain_id: 0,
            location: None,
            callback: |_, _| None,
        }
    }
}

impl TravelInspector {
    pub fn new(
        start_chain_id: u64,
        callback: fn(
            location: ExecutionLocation,
            inputs: &mut CallInputs,
        ) -> Option<MockCallOutcome>,
    ) -> Self {
        Self {
            start_chain_id,
            location: None,
            callback,
        }
    }

    fn set_block(&mut self, block_number: u64) -> MockCallOutcome {
        info!(
            "Travel contract called with function: setBlock and block number: {:?}!",
            block_number
        );
        self.location = Some(ExecutionLocation::new(block_number, self.start_chain_id));
        MockCallOutcome::from(U256::ZERO)
    }

    fn set_chain(&mut self, chain_id: u64, block_number: u64) -> MockCallOutcome {
        info!(
            "Travel contract called with function: setChain, with chain id: {:?} block number: {:?}!",
            chain_id, block_number
        );
        self.location = Some(ExecutionLocation::new(block_number, chain_id));
        MockCallOutcome::from(U256::ZERO)
    }
}

impl<DB: Database> Inspector<DB> for TravelInspector {
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
                let (selector, arguments_bytes) = inputs.input.split_at(SELECTOR_LEN);

                if selector == *SET_BLOCK_SELECTOR {
                    let block_number = U256::from_be_slice(arguments_bytes).to();
                    return Some(self.set_block(block_number).0);
                } else if selector == *SET_CHAIN_SELECTOR {
                    let (chain_id_bytes, block_number_bytes) = arguments_bytes.split_at(ARG_LEN);
                    let chain_id = U256::from_be_slice(chain_id_bytes).to();
                    let block_number = U256::from_be_slice(block_number_bytes).to();
                    return Some(self.set_chain(chain_id, block_number).0);
                }
            }
            // If the call is not to the travel contract AND the location is set, run callback.
            _ => {
                if let Some(location) = self.location {
                    if let Some(outcome) = (self.callback)(location, inputs) {
                        return Some(outcome.0);
                    }
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

    use super::{TravelInspector, SET_BLOCK_SELECTOR, TRAVEL_CONTRACT_ADDR};

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

    fn inspector_call(addr: Address) -> TravelInspector {
        let mut mock_db = CacheDB::new(EmptyDB::default());
        mock_db.insert_account_info(addr, AccountInfo::default());

        let mut evm_context = EvmContext::new(mock_db);
        let mut call_inputs = create_mock_call_inputs(addr, &SET_BLOCK_SELECTOR);

        let mut set_block_inspector = TravelInspector::new(1, |_, _| None);
        set_block_inspector.call(&mut evm_context, &mut call_inputs);

        set_block_inspector
    }

    #[test]
    fn call_to_travel_contract() {
        let inspector = inspector_call(TRAVEL_CONTRACT_ADDR);
        assert!(inspector.location.is_some());
    }

    #[test]
    fn call_to_other_contract() {
        let other_contract = address!("0000000000000000000000000000000000000000");
        let inspector = inspector_call(other_contract);
        assert!(inspector.location.is_none());
    }
}
