/**
 * This file is in large part copied from https://github.com/foundry-rs/foundry/blob/65b3cb031336bccbfe7c32c26b8869d1b8654f68/crates/evm/evm/src/executors/mod.rs
 * The original file is licensed under the Apache License, Version 2.0.
 * The original file was modified for the purpose of this project.
 * All relevant modifications are commented with "MODIFICATION" comments.
 */
use alloy_dyn_abi::DynSolValue;
use alloy_dyn_abi::JsonAbiExt;
use alloy_json_abi::Function;

use crate::composite_inspector::CompositeInspector;
use alloy_sol_types::private::{Address, Bytes, U256};
use color_eyre::eyre;
use forge::revm;
use forge::revm::interpreter::{return_ok, InstructionResult};
use forge::revm::primitives::{
    BlockEnv, Env, EnvWithHandlerCfg, ExecutionResult, Output, ResultAndState, TxEnv, TxKind,
};
use forge::traces::TraceMode;
use foundry_evm::executors::{CallResult, DeployResult, EvmError, Executor, RawCallResult};
use foundry_evm::inspectors::{InspectorData, InspectorStack};
use foundry_evm_core::backend::{BackendResult, CowBackend};
use foundry_evm_core::decode::RevertDecoder;
use tracing::instrument;
use vlayer_engine::inspector::SetInspector;

/// MODIFICATION: This struct is a wrapper around the Executor struct from foundry_evm that adds our inspector that will be passed to the backend
#[derive(Clone, Debug)]
pub struct TestExecutor {
    pub inspector: SetInspector,
    pub executor: Executor,
}

impl TestExecutor {
    pub fn new(executor: Executor, inspector: SetInspector) -> Self {
        Self {
            inspector,
            executor,
        }
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
        let mut backend = CowBackend::new_borrowed(self.executor.backend());
        let mut composite_inspector =
            CompositeInspector::new(self.inspector.clone(), self.executor.inspector().clone());
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
            cfg: self.executor.env().cfg.clone(),
            block: BlockEnv {
                basefee: U256::ZERO,
                ..self.executor.env().block.clone()
            },
            tx: TxEnv {
                caller,
                transact_to,
                data,
                value,
                gas_price: U256::ZERO,
                gas_priority_fee: None,
                ..self.executor.env().tx.clone()
            },
        };

        EnvWithHandlerCfg::new_with_spec_id(Box::new(env), self.executor.spec_id())
    }

    pub fn is_raw_call_mut_success(
        &self,
        address: Address,
        call_result: &mut RawCallResult,
        should_fail: bool,
    ) -> bool {
        self.executor
            .is_raw_call_mut_success(address, call_result, should_fail)
    }
    pub fn set_tracing(&mut self, mode: TraceMode) {
        self.executor = self.executor.set_tracing(mode).clone();
    }
    pub fn inspector_mut(&mut self) -> &mut InspectorStack {
        self.executor.inspector_mut()
    }

    pub fn setup(
        &mut self,
        from: Option<Address>,
        to: Address,
        rd: Option<&RevertDecoder>,
    ) -> Result<RawCallResult, EvmError> {
        self.executor.setup(from, to, rd)
    }

    pub fn deploy_create2_deployer(&mut self) -> eyre::Result<()> {
        self.executor.deploy_create2_deployer()
    }

    pub fn get_nonce(&self, address: Address) -> BackendResult<u64> {
        self.executor.get_nonce(address)
    }

    pub fn deploy(
        &mut self,
        from: Address,
        code: Bytes,
        value: U256,
        rd: Option<&RevertDecoder>,
    ) -> Result<DeployResult, EvmError> {
        self.executor.deploy(from, code, value, rd)
    }

    pub fn set_balance(&mut self, address: Address, amount: U256) -> BackendResult<()> {
        self.executor.set_balance(address, amount)
    }

    pub fn set_nonce(&mut self, address: Address, nonce: u64) -> BackendResult<()> {
        self.executor.set_nonce(address, nonce)
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
