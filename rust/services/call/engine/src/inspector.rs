use alloy_primitives::{address, b256, hex::decode, Address, ChainId, B256};
use once_cell::sync::Lazy;
use revm::{
    interpreter::{CallInputs, CallOutcome},
    primitives::ExecutionResult,
    Database, EvmContext, Inspector,
};
use tracing::info;

use crate::{
    engine::EngineError,
    evm::env::location::ExecutionLocation,
    io::Call,
    utils::evm_call::{
        create_encoded_return_outcome, execution_result_to_call_outcome, split_calldata,
    },
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

type TransactionCallback<'a> =
    dyn Fn(&Call, ExecutionLocation) -> Result<ExecutionResult, EngineError> + 'a;

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
    transaction_callback: Box<TransactionCallback<'a>>,
}

impl<'a> TravelInspector<'a> {
    pub fn new(
        start_chain_id: ChainId,
        transaction_callback: impl Fn(&Call, ExecutionLocation) -> Result<ExecutionResult, EngineError>
            + 'a,
    ) -> Self {
        Self {
            start_chain_id,
            location: None,
            transaction_callback: Box::new(transaction_callback),
        }
    }

    fn set_block(&mut self, block_number: u64) {
        let chain_id = self
            .location
            .map_or(self.start_chain_id, |loc| loc.chain_id);
        info!(
            "Travel contract called with function: setBlock and block number: {:?}! Chain id remains {:?}.",
            block_number, chain_id
        );
        self.location = Some((block_number, chain_id).into());
    }

    fn set_chain(&mut self, chain_id: ChainId, block_number: u64) {
        info!(
            "Travel contract called with function: setChain, with chain id: {:?} block number: {:?}!",
            chain_id, block_number
        );
        self.location = Some((block_number, chain_id).into());
    }

    fn on_call(&self, inputs: &CallInputs) -> Option<CallOutcome> {
        let Some(location) = self.location else {
            return None; // If no setChain/setBlock happened, we don't need to teleport to a new VM, but can continue with the current one.
        };
        info!(
            "Intercepting the call. Block number: {:?}, chain id: {:?}",
            location.block_number, location.chain_id
        );
        let result =
            (self.transaction_callback)(&inputs.into(), location).expect("Intercepted call failed");
        info!("Intercepted call returned: {:?}", result);
        let outcome = execution_result_to_call_outcome(&result, inputs);
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
    use alloy_primitives::{address, Address, BlockNumber, Bytes, U256};
    use revm::{
        db::{CacheDB, EmptyDB},
        interpreter::{CallInputs, CallScheme, CallValue},
        primitives::{AccountInfo, Output, SuccessReason},
        EvmContext, Inspector,
    };

    use super::*;

    const MOCK_CALLER: Address = address!("0000000000000000000000000000000000000000");
    const MAINNET_ID: ChainId = 1;
    const SEPOLIA_ID: ChainId = 11155111;
    const MAINNET_BLOCK: BlockNumber = 20_000_000;
    const SEPOLIA_BLOCK: BlockNumber = 6_000_000;

    type StaticTransactionCallback =
        dyn Fn(&Call, ExecutionLocation) -> Result<ExecutionResult, EngineError> + Send + Sync;

    static TRANSACTION_CALLBACK: &StaticTransactionCallback = &|_, _| {
        Ok(ExecutionResult::Success {
            reason: SuccessReason::Return,
            gas_used: 0,
            gas_refunded: 0,
            logs: vec![],
            output: Output::Call(Bytes::from(vec![])),
        })
    };

    fn create_mock_call_inputs(to: Address, input: impl Into<Bytes>) -> CallInputs {
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
        let input = [selector, args].concat();
        let mut call_inputs = create_mock_call_inputs(addr, Bytes::from(input));

        let mut set_block_inspector =
            TravelInspector::new(1, |call, location| (TRANSACTION_CALLBACK)(call, location));
        set_block_inspector.call(&mut evm_context, &mut call_inputs);

        set_block_inspector
    }

    #[test]
    fn set_block_sets_chain_id_to_latest_not_start() {
        let locations: Vec<ExecutionLocation> = vec![
            (MAINNET_BLOCK, MAINNET_ID).into(),
            (SEPOLIA_BLOCK, SEPOLIA_ID).into(),
            (SEPOLIA_BLOCK - 1, SEPOLIA_ID).into(),
        ];

        let mut inspector = TravelInspector::new(locations[0].chain_id, |call, location| {
            (TRANSACTION_CALLBACK)(call, location)
        });

        inspector.set_chain(locations[1].chain_id, locations[1].block_number);
        assert_eq!(inspector.location, Some(locations[1]));

        inspector.set_block(locations[2].block_number);
        assert_eq!(inspector.location, Some(locations[2]));
    }

    #[test]
    fn call_set_block() {
        let block_num = 1;
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
        let chain_id = 1;
        let block_num = 2;
        let args = [
            U256::from(chain_id).to_be_bytes::<32>(),
            U256::from(block_num).to_be_bytes::<32>(),
        ]
        .concat();
        let inspector = inspector_call(TRAVEL_CONTRACT_ADDR, &SET_CHAIN_SELECTOR, &args);
        assert!(inspector
            .location
            .is_some_and(|loc| loc.block_number == block_num && loc.chain_id == chain_id));
    }

    #[test]
    #[should_panic]
    fn call_invalid_selector() {
        inspector_call(TRAVEL_CONTRACT_ADDR, &[0; 4], &[]);
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
        let x = u64::MAX; // To use all 8 bytes
        let slice: [u8; 32] = U256::from(x).to_be_bytes();
        let y = u64_from_be_slice(&slice);
        assert_eq!(x, y)
    }

    #[test]
    #[should_panic]
    fn u64_from_invalid_slice() {
        let slice = [0];
        _ = u64_from_be_slice(&slice);
    }
}
