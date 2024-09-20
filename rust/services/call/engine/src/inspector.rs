use alloy_primitives::hex::decode;
use alloy_primitives::{address, b256, Address, ChainId, B256};
use once_cell::sync::Lazy;
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

static SET_BLOCK_SELECTOR: Lazy<Box<[u8]>> = Lazy::new(|| {
    decode("87cea3ae")
        .expect("Error decoding set_block function call")
        .into_boxed_slice()
});
static SET_CHAIN_SELECTOR: Lazy<Box<[u8]>> = Lazy::new(|| {
    decode("ffbc5638")
        .expect("Error decoding set_chain function call")
        .into_boxed_slice()
});

#[derive(Default, Clone, Debug)]
pub struct NoopInspector;

impl<DB> Inspector<DB> for NoopInspector where DB: Database {}

type Callback<'a> = dyn Fn(&Call, ExecutionLocation) -> Result<Vec<u8>, EngineError> + 'a;

enum TravelCall {
    SetBlock { block_number: u64 },
    SetChain { chain_id: u64, block_number: u64 },
}

impl TravelCall {
    pub fn from_inputs(inputs: &CallInputs) -> Self {
        let (selector, arguments_bytes) = split_calldata(inputs);
        let arguments = arguments_bytes
            .chunks_exact(ARG_LEN)
            .map(u64_from_be_slice)
            .collect::<Vec<_>>();
        if selector == SET_BLOCK_SELECTOR.as_ref() {
            let [block_number] = arguments.try_into().expect("Invalid args for set_block");
            TravelCall::SetBlock { block_number }
        } else if selector == SET_CHAIN_SELECTOR.as_ref() {
            let [chain_id, block_number] =
                arguments.try_into().expect("Invalid args for set_chain");
            TravelCall::SetChain {
                chain_id,
                block_number,
            }
        } else {
            panic!("Invalid travel call selector: {:?}", selector)
        }
    }
}

/// Take last 8 bytes from slice and interpret as big-endian encoded u64.
/// Will trim larger numbers to u64 range, and panic if slice is smaller than 8 bytes.
fn u64_from_be_slice(slice: &[u8]) -> u64 {
    u64::from_be_bytes(*slice.last_chunk().expect("invalid u64 slice"))
}

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
        match TravelCall::from_inputs(inputs) {
            TravelCall::SetBlock { block_number } => self.set_block(block_number),
            TravelCall::SetChain {
                chain_id,
                block_number,
            } => self.set_chain(chain_id, block_number),
        }

        Some(create_encoded_return_outcome(&true, inputs))
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
}

#[cfg(test)]
mod test {
    use alloy_primitives::{address, Address, U256};
    use alloy_rlp::{Bytes, BytesMut};
    use revm::{
        db::{CacheDB, EmptyDB},
        interpreter::{CallInputs, CallScheme, CallValue},
        primitives::AccountInfo,
        EvmContext, Inspector,
    };

    use super::*;

    const MOCK_CALLER: Address = address!("0000000000000000000000000000000000000000");

    fn create_mock_call_inputs(to: Address, input: Bytes) -> CallInputs {
        CallInputs {
            input: input.into(),
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

    fn inspector_call(addr: Address, selector: &[u8], args: &[u8]) -> TravelInspector<'static> {
        let mut mock_db = CacheDB::new(EmptyDB::default());
        mock_db.insert_account_info(addr, AccountInfo::default());

        let mut evm_context = EvmContext::new(mock_db);
        let mut input = BytesMut::from(selector);
        input.extend_from_slice(args);
        let mut call_inputs = create_mock_call_inputs(addr, Bytes::from(input));

        let mut set_block_inspector = TravelInspector::new(1, |_, _| Ok(vec![]));
        set_block_inspector.call(&mut evm_context, &mut call_inputs);

        set_block_inspector
    }

    #[test]
    fn call_set_block() {
        let block_num = 2137u64;
        let inspector = inspector_call(
            TRAVEL_CONTRACT_ADDR,
            &SET_BLOCK_SELECTOR,
            &U256::from(block_num).to_be_bytes::<32>(),
        );
        assert!(inspector
            .location
            .is_some_and(|loc| loc.block_number == block_num));
    }

    #[test]
    fn call_set_chain() {
        let chain_id = 1337u64;
        let block_num = 2137u64;
        let mut args = BytesMut::with_capacity(64);
        args.extend_from_slice(&U256::from(chain_id).to_be_bytes::<32>());
        args.extend_from_slice(&U256::from(block_num).to_be_bytes::<32>());
        let inspector = inspector_call(TRAVEL_CONTRACT_ADDR, &SET_CHAIN_SELECTOR, &args);
        assert!(inspector
            .location
            .is_some_and(|loc| loc.block_number == block_num && loc.chain_id == chain_id));
    }

    #[test]
    #[should_panic]
    fn call_invalid_selector() {
        inspector_call(TRAVEL_CONTRACT_ADDR, &[1u8, 2u8, 3u8, 4u8], &[]);
    }

    #[test]
    #[should_panic]
    fn call_missing_args() {
        inspector_call(TRAVEL_CONTRACT_ADDR, &SET_BLOCK_SELECTOR, &[]);
    }

    #[test]
    fn call_to_other_contract() {
        let other_contract = address!("0000000000000000000000000000000000000000");
        let inspector = inspector_call(other_contract, &[], &[]);
        assert!(inspector.location.is_none());
    }

    #[test]
    fn u64_from_u256_be_slice() {
        let x = 20240918u64;
        let slice: [u8; 32] = U256::from(x).to_be_bytes();
        let y = u64_from_be_slice(&slice);
        assert_eq!(x, y)
    }
}
