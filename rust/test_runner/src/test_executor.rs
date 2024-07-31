/**
 * This file is in large part copied from https://github.com/foundry-rs/foundry/blob/65b3cb031336bccbfe7c32c26b8869d1b8654f68/crates/evm/evm/src/executors/mod.rs
 * The original file is licensed under the Apache License, Version 2.0.
 * The original file was modified for the purpose of this project.
 * All relevant modifications are commented with "MODIFICATION" comments.
 */
use alloy_dyn_abi::DynSolValue;
use alloy_dyn_abi::JsonAbiExt;
use alloy_json_abi::Function;
use alloy_sol_types::private::{Address, Bytes, U256};
use color_eyre::eyre;
use forge::revm;
use forge::revm::interpreter::{return_ok, InstructionResult};
use forge::revm::primitives::{
    BlockEnv, Env, EnvWithHandlerCfg, ExecutionResult, Output, ResultAndState, TxEnv, TxKind,
};
use foundry_evm::executors::{CallResult, EvmError, Executor, RawCallResult};
use foundry_evm::inspectors::{InspectorData, InspectorStack};
use foundry_evm_core::backend::CowBackend;
use foundry_evm_core::decode::RevertDecoder;
use std::ops::{Deref, DerefMut};
use tracing::instrument;

use crate::cheatcode_inspector::CheatcodeInspector;
use crate::composite_inspector::CompositeInspector;

/// MODIFICATION: This struct is a wrapper around the Executor struct from foundry_evm that adds our inspector that will be passed to the backend
pub struct TestExecutor {
    pub inner: Executor,
}

impl Deref for TestExecutor {
    type Target = Executor;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for TestExecutor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl TestExecutor {
    pub fn new(inner: Executor) -> Self {
        Self { inner }
    }

    pub fn call(
        &self,
        from: Address,
        to: Address,
        func: &Function,
        args: &[DynSolValue],
        value: U256,
        rd: Option<&RevertDecoder>,
    ) -> Result<CallResult, EvmError> {
        let calldata = Bytes::from(func.abi_encode_input(args)?);
        let result = self.call_raw(from, to, calldata, value)?;
        result.into_decoded_result(func, rd)
    }

    pub fn call_raw(
        &self,
        from: Address,
        to: Address,
        calldata: Bytes,
        value: U256,
    ) -> eyre::Result<RawCallResult> {
        let env = self.build_test_env(from, TxKind::Call(to), calldata, value);
        self.call_with_env(env)
    }

    // MODIFICATION: Pass CompositeInspector instead of InspectorStack to the backend
    #[instrument(name = "call", level = "debug", skip_all)]
    pub fn call_with_env(&self, mut env: EnvWithHandlerCfg) -> eyre::Result<RawCallResult> {
        let mut backend = CowBackend::new_borrowed(self.backend());
        let mut composite_inspector =
            CompositeInspector::new(self.inspector().clone(), CheatcodeInspector::new());
        let result = backend.inspect(&mut env, &mut composite_inspector)?;
        convert_executed_result(
            env,
            composite_inspector.inspector_stack,
            result,
            backend.has_snapshot_failure(),
        )
    }

    fn build_test_env(
        &self,
        caller: Address,
        transact_to: TxKind,
        data: Bytes,
        value: U256,
    ) -> EnvWithHandlerCfg {
        let env = Env {
            cfg: self.env().cfg.clone(),
            block: BlockEnv {
                basefee: U256::ZERO,
                ..self.env().block.clone()
            },
            tx: TxEnv {
                caller,
                transact_to,
                data,
                value,
                gas_price: U256::ZERO,
                gas_priority_fee: None,
                ..self.env().tx.clone()
            },
        };

        EnvWithHandlerCfg::new_with_spec_id(Box::new(env), self.spec_id())
    }
}

fn convert_executed_result(
    env: EnvWithHandlerCfg,
    inspector: InspectorStack,
    ResultAndState { result, state }: ResultAndState,
    has_snapshot_failure: bool,
) -> eyre::Result<RawCallResult> {
    let (exit_reason, gas_refunded, gas_used, out, exec_logs) = match result {
        ExecutionResult::Success {
            reason,
            gas_used,
            gas_refunded,
            output,
            logs,
            ..
        } => (reason.into(), gas_refunded, gas_used, Some(output), logs),
        ExecutionResult::Revert { gas_used, output } => {
            // Need to fetch the unused gas
            (
                InstructionResult::Revert,
                0_u64,
                gas_used,
                Some(Output::Call(output)),
                vec![],
            )
        }
        ExecutionResult::Halt { reason, gas_used } => {
            (reason.into(), 0_u64, gas_used, None, vec![])
        }
    };
    let stipend = revm::interpreter::gas::validate_initial_tx_gas(
        env.spec_id(),
        &env.tx.data,
        env.tx.transact_to.is_create(),
        &env.tx.access_list,
    );

    let result = match &out {
        Some(Output::Call(data)) => data.clone(),
        _ => Bytes::new(),
    };

    let InspectorData {
        mut logs,
        labels,
        traces,
        coverage,
        cheatcodes,
        chisel_state,
    } = inspector.collect();

    if logs.is_empty() {
        logs = exec_logs;
    }

    let transactions = cheatcodes
        .as_ref()
        .map(|c| c.broadcastable_transactions.clone())
        .filter(|txs| !txs.is_empty());

    Ok(RawCallResult {
        exit_reason,
        reverted: !matches!(exit_reason, return_ok!()),
        has_snapshot_failure,
        result,
        gas_used,
        gas_refunded,
        stipend,
        logs,
        labels,
        traces,
        coverage,
        transactions,
        state_changeset: state,
        env,
        cheatcodes,
        out,
        chisel_state,
    })
}
