use alloy_primitives::hex::decode;
use alloy_primitives::{address, Address};
use ethers_core::types::U256;
use revm::{
    interpreter::{CallInputs, CallOutcome},
    Database, EvmContext, Inspector,
};
use tracing::info;

const TRAVEL_CONTRACT_ADDR: Address = address!("1234567890AbcdEF1234567890aBcdef12345678");
const SET_BLOCK_FUNCTION: &str = "768d130e";
const SET_CHAIN_FUNCTION: &str = "5852cc0c";

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

        if inputs.bytecode_address == TRAVEL_CONTRACT_ADDR {
            let input = &inputs.input;
            let function = &input[..4];
            let argument = U256::from_big_endian(&input[4..]);

            if function == decode(SET_BLOCK_FUNCTION).unwrap() {
                info!(
                    "Travel contract called with function: setBlock and argument: {:?}!",
                    argument
                );
                self.set_block = Some(argument);
            } else if function == decode(SET_CHAIN_FUNCTION).unwrap() {
                info!(
                    "Travel contract called with function: setChain and argument: {:?}!",
                    argument
                );
                self.set_chain = Some(argument);
            }
        } else {
            match &self.set_block {
                Some(number) => {
                    info!("Need to change block to {:?}!", number);
                }
                None => (),
            };

            match &self.set_chain {
                Some(number) => {
                    info!("Need to change chain to {:?}!", number);
                }
                None => (),
            };

            self.set_block = None;
            self.set_chain = None;
        }

        None
    }
}
