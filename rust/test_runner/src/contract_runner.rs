use std::{
    collections::{BTreeMap, HashMap},
    time::Instant,
};

use alloy_dyn_abi::DynSolValue;
use alloy_sol_types::private::alloy_json_abi::Function;
use color_eyre::eyre;
use forge::{
    multi_runner::TestContract,
    result::{SuiteResult, TestResult, TestSetup},
    revm::primitives::{address, Address, Bytes, U256},
};
use foundry_common::{TestFilter, TestFunctionExt, TestFunctionKind};
use foundry_evm::{
    constants::CALLER,
    decode::RevertDecoder,
    executors::{CallResult, EvmError, ExecutionErr, RawCallResult},
    fuzz::{fixture_name, FuzzFixtures},
    traces::{TraceKind, TraceMode},
};
use rayon::prelude::*;
use tracing::{debug, debug_span, enabled, trace};

/**
 * This file is in large part copied from https://github.com/foundry-rs/foundry/blob/6bb5c8ea8dcd00ccbc1811f1175cabed3cb4c116/crates/forge/src/runner.rs
 * The original file is licensed under the Apache License, Version 2.0.
 * The original file was modified for the purpose of this project.
 * All relevant modifications are commented with "MODIFICATION" comments.
 */
use crate::test_executor::TestExecutor;

/// When running tests, we deploy all external libraries present in the project. To avoid additional
/// libraries affecting nonces of senders used in tests, we are using separate address to
/// predeploy libraries.
///
/// `address(uint160(uint256(keccak256("foundry library deployer"))))`
pub const LIBRARY_DEPLOYER: Address = address!("1F95D37F27EA0dEA9C252FC09D5A6eaA97647353");

/// A type that executes all tests of a contract
pub struct ContractRunner<'a> {
    /// The data of the contract.
    pub contract: &'a TestContract,
    /// The libraries that need to be deployed before the contract.
    pub libs_to_deploy: &'a Vec<Bytes>,
    /// The executor used by the runner.
    /// MODIFICATION: Changed the type from Executor to TestExecutor.
    pub executor: TestExecutor<'a>,
    /// Revert decoder. Contains all known errors.
    pub revert_decoder: &'a RevertDecoder,
    /// The initial balance of the test contract.
    pub initial_balance: U256,
    /// The address which will be used as the `from` field in all EVM calls.
    pub sender: Address,
    /// The handle to the tokio runtime.
    pub tokio_handle: &'a tokio::runtime::Handle,
    /// The span of the contract.
    pub span: tracing::Span,
}

impl<'a> ContractRunner<'a> {
    /// Deploys the test contract inside the runner from the sending account, and optionally runs
    /// the `setUp` function on the test contract.
    pub fn setup(&mut self, call_setup: bool) -> TestSetup {
        self._setup(call_setup)
            .unwrap_or_else(|err| TestSetup::failed(err.to_string()))
    }

