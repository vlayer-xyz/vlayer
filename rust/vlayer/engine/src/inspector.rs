use alloy_primitives::hex::decode;
use alloy_primitives::{address, Address};
use ethers_core::types::U256;
use revm::{
    interpreter::{CallInputs, CallOutcome},
    Database, EvmContext, Inspector,
};
use tracing::info;

const TRAVEL_CONTRACT_ADDR: Address = address!("1234567890AbcdEF1234567890aBcdef12345678");
const SET_BLOCK_SELECTOR: &str = "87cea3ae";
const SET_CHAIN_SELECTOR: &str = "1b44fd15";

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
                let (selector, argument_bytes) = input.split_at(4);
                let argument = U256::from_big_endian(argument_bytes);

                if selector
                    == decode(SET_BLOCK_SELECTOR).expect("Error decoding set_block function call")
                {
                    info!(
                        "Travel contract called with function: setBlock and argument: {:?}!",
                        argument
                    );
                    self.set_block = Some(argument);
                } else if selector
                    == decode(SET_CHAIN_SELECTOR).expect("Error decoding set_chain function call")
                {
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
