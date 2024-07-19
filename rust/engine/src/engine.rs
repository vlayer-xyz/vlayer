use alloy_primitives::{address, hex::decode, Address, TxKind};
use ethers_core::types::U256;
use once_cell::sync::Lazy;
use revm::{
    inspector_handle_register,
    interpreter::CallInputs,
    primitives::{ExecutionResult, ResultAndState, SuccessReason},
    Database, Evm, EvmContext, Inspector,
};
use thiserror::Error;
use tracing::info;

use crate::{
    evm::{
        block_header::EvmBlockHeader,
        env::{EvmEnv, ExecutionLocation},
    },
    inspector::{MockCallOutcome, TravelInspector},
    io::Call,
};

#[derive(Default)]
pub struct Engine {}

#[derive(Error, Debug, PartialEq)]
pub enum EngineError {
    #[error("EVM transact preverified error: {0}")]
    TransactPreverifiedError(String),

    #[error("EVM transact error: {0}")]
    TransactError(String),

    #[error("Unsupported chain id: {0}")]
    UnsupportedChainId(u64),

    #[error("Chain spec error: {0}")]
    ChainSpecError(String),

    #[error("EVM not found for location")]
    EvmNotFound(ExecutionLocation),

    #[error("EVM Env not found for location")]
    EvmEnvNotFound(ExecutionLocation),
}

const SELECTOR_LEN: usize = 4;
const TRAVEL_CONTRACT_ADDR: Address = address!("76dc9aa45aa006a0f63942d8f9f21bd4537972a3");
static SET_BLOCK_SELECTOR: Lazy<Vec<u8>> =
    Lazy::new(|| decode("87cea3ae").expect("Error decoding set_block function call"));
static SET_CHAIN_SELECTOR: Lazy<Vec<u8>> =
    Lazy::new(|| decode("ffbc5638").expect("Error decoding set_chain function call"));

impl Engine {
    pub fn call<D, H>(self, tx: &Call, env: &mut EvmEnv<D, H>) -> Result<Vec<u8>, EngineError>
    where
        D: Database,
        D::Error: std::fmt::Debug,
        H: EvmBlockHeader,
    {
        let evm = Evm::builder()
            .with_db(&mut env.db)
            .with_external_context(TravelInspector::new(Self::inspector_callback()))
            .with_cfg_env_with_handler_cfg(env.cfg_env.clone())
            .append_handler_register(inspector_handle_register)
            .modify_block_env(|blk_env| env.header.fill_block_env(blk_env))
            .build();

        Self::transact(evm, tx)
    }

    fn transact<D, I>(mut evm: Evm<'_, I, D>, tx: &Call) -> Result<Vec<u8>, EngineError>
    where
        D: Database,
        D::Error: std::fmt::Debug,
        I: Inspector<D>,
    {
        let tx_env = evm.tx_mut();
        tx_env.caller = tx.caller;
        tx_env.transact_to = TxKind::Call(tx.to);
        tx_env.data = tx.data.clone().into();

        let ResultAndState { result, .. } = evm
            .transact_preverified()
            .map_err(|err| EngineError::TransactPreverifiedError(format!("{:?}", err)))?;

        let ExecutionResult::Success {
            reason: SuccessReason::Return,
            output,
            ..
        } = result
        else {
            return Err(EngineError::TransactError(format!("{:?}", result)));
        };
        Ok(output.into_data().into())
    }

    pub(crate) fn inspector_callback<D: Database>() -> fn(
        &mut TravelInspector<D>,
        &mut EvmContext<&mut D>,
        &mut CallInputs,
    ) -> Option<MockCallOutcome> {
        |inspector: &mut TravelInspector<D>, _: &mut EvmContext<&mut D>, inputs: &mut CallInputs| {
            info!(
                "Address: {:?}, caller:{:?}, input:{:?}",
                inputs.bytecode_address, inputs.caller, inputs.input,
            );

            match inputs.bytecode_address {
                TRAVEL_CONTRACT_ADDR => Self::handle_travel_call(inspector, inputs),
                _ => Self::handle_call(inspector),
            }
        }
    }

    fn handle_travel_call<D: Database>(
        inspector: &mut TravelInspector<D>,
        inputs: &mut CallInputs,
    ) -> Option<MockCallOutcome> {
        let (selector, argument_bytes) = inputs.input.split_at(SELECTOR_LEN);
        let argument = U256::from_big_endian(argument_bytes);

        if selector == *SET_BLOCK_SELECTOR {
            info!(
                "Travel contract called with function: setBlock and argument: {:?}!",
                argument
            );
            inspector.set_block = Some(argument);
            return Some(MockCallOutcome::from(U256::zero()));
        } else if selector == *SET_CHAIN_SELECTOR {
            info!(
                "Travel contract called with function: setChain and argument: {:?}!",
                argument
            );
            inspector.set_chain = Some(argument);
            return Some(MockCallOutcome::from(U256::zero()));
        }
        None
    }

    fn handle_call<D: Database>(inspector: &mut TravelInspector<D>) -> Option<MockCallOutcome> {
        if let Some(block_number) = &inspector.set_block.take() {
            info!(
                "Intercepting the call. Returning last block number: {:?}",
                *block_number
            );
            return Some(MockCallOutcome::from(*block_number));
        }
        if let Some(chain_id) = &inspector.set_chain.take() {
            info!(
                "Intercepting the call. Returning last chain id: {:?}",
                *chain_id
            );
            return Some(MockCallOutcome::from(*chain_id));
        }
        None
    }
}