    fn _setup(&mut self, call_setup: bool) -> eyre::Result<TestSetup> {
        trace!(call_setup, "setting up");

        // We max out their balance so that they can deploy and make calls.
        self.executor.set_balance(self.sender, U256::MAX)?;
        self.executor.set_balance(CALLER, U256::MAX)?;

        // We set the nonce of the deployer accounts to 1 to get the same addresses as DappTools
        self.executor.set_nonce(self.sender, 1)?;

        // Deploy libraries
        self.executor.set_balance(LIBRARY_DEPLOYER, U256::MAX)?;

        let mut logs = Vec::new();
        let mut traces = Vec::with_capacity(self.libs_to_deploy.len());
        for code in self.libs_to_deploy {
            match self.executor.deploy(
                LIBRARY_DEPLOYER,
                code.clone(),
                U256::ZERO,
                Some(self.revert_decoder),
            ) {
                Ok(d) => {
                    logs.extend(d.raw.logs);
                    traces.extend(d.raw.traces.map(|traces| (TraceKind::Deployment, traces)));
                }
                Err(e) => {
                    return Ok(TestSetup::from_evm_error_with(e, logs, traces, Default::default()))
                }
            }
        }

        let address = self.sender.create(self.executor.get_nonce(self.sender)?);

        // Set the contracts initial balance before deployment, so it is available during
        // construction
        self.executor.set_balance(address, self.initial_balance)?;

        // Deploy the test contract
        match self.executor.deploy(
            self.sender,
            self.contract.bytecode.clone(),
            U256::ZERO,
            Some(self.revert_decoder),
        ) {
            Ok(d) => {
                logs.extend(d.raw.logs);
                traces.extend(d.raw.traces.map(|traces| (TraceKind::Deployment, traces)));
                d.address
            }
            Err(e) => {
                return Ok(TestSetup::from_evm_error_with(e, logs, traces, Default::default()))
            }
        };

        // Reset `self.sender`s, `CALLER`s and `LIBRARY_DEPLOYER`'s balance to the initial balance.
        self.executor
            .set_balance(self.sender, self.initial_balance)?;
        self.executor.set_balance(CALLER, self.initial_balance)?;
        self.executor
            .set_balance(LIBRARY_DEPLOYER, self.initial_balance)?;

        self.executor.deploy_create2_deployer()?;

        // Optionally call the `setUp` function
        let result = if call_setup {
            trace!("calling setUp");
            let res = self
                .executor
                .setup(None, address, Some(self.revert_decoder));
            let (setup_logs, setup_traces, labeled_addresses, reason, coverage) = match res {
                Ok(RawCallResult {
                    traces,
                    labels,
                    logs,
                    coverage,
                    ..
                }) => {
                    trace!(%address, "successfully called setUp");
                    (logs, traces, labels, None, coverage)
                }
                Err(EvmError::Execution(err)) => {
                    let ExecutionErr {
                        raw:
                            RawCallResult {
                                traces,
                                labels,
                                logs,
                                coverage,
                                ..
                            },
                        reason,
                    } = *err;
                    (logs, traces, labels, Some(format!("setup failed: {reason}")), coverage)
                }
                Err(err) => {
                    (Vec::new(), None, HashMap::new(), Some(format!("setup failed: {err}")), None)
                }
            };
            traces.extend(setup_traces.map(|traces| (TraceKind::Setup, traces)));
            logs.extend(setup_logs);

            TestSetup {
                address,
                logs,
                traces,
                labeled_addresses,
                reason,
                coverage,
                fuzz_fixtures: self.fuzz_fixtures(address),
            }
        } else {
            TestSetup::success(
                address,
                logs,
                traces,
                Default::default(),
                None,
                self.fuzz_fixtures(address),
            )
        };

        Ok(result)
    }

    /// Collect fixtures from test contract.
    ///
    /// Fixtures can be defined:
    /// - as storage arrays in test contract, prefixed with `fixture`
    /// - as functions prefixed with `fixture` and followed by parameter name to be fuzzed
    ///
    /// Storage array fixtures:
    /// `uint256[] public fixture_amount = [1, 2, 3];`
    /// define an array of uint256 values to be used for fuzzing `amount` named parameter in scope
    /// of the current test.
    ///
    /// Function fixtures:
    /// `function fixture_owner() public returns (address[] memory){}`
    /// returns an array of addresses to be used for fuzzing `owner` named parameter in scope of the
    /// current test.
    fn fuzz_fixtures(&self, address: Address) -> FuzzFixtures {
        let mut fixtures = HashMap::new();
        let fixture_functions = self
            .contract
            .abi
            .functions()
            .filter(|func| func.is_fixture());
        for func in fixture_functions {
            if func.inputs.is_empty() {
                // Read fixtures declared as functions.
                if let Ok(CallResult { decoded_result, .. }) =
                    self.executor
                        .call(CALLER, address, func, &[], U256::ZERO, None)
                {
                    fixtures.insert(fixture_name(func.name.clone()), decoded_result);
                }
            } else {
                // For reading fixtures from storage arrays we collect values by calling the
                // function with incremented indexes until there's an error.
                let mut vals = Vec::new();
                let mut index = 0;
                loop {
                    if let Ok(CallResult { decoded_result, .. }) = self.executor.call(
                        CALLER,
                        address,
                        func,
                        &[DynSolValue::Uint(U256::from(index), 256)],
                        U256::ZERO,
                        None,
                    ) {
                        vals.push(decoded_result);
                    } else {
                        // No result returned for this index, we reached the end of storage
                        // array or the function is not a valid fixture.
                        break;
                    }
                    index += 1;
                }
                fixtures.insert(fixture_name(func.name.clone()), DynSolValue::Array(vals));
            };
        }
        FuzzFixtures::new(fixtures)
    }

