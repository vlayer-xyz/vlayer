use alloy_primitives::{address, Address};
use revm::{
    interpreter::{CallInputs, CallOutcome},
    Database, EvmContext, Inspector,
};

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
        println!(
            "Address: {:?}, caller:{:?},target:{:?} is_static:{:?}, transfer:{:?}, input_size:{:?}",
            inputs.bytecode_address,
            inputs.caller,
            inputs.target_address,
            inputs.is_static,
            inputs.value,
            inputs.input.len(),
        );

        if self.set_block {
            println!("Need to change block!");
        }
        self.set_block = inputs.bytecode_address == SET_BLOCK_CONTRACT_ADDR;

        None
    }
}
