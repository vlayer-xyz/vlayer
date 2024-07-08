use alloy_primitives::hex::decode;
use alloy_primitives::{address, Address};
use ethers_core::types::U256;
use once_cell::sync::Lazy;
use revm::{
    interpreter::{CallInputs, CallOutcome},
    Database, EvmContext, Inspector,
};
use tracing::info;

// First 4 bytes of the call data is the selector id - the rest are arguments.
const SELECTOR_LEN: usize = 4;
const TRAVEL_CONTRACT_ADDR: Address = address!("1234567890AbcdEF1234567890aBcdef12345678");
static SET_BLOCK_SELECTOR: Lazy<Vec<u8>> =
    Lazy::new(|| decode("87cea3ae").expect("Error decoding set_block function call"));
const SET_CHAIN_SELECTOR: Lazy<Vec<u8>> =
    Lazy::new(|| decode("1b44fd15").expect("Error decoding set_chain function call"));

#[derive(Clone, Debug, Default)]
pub struct SetInspector {
    set_block: Option<U256>,
    set_chain: Option<U256>,
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
                let input = &inputs.input;
                let (selector, argument_bytes) = input.split_at(SELECTOR_LEN);
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
            _ => {
                if let Some(number) = &self.set_block {
                    info!("Need to change block to {:?}!", number);
                }
                if let Some(number) = &self.set_chain {
                    info!("Need to change chain to {:?}!", number);
                }
                self.set_block = None;
                self.set_chain = None;
            }
        }

        None
    }
}
