use alloy_sol_types::private::{Address, U256};
use delegate::delegate;
use derive_new::new;
use forge::revm::{
    interpreter::{CallInputs, CallOutcome, CreateInputs, CreateOutcome, Interpreter},
    precompile::Log,
    Database, EvmContext, Inspector,
};
use foundry_evm::inspectors::InspectorStack;
use foundry_evm_core::{backend::DatabaseExt, InspectorExt};

use crate::cheatcode_inspector::CheatcodeInspector;

#[derive(new)]
pub struct CompositeInspector {
    pub inspector_stack: InspectorStack,
    pub cheatcode_inspector: CheatcodeInspector,
}

impl<DB: Database + DatabaseExt> Inspector<DB> for CompositeInspector
where
    InspectorStack: Inspector<DB>,
{
    delegate! {
        to self.inspector_stack {
            fn call_end(&mut self, ecx: &mut EvmContext<DB>, inputs: &CallInputs, outcome: CallOutcome) -> CallOutcome;
            fn step(&mut self, interpreter: &mut Interpreter, ecx: &mut EvmContext<DB>);
            fn step_end(&mut self, interpreter: &mut Interpreter, ecx: &mut EvmContext<DB>);
            fn create(&mut self, context: &mut EvmContext<DB>, create: &mut CreateInputs) -> Option<CreateOutcome>;
            fn create_end(&mut self, context: &mut EvmContext<DB>, call: &CreateInputs, outcome: CreateOutcome) -> CreateOutcome;
            fn initialize_interp(&mut self, interpreter: &mut Interpreter, ecx: &mut EvmContext<DB>);
            fn log(&mut self, interpreter: &mut Interpreter, ecx: &mut EvmContext<DB>, log: &Log);
            fn selfdestruct(&mut self, contract: Address, target: Address, value: U256);
        }
    }

    fn call(
        &mut self,
        context: &mut EvmContext<DB>,
        inputs: &mut CallInputs,
    ) -> Option<CallOutcome> {
        let inspector_stack_outcome = self.inspector_stack.call(context, inputs);
        if let Some(call_outcome) = self.cheatcode_inspector.call(context, inputs) {
            return Some(call_outcome);
        }
        inspector_stack_outcome
    }
}

impl InspectorExt for CompositeInspector {}