    /// Runs all tests for a contract whose names match the provided regular expression
    pub fn run_tests(mut self, filter: &dyn TestFilter) -> SuiteResult {
        let start = Instant::now();
        let mut warnings = Vec::new();

        // Check if `setUp` function with valid signature declared.
        let setup_fns: Vec<_> = self
            .contract
            .abi
            .functions()
            .filter(|func| func.name.is_setup())
            .collect();
        let call_setup = setup_fns.len() == 1 && setup_fns[0].name == "setUp";
        // There is a single miss-cased `setUp` function, so we add a warning
        for &setup_fn in &setup_fns {
            if setup_fn.name != "setUp" {
                warnings.push(format!(
                    "Found invalid setup function \"{}\" did you mean \"setUp()\"?",
                    setup_fn.signature()
                ));
            }
        }
        // There are multiple setUp function, so we return a single test result for `setUp`
        if setup_fns.len() > 1 {
            return SuiteResult::new(
                start.elapsed(),
                [
                    (
                        "setUp()".to_string(),
                        TestResult::fail("multiple setUp functions".to_string()),
                    ),
                ]
                .into(),
                warnings,
            );
        }

        // Check if `afterInvariant` function with valid signature declared.
        let after_invariant_fns = self
            .contract
            .abi
            .functions()
            .filter(|func| func.name.is_after_invariant())
            .count()
            > 1;
        if after_invariant_fns {
            // Return a single test result failure if multiple functions declared.
            return SuiteResult::new(
                start.elapsed(),
                [(
                    "afterInvariant()".to_string(),
                    TestResult::fail("multiple afterInvariant functions".to_string()),
                )]
                .into(),
                warnings,
            );
        }

        // Invariant testing requires tracing to figure out what contracts were created.
        // We also want to disable `debug` for setup since we won't be using those traces.
        let has_invariants = self
            .contract
            .abi
            .functions()
            .any(TestFunctionExt::is_invariant_test);

        let prev_tracer = self.executor.inspector_mut().tracer.take();
        if prev_tracer.is_some() || has_invariants {
            self.executor.set_tracing(TraceMode::Call);
        }

        let setup_time = Instant::now();
        let setup = self.setup(call_setup);
        debug!("finished setting up in {:?}", setup_time.elapsed());

        self.executor.inspector_mut().tracer = prev_tracer;

        if setup.reason.is_some() {
            // The setup failed, so we return a single test result for `setUp`
            return SuiteResult::new(
                start.elapsed(),
                [("setUp()".to_string(), TestResult::setup_fail(setup))].into(),
                warnings,
            );
        }

        // Filter out functions sequentially since it's very fast and there is no need to do it
        // in parallel.
        let find_timer = Instant::now();
        let functions = self
            .contract
            .abi
            .functions()
            .filter(|func| is_matching_test(func, filter))
            .collect::<Vec<_>>();
        let find_time = find_timer.elapsed();
        debug!(
            "Found {} test functions out of {} in {:?}",
            functions.len(),
            self.contract.abi.functions().count(),
            find_time,
        );

        let test_results = functions
            .par_iter()
            .map(|&func| {
                let start = Instant::now();

                let _guard = self.tokio_handle.enter();

                let _guard;
                let current_span = tracing::Span::current();
                if current_span.is_none() || current_span.id() != self.span.id() {
                    _guard = self.span.enter();
                }

                let sig = func.signature();
                let kind = func.test_function_kind();

                let _guard = debug_span!(
                    "test",
                    %kind,
                    name = %if enabled!(tracing::Level::TRACE) { &sig } else { &func.name },
                )
                .entered();

                let setup = setup.clone();
                let mut res = match kind {
                    TestFunctionKind::UnitTest { should_fail } => {
                        self.run_unit_test(func, should_fail, setup)
                    }
                    TestFunctionKind::FuzzTest { .. } => {
                        unimplemented!("fuzz tests are not supported yet");
                    }
                    TestFunctionKind::InvariantTest => {
                        unimplemented!("invariant tests are not supported yet");
                    }
                    _ => unreachable!(),
                };

                res.duration = start.elapsed();

                (sig, res)
            })
            .collect::<BTreeMap<_, _>>();

        let duration = start.elapsed();
        SuiteResult::new(duration, test_results, warnings)
    }

    /// Runs a single unit test.
    ///
    /// Calls the given functions and returns the `TestResult`.
    ///
    /// State modifications are not committed to the evm database but discarded after the call,
    /// similar to `eth_call`.
    pub fn run_unit_test(
        &self,
        func: &Function,
        should_fail: bool,
        setup: TestSetup,
    ) -> TestResult {
        let address = setup.address;
        let test_result = TestResult::new(setup);

        // Run unit test
        let (mut raw_call_result, reason) = match self.executor.call(
            self.sender,
            address,
            func,
            &[],
            U256::ZERO,
            Some(self.revert_decoder),
        ) {
            Ok(res) => (res.raw, None),
            Err(EvmError::Execution(err)) => (err.raw, Some(err.reason)),
            Err(EvmError::SkipError) => return test_result.single_skip(),
            Err(err) => return test_result.single_fail(err),
        };

        let success =
            self.executor
                .is_raw_call_mut_success(address, &mut raw_call_result, should_fail);
        test_result.single_result(success, reason, raw_call_result)
    }
}

fn is_matching_test(func: &Function, filter: &dyn TestFilter) -> bool {
    func.is_any_test() && filter.matches_test(&func.signature())
}
