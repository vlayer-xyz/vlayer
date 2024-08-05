use alloy_primitives::hex::decode;
use alloy_primitives::{address, b256, Address, ChainId, B256, U256};
use once_cell::sync::Lazy;
use revm::interpreter::Interpreter;
use revm::{
    interpreter::{CallInputs, CallOutcome},
    Database, EvmContext, Inspector,
};
use tracing::info;

use crate::engine::EngineError;
use crate::evm::env::location::ExecutionLocation;
use crate::io::Call;
use crate::utils::evm_call::{
    create_encoded_return_outcome, create_return_outcome, split_calldata,
};

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

#[derive(Default, Clone, Debug)]
pub struct NoopInspector;

impl<DB> Inspector<DB> for NoopInspector where DB: Database {}

type Callback<'a> = dyn Fn(&Call, ExecutionLocation) -> Result<Vec<u8>, EngineError> + 'a;

pub struct TravelInspector<'a> {
    start_chain_id: ChainId,
    pub location: Option<ExecutionLocation>,
    callback: Box<Callback<'a>>,
}

impl<'a> TravelInspector<'a> {
    pub fn new(
        start_chain_id: ChainId,
        callback: impl Fn(&Call, ExecutionLocation) -> Result<Vec<u8>, EngineError> + 'a,
    ) -> Self {
        Self {
            start_chain_id,
            location: None,
            callback: Box::new(callback),
        }
    }

    fn set_block(&mut self, block_number: u64) {
        info!(
            "Travel contract called with function: setBlock and block number: {:?}!",
            block_number
        );
        self.location = Some(ExecutionLocation::new(block_number, self.start_chain_id));
    }

    fn set_chain(&mut self, chain_id: ChainId, block_number: u64) {
        info!(
            "Travel contract called with function: setChain, with chain id: {:?} block number: {:?}!",
            chain_id, block_number
        );
        self.location = Some(ExecutionLocation::new(block_number, chain_id));
    }

    fn on_call(&self, inputs: &CallInputs) -> Option<CallOutcome> {
        let Some(location) = self.location else {
            return None; // If no setChain/setBlock happened, we don't need to teleport to a new VM, but can continue with the current one.
        };
        info!(
            "Intercepting the call. Block number: {:?}, chain id: {:?}",
            location.block_number, location.chain_id
        );
        let result = (self.callback)(&inputs.into(), location).expect("Intercepted call failed");
        info!("Intercepted call returned: {:?}", result);
        let outcome = create_return_outcome(result, inputs);
        Some(outcome)
    }

    fn on_travel_call(&mut self, inputs: &CallInputs) -> Option<CallOutcome> {
        let (selector, arguments_bytes) = split_calldata(inputs);

        if selector == *SET_BLOCK_SELECTOR {
            let block_number = U256::from_be_slice(arguments_bytes).to();
            self.set_block(block_number);
        } else if selector == *SET_CHAIN_SELECTOR {
            let (chain_id_bytes, block_number_bytes) = arguments_bytes.split_at(ARG_LEN);
            let chain_id = U256::from_be_slice(chain_id_bytes).to();
            let block_number = U256::from_be_slice(block_number_bytes).to();
            self.set_chain(chain_id, block_number);
        }

        Some(create_encoded_return_outcome(true, inputs))
    }
}

impl<'a, DB> Inspector<DB> for TravelInspector<'a>
where
    DB: Database,
{
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
            TRAVEL_CONTRACT_ADDR => self.on_travel_call(inputs),
            _ => self.on_call(inputs),
        }
    }

    fn step(&mut self, interp: &mut Interpreter, _context: &mut EvmContext<DB>) {
        dbg!(interp.current_opcode());
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

    fn inspector_call(addr: Address) -> TravelInspector<'static> {
        let mut mock_db = CacheDB::new(EmptyDB::default());
        mock_db.insert_account_info(addr, AccountInfo::default());

        let mut evm_context = EvmContext::new(mock_db);
        let mut call_inputs = create_mock_call_inputs(addr, &SET_BLOCK_SELECTOR);

        let mut set_block_inspector = TravelInspector::new(1, |_, _| Ok(vec![]));
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
