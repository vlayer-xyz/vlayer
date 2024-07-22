use alloy_sol_types::private::{Address, U256};
use forge::revm::interpreter::{CallInputs, CallOutcome, CreateInputs, CreateOutcome, Interpreter};
use forge::revm::precompile::Log;
use forge::revm::{Database, EvmContext, Inspector};
use foundry_evm::inspectors::InspectorStack;
use foundry_evm_core::backend::DatabaseExt;
use foundry_evm_core::InspectorExt;
use vlayer_engine::inspector::TravelInspector;

pub struct CompositeInspector {
    pub set_inspector: TravelInspector,
    pub inspector_stack: InspectorStack,
}

impl CompositeInspector {
    pub fn new(set_inspector: TravelInspector, inspector_stack: InspectorStack) -> Self {
        Self {
            set_inspector,
            inspector_stack,
        }
    }
}

impl<DB: Database + DatabaseExt> Inspector<DB> for CompositeInspector {
    fn call(
        &mut self,
        context: &mut EvmContext<DB>,
        inputs: &mut CallInputs,
    ) -> Option<CallOutcome> {
        let inspector_stack_outcome = self.inspector_stack.call(context, inputs);
        if let Some(call_outcome) = self.set_inspector.call(context, inputs) {
            return Some(call_outcome);
        }
        inspector_stack_outcome
    }

    fn call_end(
        &mut self,
        ecx: &mut EvmContext<DB>,
        inputs: &CallInputs,
        outcome: CallOutcome,
    ) -> CallOutcome {
        self.inspector_stack.call_end(ecx, inputs, outcome)
    }

    #[inline]
    fn step(&mut self, interpreter: &mut Interpreter, ecx: &mut EvmContext<DB>) {
        self.inspector_stack.step(interpreter, ecx)
    }

    #[inline]
    fn step_end(&mut self, interpreter: &mut Interpreter, ecx: &mut EvmContext<DB>) {
        self.inspector_stack.step_end(interpreter, ecx)
    }

    fn create(
        &mut self,
        context: &mut EvmContext<DB>,
        create: &mut CreateInputs,
    ) -> Option<CreateOutcome> {
        self.inspector_stack.create(context, create)
    }

    fn create_end(
        &mut self,
        context: &mut EvmContext<DB>,
        call: &CreateInputs,
        outcome: CreateOutcome,
    ) -> CreateOutcome {
        self.inspector_stack.create_end(context, call, outcome)
    }

    fn initialize_interp(&mut self, interpreter: &mut Interpreter, ecx: &mut EvmContext<DB>) {
        self.inspector_stack.initialize_interp(interpreter, ecx)
    }

    fn log(&mut self, ecx: &mut EvmContext<DB>, log: &Log) {
        self.inspector_stack.log(ecx, log)
    }

    fn selfdestruct(&mut self, contract: Address, target: Address, value: U256) {
        <InspectorStack as Inspector<DB>>::selfdestruct(
            &mut self.inspector_stack,
            contract,
            target,
            value,
        );
    }
}

impl<DB: Database + DatabaseExt> InspectorExt<DB> for CompositeInspector {}
