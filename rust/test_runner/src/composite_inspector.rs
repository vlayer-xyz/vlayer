use alloy_sol_types::private::{Address, U256};
use delegate::delegate;
use derive_new::new;
use forge::revm::{
    Database, EvmContext, Inspector,
    interpreter::{
        CallInputs, CallOutcome, CreateInputs, CreateOutcome, Gas, InstructionResult, Interpreter,
        InterpreterResult,
    },
    precompile::{Log, PrecompileWithAddress},
    primitives::Env,
};
use foundry_evm::{inspectors::InspectorStack, revm::primitives::PrecompileOutput};
use foundry_evm_core::{InspectorExt, backend::DatabaseExt};

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
        if let Some(value) = find_and_call_precompiles(context, inputs) {
            return Some(value);
        }
        inspector_stack_outcome
    }
}

fn find_and_call_precompiles<DB: Database + DatabaseExt>(
    context: &EvmContext<DB>,
    inputs: &CallInputs,
) -> Option<CallOutcome> {
    call_precompiles::precompiles(true)
        .iter()
        .find(|precompile| precompile.address() == &inputs.target_address)
        .and_then(|precompile| call_precompile(precompile, inputs, &context.env))
}

fn call_precompile(
    precompile: &PrecompileWithAddress,
    inputs: &CallInputs,
    env: &Env,
) -> Option<CallOutcome> {
    precompile
        .precompile()
        .call_ref(&inputs.input, u64::MAX, env)
        .map(|outcome| to_call_outcome(outcome, inputs))
        .ok()
}

fn to_call_outcome(precompile_outcome: PrecompileOutput, inputs: &CallInputs) -> CallOutcome {
    CallOutcome::new(
        InterpreterResult::new(
            InstructionResult::Return,
            precompile_outcome.bytes,
            Gas::new(inputs.gas_limit),
        ),
        inputs.return_memory_offset.clone(),
    )
}

impl InspectorExt for CompositeInspector {}
