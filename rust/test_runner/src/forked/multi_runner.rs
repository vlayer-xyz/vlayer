/**
 * This file is in large part copied from https://github.com/foundry-rs/foundry/blob/1d5fa644df2dd6b141db15bed37d42f8fb7600b3/crates/forge/src/multi_runner.rs
 * The only copied functions are: test, run_test_suite and is_matching_test
 * The original file is licensed under the Apache License, Version 2.0.
 * The original file was modified for the purpose of this project.
 * All relevant modifications are commented with "MODIFICATION" comments.
 */

use std::{
    sync::mpsc,
    time::Instant,
};
use alloy_json_abi::Function;
use forge::{multi_runner::TestContract, result::SuiteResult, MultiContractRunner};
use foundry_common::{get_contract_name, TestFilter, TestFunctionExt};
use foundry_compilers::ArtifactId;
use foundry_evm_core::backend::Backend;
use progress::TestsProgress;
use rayon::prelude::*;
use tracing::{debug, debug_span, enabled, trace};

use crate::forked::{runner::ContractRunner, progress, test_executor::TestExecutor};

/// Executes _all_ tests that match the given `filter`.
///
/// This will create the runtime based on the configured `evm` ops and create the `Backend`
/// before executing all contracts and their tests in _parallel_.
///
/// Each Executor gets its own instance of the `Backend`.
// MODIFICATION: self replaced with runner
pub fn test(
    mut runner: MultiContractRunner,
    filter: &dyn TestFilter,
    tx: mpsc::Sender<(String, SuiteResult)>,
    show_progress: bool,
) {
    let tokio_handle = tokio::runtime::Handle::current();
    trace!("running all tests");

    // MODIFICATION: self replaced with runner
    // The DB backend that serves all the data.
    let db = Backend::spawn(runner.fork.take());

    let find_timer = Instant::now();
    // MODIFICATION: self replaced with runner
    let contracts = runner.matching_contracts(filter).collect::<Vec<_>>();
    let find_time = find_timer.elapsed();
    debug!(
        "Found {} test contracts out of {} in {:?}",
        contracts.len(),
        // MODIFICATION: self replaced with runner
        runner.contracts.len(),
        find_time,
    );

    if show_progress {
        let tests_progress = TestsProgress::new(contracts.len(), rayon::current_num_threads());
        // Collect test suite results to stream at the end of test run.
        let results: Vec<(String, SuiteResult)> = contracts
            .par_iter()
            .map(|&(id, contract)| {
                let _guard = tokio_handle.enter();
                tests_progress.inner.lock().start_suite_progress(&id.identifier());

                // MODIFICATION: self replaced with runner
                let result = run_test_suite(
                    &runner,
                    id,
                    contract,
                    &db,
                    filter,
                    &tokio_handle,
                    Some(&tests_progress),
                );

                tests_progress
                    .inner
                    .lock()
                    .end_suite_progress(&id.identifier(), result.summary());

                (id.identifier(), result)
            })
            .collect();

        tests_progress.inner.lock().clear();

        results.iter().for_each(|result| {
            let _ = tx.send(result.to_owned());
        });
    } else {
        contracts.par_iter().for_each(|&(id, contract)| {
            let _guard = tokio_handle.enter();
            // MODIFICATION: self replaced with runner
            let result = run_test_suite(&runner, id, contract, &db, filter, &tokio_handle, None);
            let _ = tx.send((id.identifier(), result));
        })
    }
}

// MODIFICATION: self replaced with runner
fn run_test_suite(
    runner: &MultiContractRunner,
    artifact_id: &ArtifactId,
    contract: &TestContract,
    db: &Backend,
    filter: &dyn TestFilter,
    tokio_handle: &tokio::runtime::Handle,
    progress: Option<&TestsProgress>,
) -> SuiteResult {
    let identifier = artifact_id.identifier();
    let mut span_name = identifier.as_str();

    if !enabled!(tracing::Level::TRACE) {
        span_name = get_contract_name(&identifier);
    }
    let span = debug_span!("suite", name = %span_name);
    let span_local = span.clone();
    let _guard = span_local.enter();

    debug!("start executing all tests in contract");

    let runner = ContractRunner::new(
        &identifier,
        contract,
        // MODIFICATION: Executor replaced with TestExecutor
        TestExecutor::new(runner.tcfg.executor(runner.known_contracts.clone(), artifact_id, db.clone()), &runner.config.rpc_endpoints),
        progress,
        tokio_handle,
        span,
        runner,
    );
    let r = runner.run_tests(filter);

    debug!(duration=?r.duration, "executed all tests in contract");

    r
}

/// Returns `true` if the function is a test function that matches the given filter.
pub(crate) fn is_matching_test(func: &Function, filter: &dyn TestFilter) -> bool {
    func.is_any_test() && filter.matches_test(&func.signature())
}
