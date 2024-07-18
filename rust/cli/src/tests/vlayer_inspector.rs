use forge::revm::interpreter::{CallInputs, CallOutcome};
use forge::revm::{Database, EvmContext, Inspector};
use foundry_evm::inspectors::InspectorStack;
use foundry_evm_core::backend::DatabaseExt;
use foundry_evm_core::InspectorExt;
use vlayer_engine::inspector::SetInspector;

pub struct VlayerTestInspector {
    pub set_inspector: SetInspector,
    pub inspector_stack: InspectorStack,
}

impl<DB: Database + DatabaseExt> Inspector<DB> for VlayerTestInspector {
    fn call(
        &mut self,
        context: &mut EvmContext<DB>,
        inputs: &mut CallInputs,
    ) -> Option<CallOutcome> {
        if let Some(call_outcome) = self.set_inspector.call(context, inputs) {
            return Some(call_outcome);
        }
        self.inspector_stack.call(context, inputs)
    }
}

impl<DB: Database + DatabaseExt> InspectorExt<DB> for VlayerTestInspector {}
