use revm::{
    inspectors::GasInspector,
    interpreter::{CallInputs, CallOutcome},
    Database, EvmContext, Inspector,
};

#[derive(Clone, Debug, Default)]
pub struct CustomPrintTracer {
    gas_inspector: GasInspector,
}

impl<DB: Database> Inspector<DB> for CustomPrintTracer {
    fn call(
        &mut self,
        _context: &mut EvmContext<DB>,
        inputs: &mut CallInputs,
    ) -> Option<CallOutcome> {
        println!("Hello spike!");
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
