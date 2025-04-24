use alloy_primitives::{Address, ChainId, address};
use call_common::{ExecutionLocation, RevmDB, WrappedRevmDBError, metadata::Metadata};
use call_precompiles::{is_time_dependent, precompile_by_address};
use revm::{
    EvmContext, Inspector as IInspector,
    db::WrapDatabaseRef,
    interpreter::{CallInputs, CallOutcome, CallScheme},
    primitives::ExecutionResult,
};
use tracing::{debug, info};

use crate::{
    io::Call,
    travel_call::{args::Args, error::Error},
    utils::evm_call::{create_encoded_return_outcome, execution_result_to_call_outcome},
};

/// This is calculated as:
/// `address(bytes20(uint160(uint256(keccak256('vlayer.traveler')))))`
pub const CONTRACT_ADDR: Address = address!("76dC9aa45aa006A0F63942d8F9f21Bd4537972A3");

pub type TxResultWithMetadata = (ExecutionResult, Box<[Metadata]>);
type TransactionCallback<'a, D> =
    dyn Fn(&Call, ExecutionLocation) -> Result<TxResultWithMetadata, Error<D>> + 'a;

pub struct Inspector<'a, D: RevmDB> {
    start_chain_id: ChainId,
    pub location: Option<ExecutionLocation>,
    transaction_callback: Box<TransactionCallback<'a, WrappedRevmDBError<D>>>,
    metadata: Vec<Metadata>,
    is_vlayer_test: bool,
    is_on_historic_block: bool,
}

impl<'a, D: RevmDB> Inspector<'a, D> {
    pub fn new(
        start_chain_id: ChainId,
        transaction_callback: impl Fn(
            &Call,
            ExecutionLocation,
        )
            -> Result<TxResultWithMetadata, Error<WrappedRevmDBError<D>>>
        + 'a,
        is_vlayer_test: bool,
        is_on_historic_block: bool,
    ) -> Self {
        Self {
            start_chain_id,
            location: None,
            transaction_callback: Box::new(transaction_callback),
            metadata: vec![Metadata::start_chain(start_chain_id)],
            is_vlayer_test,
            is_on_historic_block,
        }
    }

    pub fn into_metadata(self) -> Box<[Metadata]> {
        self.metadata.into_boxed_slice()
    }

    fn chain_id(&self) -> ChainId {
        self.location
            .map_or(self.start_chain_id, |loc| loc.chain_id)
    }

    fn set_block(&mut self, block_number: u64) {
        let chain_id = self.chain_id();
        info!("setBlock({block_number}). Chain id remains {chain_id}.");
        self.metadata
            .push(Metadata::set_block(chain_id, block_number));
        self.location = Some((chain_id, block_number).into());
    }

    fn set_chain(&mut self, chain_id: ChainId, block_number: u64) {
        info!("setChain({chain_id}, {block_number})",);
        self.metadata
            .push(Metadata::set_chain(chain_id, block_number));
        self.location = Some((chain_id, block_number).into());
    }

    fn on_call(&mut self, inputs: &CallInputs) -> Option<CallOutcome> {
        info!("Call to normal contract or precompile");
        let Some(location) = self.location.take() else {
            info!("No location set. Returning None.");
            return None; // If no setChain/setBlock happened, we don't need to teleport to a new VM, but can continue with the current one.
        };

        // `Call` does not support `DELEGATECALL` semantics, because it only stores a single `to` address.
        // In contrast, `CallInputs` distinguishes between `target_address` (storage context) and `bytecode_address` (code location).
        // When converting `CallInputs` to `Call`, we lose this distinction, making it impossible to correctly emulate `DELEGATECALL`.
        if matches!(inputs.scheme, CallScheme::DelegateCall) {
            panic!("DELEGATECALL is not supported in travel calls");
        }
        info!(
            "Intercepting the call. Block number: {:?}, chain id: {:?}",
            location.block_number, location.chain_id
        );
        let (result, metadata) =
            (self.transaction_callback)(&inputs.into(), location).expect("Intercepted call failed");
        info!("Intercepted call returned: {result:?}");
        self.metadata.extend(metadata);
        let outcome = execution_result_to_call_outcome(&result, inputs);
        Some(outcome)
    }

