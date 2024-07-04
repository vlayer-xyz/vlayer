use alloy_primitives::{address, Address};
use revm::{
    interpreter::{CallInputs, CallOutcome},
    Database, EvmContext, Inspector,
};
use tracing::info;

const SET_BLOCK_CONTRACT_ADDR: Address = address!("1234567890AbcdEF1234567890aBcdef12345678");

#[derive(Clone, Debug, Default)]
pub struct SetBlockInspector {
    set_block: bool,
}

impl<DB: Database> Inspector<DB> for SetBlockInspector {
    fn call(
        &mut self,
        _context: &mut EvmContext<DB>,
        inputs: &mut CallInputs,
    ) -> Option<CallOutcome> {
        info!(
            "Address: {:?}, caller:{:?}, input:{:?}",
            inputs.bytecode_address, inputs.caller, inputs.input,
        );

        if self.set_block {
            info!("Need to change block!");
        }
        self.set_block = inputs.bytecode_address == SET_BLOCK_CONTRACT_ADDR;

        None
    }
}
