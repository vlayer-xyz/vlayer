use alloy_primitives::{address, Address, ChainId};
use call_common::metadata::{ExecutionLocation as MetaExecutionLocation, Metadata, Precompile};
use call_precompiles::precompile_by_address;
use revm::{
    interpreter::{CallInputs, CallOutcome},
    primitives::ExecutionResult,
    Database, EvmContext, Inspector as IInspector,
};
use tracing::{debug, info};

use crate::{
    evm::env::location::ExecutionLocation,
    io::Call,
    travel_call::{self, args::Args},
    utils::evm_call::{create_encoded_return_outcome, execution_result_to_call_outcome},
};

/// This is calculated as:
/// `address(bytes20(uint160(uint256(keccak256('vlayer.traveler')))))`
pub const CONTRACT_ADDR: Address = address!("76dC9aa45aa006A0F63942d8F9f21Bd4537972A3");

type TransactionCallback<'a> =
    dyn Fn(&Call, ExecutionLocation) -> Result<ExecutionResult, travel_call::error::Error> + 'a;

pub struct Inspector<'a> {
    start_chain_id: ChainId,
    pub location: Option<ExecutionLocation>,
    transaction_callback: Box<TransactionCallback<'a>>,
    proof_metadata: Vec<Metadata>,
}

impl<'a> Inspector<'a> {
    pub fn new(
        start_chain_id: ChainId,
        transaction_callback: impl Fn(&Call, ExecutionLocation) -> Result<ExecutionResult, travel_call::error::Error>
            + 'a,
    ) -> Self {
        Self {
            start_chain_id,
            location: None,
            transaction_callback: Box::new(transaction_callback),
            proof_metadata: vec![Metadata::StartChain(start_chain_id)],
        }
    }

    fn chain_id(&self) -> ChainId {
        self.location
            .map_or(self.start_chain_id, |loc| loc.chain_id)
    }

    fn set_block(&mut self, block_number: u64) {
        let chain_id = self.chain_id();
        info!("setBlock({block_number}). Chain id remains {chain_id}.");
        self.proof_metadata
            .push(Metadata::SetBlock(MetaExecutionLocation::new(chain_id, block_number)));
        self.location = Some((chain_id, block_number).into());
    }

    fn set_chain(&mut self, chain_id: ChainId, block_number: u64) {
        info!("setChain({chain_id}, {block_number})",);
        self.proof_metadata
            .push(Metadata::SetChain(MetaExecutionLocation::new(chain_id, block_number)));
        self.location = Some((chain_id, block_number).into());
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
        info!("Intercepted call returned: {result:?}");
        let outcome = execution_result_to_call_outcome(&result, inputs);
        Some(outcome)
    }

    fn on_travel_call(&mut self, inputs: &CallInputs) -> Option<CallOutcome> {
        match Args::from_inputs(inputs) {
            Args::SetBlock { block_number } => self.set_block(block_number),
            Args::SetChain {
                chain_id,
                block_number,
            } => self.set_chain(chain_id, block_number),
        }

        Some(create_encoded_return_outcome(&true, inputs))
    }
}

impl<DB> IInspector<DB> for Inspector<'_>
where
    DB: Database,
{
    fn call(
        &mut self,
        _context: &mut EvmContext<DB>,
        inputs: &mut CallInputs,
    ) -> Option<CallOutcome> {
        info!("Call: {:?} -> {:?}", inputs.caller, inputs.bytecode_address);
        debug!("Input: {:?}", inputs.input);

        if let Some(precompile) = precompile_by_address(&inputs.bytecode_address) {
            debug!("Calling PRECOMPILE {:?}", precompile.tag());
            self.proof_metadata
                .push(Metadata::Precompile(Precompile::new(precompile.tag(), inputs.input.len())));
        }

        match inputs.bytecode_address {
            CONTRACT_ADDR => self.on_travel_call(inputs),
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
        EvmContext, Inspector as IInspector,
    };
    use travel_call::args::{SET_BLOCK_SELECTOR, SET_CHAIN_SELECTOR};

    use super::*;

    const MOCK_CALLER: Address = address!("0000000000000000000000000000000000000000");
    const MAINNET_ID: ChainId = 1;
    const SEPOLIA_ID: ChainId = 11_155_111;
    const MAINNET_BLOCK: BlockNumber = 20_000_000;
    const SEPOLIA_BLOCK: BlockNumber = 6_000_000;

    type StaticTransactionCallback = dyn Fn(&Call, ExecutionLocation) -> Result<ExecutionResult, travel_call::error::Error>
        + Send
        + Sync;

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

    fn inspector_call(addr: Address, selector: &[u8], args: &[u8]) -> Inspector<'static> {
        let mut mock_db = CacheDB::new(EmptyDB::default());
        mock_db.insert_account_info(addr, AccountInfo::default());

        let mut evm_context = EvmContext::new(mock_db);
        let input = [selector, args].concat();
        let mut call_inputs = create_mock_call_inputs(addr, Bytes::from(input));

        let mut set_block_inspector =
            Inspector::new(1, |call, location| (TRANSACTION_CALLBACK)(call, location));
        set_block_inspector.call(&mut evm_context, &mut call_inputs);

        set_block_inspector
    }

    #[test]
    fn set_block_sets_chain_id_to_latest_not_start() {
        let locations: Vec<ExecutionLocation> = vec![
            (MAINNET_ID, MAINNET_BLOCK).into(),
            (SEPOLIA_ID, SEPOLIA_BLOCK).into(),
            (SEPOLIA_ID, SEPOLIA_BLOCK - 1).into(),
        ];

        let mut inspector = Inspector::new(locations[0].chain_id, |call, location| {
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
            CONTRACT_ADDR,
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
        let inspector = inspector_call(CONTRACT_ADDR, &SET_CHAIN_SELECTOR, &args);
        assert!(inspector
            .location
            .is_some_and(|loc| loc.block_number == block_num && loc.chain_id == chain_id));
    }

    #[test]
    #[should_panic(expected = "Invalid travel call selector")]
    fn call_invalid_selector() {
        inspector_call(CONTRACT_ADDR, &[0; 4], &[]);
    }

    #[test]
    #[should_panic(expected = "Invalid args for set_block")]
    fn call_missing_args() {
        inspector_call(CONTRACT_ADDR, &SET_BLOCK_SELECTOR, &[]);
    }

    #[test]
    fn call_to_other_contract() {
        let other_contract = address!("0000000000000000000000000000000000000000");
        let inspector = inspector_call(other_contract, &[], &[]);
        assert!(inspector.location.is_none());
    }
}
