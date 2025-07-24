use call_common::Metadata;
use call_engine::{Call as EngineCall, CallGuestId};
use call_host::{
    CycleEstimator, CycleEstimatorError, Host, PreflightResult, Prover, ProvingInput,
    Risc0CycleEstimator,
};
use dashmap::Entry;
use tracing::{error, info, instrument, warn};

pub use crate::proving::RawData;
use crate::{
    gas_meter::{Client as GasMeterClient, ComputationStage, Error as GasMeterError},
    handlers::State as AppState,
    metrics::Metrics,
    preflight::{self, Error as PreflightError},
    proving::{self, Error as ProvingError},
    state::{State, set_state},
    v_call::CallHash,
};

const CYCLES_PER_VGAS: u64 = 1_000_000;

#[derive(Default)]
pub struct Status {
    pub state: State,
    pub metrics: Metrics,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Allocating gas: {0}")]
    AllocateGasRpc(#[from] GasMeterError),
    #[error("Your vgas balance is insufficient to allocate given vgas_limit of {vgas_limit}.")]
    AllocateGasInsufficientBalance { vgas_limit: u64 },
    #[error("Preflight: {0}")]
    Preflight(#[from] PreflightError),
    #[error("EVM gas limit {evm_gas_limit} exceeded.")]
    PreflightEvmGasLimitExceeded { evm_gas_limit: u64 },
    #[error("Estimating cycles: {0}")]
    EstimatingCycles(#[from] CycleEstimatorError),
    #[error("Insufficient vgas_limit: provided {provided}, estimated vgas: {estimated}")]
    InsufficientVgas { provided: u64, estimated: u64 },
    #[error("Proving: {0}")]
    Proving(#[from] ProvingError),
}

pub struct Generator {
    gas_meter_client: Box<dyn GasMeterClient>,
    vgas_limit: u64,
    app_state: AppState,
    call_hash: CallHash,
    metrics: Metrics,
}

impl Generator {
    pub fn new(
        gas_meter_client: Box<dyn GasMeterClient>,
        vgas_limit: u64,
        app_state: AppState,
        call_hash: CallHash,
    ) -> Self {
        Self {
            gas_meter_client,
            vgas_limit,
            app_state,
            call_hash,
            metrics: Metrics::default(),
        }
    }

    #[instrument(name = "proof", skip_all, fields(hash = %self.call_hash))]
    pub async fn run(mut self, host: Host, call: EngineCall) {
        info!("Generating proof");
        let prover = host.prover();
        let call_guest_id = host.call_guest_id();

        if !&self.allocate_vgas().await {
            return;
        }
        let Some(preflight_result) = self.preflight(host, call).await else {
            return;
        };
        let Some(estimated_cycles) = self.estimate_cycles(&preflight_result) else {
            return;
        };
        let estimated_vgas = to_vgas(estimated_cycles);
        self.metrics.gas = estimated_vgas;
        if !self.refund(estimated_vgas).await {
            return;
        }
        if !self.send_metadata(preflight_result.metadata.clone()).await {
            return;
        }
        if !self.validate_vgas_limit(estimated_vgas, estimated_cycles) {
            return;
        }
        self.proving(preflight_result, &prover, call_guest_id, estimated_vgas)
            .await;
    }

    async fn allocate_vgas(&self) -> bool {
        set_state(&self.app_state, self.call_hash, State::AllocateGasPending);

        match self.gas_meter_client.allocate(self.vgas_limit).await {
            Ok(()) => {
                set_state(&self.app_state, self.call_hash, State::PreflightPending);
                true
            }
            Err(err) => {
                let state = allocate_error_to_state(err, self.vgas_limit);
                set_state(&self.app_state, self.call_hash, state);
                false
            }
        }
    }

    async fn preflight(&mut self, host: Host, call: EngineCall) -> Option<PreflightResult> {
        let evm_gas_limit = call.gas_limit;
        match preflight::await_preflight(host, call, &mut self.metrics).await {
            Ok(res) => {
                let entry =
                    set_state(&self.app_state, self.call_hash, State::EstimatingCyclesPending);
                set_metrics(entry, self.metrics);
                Some(res)
            }
            Err(err) => {
                let state = preflight_error_to_state(err, evm_gas_limit);
                let entry = set_state(&self.app_state, self.call_hash, state);
                set_metrics(entry, self.metrics);
                None
            }
        }
    }

    fn estimate_cycles(&mut self, preflight_result: &PreflightResult) -> Option<u64> {
        let estimation_start = std::time::Instant::now();

        let estimated_cycles = match Risc0CycleEstimator
            .estimate(&preflight_result.input, preflight_result.guest_elf.clone())
        {
            Ok(result) => {
                info!(estimated_cycles = result, "Cycle estimation");
                result
            }
            Err(err) => {
                error!("Cycle estimation failed with error: {err}");
                let entry = set_state(
                    &self.app_state,
                    self.call_hash,
                    State::EstimatingCyclesError(Box::new(Error::EstimatingCycles(err))),
                );
                set_metrics(entry, self.metrics);
                return None;
            }
        };

        let elapsed = estimation_start.elapsed();
        info!(estimating_cycles_elapsed_time = ?elapsed, "Cycle estimation lasted");

        Some(estimated_cycles)
    }

    async fn refund(&self, estimated_vgas: u64) -> bool {
        match self
            .gas_meter_client
            .refund(ComputationStage::Preflight, estimated_vgas)
            .await
        {
            Ok(()) => {
                info!("Preflight refund succeeded for {estimated_vgas} vgas");
                true
            }
            Err(err) => {
                error!("Preflight refund failed with error: {err}");
                let entry = set_state(
                    &self.app_state,
                    self.call_hash,
                    State::PreflightError(Error::AllocateGasRpc(err).into()),
                );
                set_metrics(entry, self.metrics);
                false
            }
        }
    }

    async fn send_metadata(&self, metadata: Box<[Metadata]>) -> bool {
        match self.gas_meter_client.send_metadata(metadata).await {
            Ok(()) => {
                info!("Send metadata succeeded");
                true
            }
            Err(err) => {
                error!("Send metadata failed with error: {err}");
                let entry = set_state(
                    &self.app_state,
                    self.call_hash,
                    State::PreflightError(Error::AllocateGasRpc(err).into()),
                );
                set_metrics(entry, self.metrics);
                false
            }
        }
    }

    fn validate_vgas_limit(&self, estimated_vgas: u64, estimated_cycles: u64) -> bool {
        if self.vgas_limit <= estimated_vgas {
            let cycles_limit = to_cycles(self.vgas_limit);
            warn!(
                "Insufficient vgas_limit: provided {} vgas ({} cycles), estimated vgas: {} ({} cycles)",
                self.vgas_limit, cycles_limit, estimated_vgas, estimated_cycles
            );
            let entry = set_state(
                &self.app_state,
                self.call_hash,
                State::EstimatingCyclesError(Box::new(Error::InsufficientVgas {
                    provided: self.vgas_limit,
                    estimated: estimated_vgas,
                })),
            );
            set_metrics(entry, self.metrics);
            false
        } else {
            true
        }
    }

    async fn proving(
        &mut self,
        preflight_result: PreflightResult,
        prover: &Prover,
        call_guest_id: CallGuestId,
        estimated_vgas: u64,
    ) {
        set_state(&self.app_state, self.call_hash, State::ProvingPending);

        let proving_input = ProvingInput::new(preflight_result.host_output, preflight_result.input);
        match proving::await_proving(
            prover,
            call_guest_id,
            proving_input,
            &self.gas_meter_client,
            &mut self.metrics,
            estimated_vgas,
        )
        .await
        .map_err(Error::Proving)
        {
            Ok(raw_data) => {
                let entry =
                    set_state(&self.app_state, self.call_hash, State::Done(raw_data.into()));
                set_metrics(entry, self.metrics);
            }
            Err(err) => {
                error!("Proving failed with error: {err}");
                let entry =
                    set_state(&self.app_state, self.call_hash, State::ProvingError(err.into()));
                set_metrics(entry, self.metrics);
            }
        };
    }
}

fn set_metrics(
    entry: Entry<'_, CallHash, Status>,
    metrics: Metrics,
) -> Entry<'_, CallHash, Status> {
    entry.and_modify(|res| res.metrics = metrics)
}

fn allocate_error_to_state(err: GasMeterError, vgas_limit: u64) -> State {
    if err.is_insufficient_gas_balance() {
        return State::AllocateGasError(
            Error::AllocateGasInsufficientBalance { vgas_limit }.into(),
        );
    }
    error!("Gas meter failed with error: {err}");
    State::AllocateGasError(Error::AllocateGasRpc(err).into())
}

fn preflight_error_to_state(err: PreflightError, evm_gas_limit: u64) -> State {
    if let preflight::Error::Preflight(ref preflight_err) = err {
        if preflight_err.is_gas_limit_exceeded() {
            error!("Preflight gas limit exceeded!");
            return State::PreflightError(
                Error::PreflightEvmGasLimitExceeded { evm_gas_limit }.into(),
            );
        }
    }

    error!("Preflight failed with error: {err}");
    State::PreflightError(Error::Preflight(err).into())
}

const fn to_vgas(cycles: u64) -> u64 {
    cycles.div_ceil(CYCLES_PER_VGAS)
}

const fn to_cycles(vgas: u64) -> u64 {
    vgas * CYCLES_PER_VGAS
}
