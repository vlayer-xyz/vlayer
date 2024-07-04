use alloy_primitives::address;
use revm::{
    inspectors::GasInspector,
    interpreter::{CallInputs, CallOutcome},
    Database, EvmContext, Inspector,
};

#[derive(Clone, Debug, Default)]
pub struct CustomPrintTracer {
    gas_inspector: GasInspector,
    set_block: bool,
}

impl<DB: Database> Inspector<DB> for CustomPrintTracer {
    fn call(
        &mut self,
        _context: &mut EvmContext<DB>,
        inputs: &mut CallInputs,
    ) -> Option<CallOutcome> {
        println!(
            "SM Address: {:?}, caller:{:?},target:{:?} is_static:{:?}, transfer:{:?}, input_size:{:?}",
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

        if inputs.bytecode_address == address!("1234567890AbcdEF1234567890aBcdef12345678") {
            self.set_block = true;
        } else {
            self.set_block = false;
        }
        None
    }

    fn call_end(
        &mut self,
        context: &mut EvmContext<DB>,
        inputs: &CallInputs,
        outcome: CallOutcome,
    ) -> CallOutcome {
        self.gas_inspector.call_end(context, inputs, outcome)
    }
}