    fn on_travel_call(&mut self, inputs: &CallInputs) -> Option<CallOutcome> {
        info!("Call to travel contract");
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

impl<D> IInspector<WrapDatabaseRef<&D>> for Inspector<'_, D>
where
    D: RevmDB,
{
    #[allow(clippy::panic)]
    fn call(
        &mut self,
        _context: &mut EvmContext<WrapDatabaseRef<&D>>,
        inputs: &mut CallInputs,
    ) -> Option<CallOutcome> {
        info!(caller = ?inputs.caller, callee = ?inputs.bytecode_address, "Call");
        debug!("Input: {:?}", inputs.input);

        if let Some(precompile) =
            precompile_by_address(&inputs.bytecode_address, self.is_vlayer_test)
        {
            if self.is_on_historic_block && is_time_dependent(&precompile) {
                panic!("Precompile `{:?}` is not allowed for travel calls", precompile.tag());
            }

            debug!("Calling PRECOMPILE {:?}", precompile.tag());
            self.metadata
                .push(Metadata::precompile(precompile.tag(), inputs.input.len()));
        }

        match inputs.bytecode_address {
            CONTRACT_ADDR => self.on_travel_call(inputs),
            _ => self.on_call(inputs),
        }
    }
}

#[cfg(test)]
mod test {

    use std::convert::Infallible;

    use alloy_primitives::{Address, BlockNumber, Bytes, U256, address};
    use call_precompiles::{precompile::Tag, precompile_by_tag};
    use lazy_static::lazy_static;
    use revm::{
        EvmContext, InMemoryDB, Inspector as IInspector,
        db::{EmptyDB, WrapDatabaseRef},
        interpreter::{CallInputs, CallScheme, CallValue},
        primitives::{AccountInfo, Output, SuccessReason},
    };

    use super::*;
    use crate::travel_call::args::{SET_BLOCK_SELECTOR, SET_CHAIN_SELECTOR};

    const MOCK_CALLER: Address = address!("0000000000000000000000000000000000000000");
    const MAINNET_ID: ChainId = 1;
    const SEPOLIA_ID: ChainId = 11_155_111;
    const MAINNET_BLOCK: BlockNumber = 20_000_000;
    const SEPOLIA_BLOCK: BlockNumber = 6_000_000;

    lazy_static! {
        static ref JSON_GET_STRING_PRECOMPILE: Address =
            *precompile_by_tag(&Tag::JsonGetString).unwrap().address();
        static ref WEB_PROOF_PRECOMPILE: Address =
            *precompile_by_tag(&Tag::WebProof).unwrap().address();
    }

    type StaticTransactionCallback = dyn Fn(&Call, ExecutionLocation) -> Result<TxResultWithMetadata, Error<Infallible>>
        + Send
        + Sync;

    static TRANSACTION_CALLBACK: &StaticTransactionCallback = &|_, _| {
        Ok((
            ExecutionResult::Success {
                reason: SuccessReason::Return,
                gas_used: 0,
                gas_refunded: 0,
                logs: vec![],
                output: Output::Call(Bytes::from(vec![])),
            },
            Box::new([]),
        ))
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

    fn inspector_call(
        addr: Address,
        selector: &[u8],
        args: &[u8],
    ) -> Inspector<'static, impl RevmDB> {
        let mut mock_db = InMemoryDB::default();
        mock_db.insert_account_info(addr, AccountInfo::default());

        let mut evm_context = EvmContext::new(WrapDatabaseRef::from(&mock_db));
        let input = [selector, args].concat();
        let mut call_inputs = create_mock_call_inputs(addr, Bytes::from(input));

        let mut set_block_inspector =
            Inspector::new(1, |call, location| (TRANSACTION_CALLBACK)(call, location), true, false);
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

        let mut inspector: Inspector<'_, EmptyDB> = Inspector::new(
            locations[0].chain_id,
            |call, location| (TRANSACTION_CALLBACK)(call, location),
            true,
            false,
        );

        inspector.set_chain(locations[1].chain_id, locations[1].block_number);
        assert_eq!(inspector.location, Some(locations[1]));

        inspector.set_block(locations[2].block_number);
        assert_eq!(inspector.location, Some(locations[2]));
    }

    #[test]
    fn call_set_block() {
        let block_num = 1;
        let block = U256::from(block_num).to_be_bytes::<32>();
        let inspector = inspector_call(CONTRACT_ADDR, &SET_BLOCK_SELECTOR, &block);
        assert!(
            inspector
                .location
                .is_some_and(|loc| loc.block_number == block_num)
        );
    }

    #[test]
    fn set_block_resets_after_one_call() {
        let block_num = 1;
        let mut inspector: Inspector<'_, InMemoryDB> =
            Inspector::new(MAINNET_ID, TRANSACTION_CALLBACK, true, false);
        assert_eq!(inspector.location, None);

        inspector.set_block(block_num);
        assert_eq!(inspector.location, Some((MAINNET_ID, block_num).into()));

        let call_inputs = create_mock_call_inputs(*JSON_GET_STRING_PRECOMPILE, []);
        inspector.on_call(&call_inputs);
        assert_eq!(inspector.location, None);
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
        assert!(
            inspector
                .location
                .is_some_and(|loc| loc.block_number == block_num && loc.chain_id == chain_id)
        );
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

    #[test]
    #[should_panic(expected = "DELEGATECALL is not supported in travel calls")]
    fn delegate_call_panics_in_travel_call() {
        let other_contract = address!("0000000000000000000000000000000000000000");
        let mut inspector = inspector_call(other_contract, &[], &[]);
        let mut call_inputs = create_mock_call_inputs(other_contract, []);
        call_inputs.scheme = CallScheme::DelegateCall;

        inspector.set_block(1);
        inspector.on_call(&call_inputs);
    }

    #[test]
    #[should_panic(expected = "Precompile `WebProof` is not allowed for travel calls")]
    fn panics_for_precompile_not_allowed_in_travel_call() {
        let precompile_address = *WEB_PROOF_PRECOMPILE;
        let mut mock_db = InMemoryDB::default();
        mock_db.insert_account_info(precompile_address, AccountInfo::default());

        let mut evm_context = EvmContext::new(WrapDatabaseRef::from(&mock_db));
        let mut call_inputs = create_mock_call_inputs(precompile_address, []);

        let mut inspector = Inspector::new(MAINNET_ID, TRANSACTION_CALLBACK, true, true);

        inspector.call(&mut evm_context, &mut call_inputs);
    }
}
