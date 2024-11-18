use std::{
    sync::{mpsc, Arc},
    time::Instant,
};
use alloy_json_abi::Function;
use forge::{multi_runner::TestContract, result::SuiteResult, MultiContractRunner};
use foundry_cheatcodes::CheatsConfig;
use foundry_common::{get_contract_name, TestFilter, TestFunctionExt};
use foundry_compilers::ArtifactId;
use foundry_evm::{executors::ExecutorBuilder, traces::TraceMode};
use foundry_evm_core::backend::Backend;
use progress::TestsProgress;
use rayon::prelude::*;
use tracing::{debug, debug_span, enabled, trace};

use crate::forked::{contract_runner::ContractRunner, progress, test_executor::TestExecutor};

/// Executes _all_ tests that match the given `filter`.
///
/// This will create the runtime based on the configured `evm` ops and create the `Backend`
/// before executing all contracts and their tests in _parallel_.
///
/// Each Executor gets its own instance of the `Backend`.
pub fn test(
    mut runner: MultiContractRunner,
    filter: &dyn TestFilter,
    tx: mpsc::Sender<(String, SuiteResult)>,
    show_progress: bool,
) {
    let tokio_handle = tokio::runtime::Handle::current();
    trace!("running all tests");

    // The DB backend that serves all the data.
    let db = Backend::spawn(runner.fork.take());

    let find_timer = Instant::now();
    let contracts = runner.matching_contracts(filter).collect::<Vec<_>>();
    let find_time = find_timer.elapsed();
    debug!(
        "Found {} test contracts out of {} in {:?}",
        contracts.len(),
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
                tests_progress
                    .inner
                    .lock()
                    .start_suite_progress(&id.identifier());

                let result = run_test_suite(
                    &runner,
                    id,
                    contract,
                    db.clone(),
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
            let result =
                run_test_suite(&runner, id, contract, db.clone(), filter, &tokio_handle, None);
            let _ = tx.send((id.identifier(), result));
        })
    }
}

fn run_test_suite(
    runner: &MultiContractRunner,
    artifact_id: &ArtifactId,
    contract: &TestContract,
    db: Backend,
    filter: &dyn TestFilter,
    tokio_handle: &tokio::runtime::Handle,
    progress: Option<&TestsProgress>,
) -> SuiteResult {
    let identifier = artifact_id.identifier();
    let mut span_name = identifier.as_str();

    let cheats_config = CheatsConfig::new(
        &runner.config,
        runner.evm_opts.clone(),
        Some(runner.known_contracts.clone()),
        Some(artifact_id.name.clone()),
        Some(artifact_id.version.clone()),
    );

    let trace_mode = TraceMode::default()
        .with_debug(runner.debug)
        .with_decode_internal(runner.decode_internal)
        .with_verbosity(runner.evm_opts.verbosity);

    let executor = ExecutorBuilder::new()
        .inspectors(|stack| {
            stack
                .cheatcodes(Arc::new(cheats_config))
                .trace_mode(trace_mode)
                .coverage(runner.coverage)
                .enable_isolation(runner.isolation)
                .alphanet(runner.alphanet)
        })
        .spec(runner.evm_spec)
        .gas_limit(runner.evm_opts.gas_limit())
        .legacy_assertions(runner.config.legacy_assertions)
        .build(runner.env.clone(), db);

    if !enabled!(tracing::Level::TRACE) {
        span_name = get_contract_name(&identifier);
    }
    let span = debug_span!("suite", name = %span_name);
    let span_local = span.clone();
    let _guard = span_local.enter();

    debug!("start executing all tests in contract");

    // MODIFICATION: use our forked ContractRunner
    let contract_runner = ContractRunner {
        name: &identifier,
        contract,
        libs_to_deploy: &runner.libs_to_deploy,
        executor: TestExecutor::new(executor, &runner.config.rpc_endpoints),
        revert_decoder: &runner.revert_decoder,
        initial_balance: runner.evm_opts.initial_balance,
        sender: runner.sender.unwrap_or_default(),
        debug: runner.debug,
        progress,
        tokio_handle,
        span,
    };
    let r = contract_runner.run_tests(filter, &runner.test_options, runner.known_contracts.clone());

    debug!(duration=?r.duration, "executed all tests in contract");

    r
}

/// Returns `true` if the function is a test function that matches the given filter.
pub(crate) fn is_matching_test(func: &Function, filter: &dyn TestFilter) -> bool {
    func.is_any_test() && filter.matches_test(&func.signature())
}
