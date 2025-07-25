use call_common::Metadata;
use call_engine::{Call as EngineCall, CallGuestId};
use call_host::{CycleEstimator, Host, PreflightResult, Prover, ProvingInput, Risc0CycleEstimator};
use tracing::{error, info, instrument, warn};

use crate::{
    gas_meter::{Client as GasMeterClient, ComputationStage},
    handlers::State as AppState,
    metrics::Metrics,
    preflight::{self},
    proof::{
        Error, allocate_error_to_state, preflight_error_to_state, set_metrics,
        state::{State, set_state},
        to_cycles, to_vgas,
    },
    proving::{self},
    v_call::CallHash,
};

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

        match self.run_pipeline(host, call).await {
            Ok(()) => {
                info!("Proof generation completed successfully");
            }
            Err(_) => {
                warn!("Proof generation failed");
            }
        }
    }

    async fn run_pipeline(&mut self, host: Host, call: EngineCall) -> Result<(), ()> {
        let prover = host.prover();
        let call_guest_id = host.call_guest_id();

        self.allocate_vgas().await?;
        let preflight_result = self.preflight(host, call).await?;
        let estimated_cycles = self.estimate_cycles(&preflight_result)?;
        let estimated_vgas = to_vgas(estimated_cycles);
        self.metrics.gas = estimated_vgas;
        self.refund(estimated_vgas).await?;
        self.send_metadata(preflight_result.metadata.clone())
            .await?;
        self.validate_vgas_limit(estimated_cycles)?;
        self.proving(preflight_result, &prover, call_guest_id, estimated_vgas)
            .await;
        Ok(())
    }

    async fn allocate_vgas(&self) -> Result<(), ()> {
        set_state(&self.app_state, self.call_hash, State::AllocateGasPending);

        match self.gas_meter_client.allocate(self.vgas_limit).await {
            Ok(()) => {
                set_state(&self.app_state, self.call_hash, State::PreflightPending);
                Ok(())
            }
            Err(err) => {
                let state = allocate_error_to_state(err, self.vgas_limit);
                set_state(&self.app_state, self.call_hash, state);
                Err(())
            }
        }
    }

    async fn preflight(&mut self, host: Host, call: EngineCall) -> Result<PreflightResult, ()> {
        let evm_gas_limit = call.gas_limit;
        match preflight::await_preflight(host, call, &mut self.metrics).await {
            Ok(res) => {
                let entry =
                    set_state(&self.app_state, self.call_hash, State::EstimatingCyclesPending);
                set_metrics(entry, self.metrics);
                Ok(res)
            }
            Err(err) => {
                let state = preflight_error_to_state(err, evm_gas_limit);
                let entry = set_state(&self.app_state, self.call_hash, state);
                set_metrics(entry, self.metrics);
                Err(())
            }
        }
    }

    fn estimate_cycles(&self, preflight_result: &PreflightResult) -> Result<u64, ()> {
        let estimation_start = std::time::Instant::now();

        let estimated_cycles = match Risc0CycleEstimator
            .estimate(&preflight_result.input, preflight_result.guest_elf.clone())
        {
            Ok(result) => {
                info!(estimated_cycles = result, "Cycle estimation");
                Some(result)
            }
            Err(err) => {
                error!("Cycle estimation failed with error: {err}");
                let entry = set_state(
                    &self.app_state,
                    self.call_hash,
                    State::EstimatingCyclesError(Box::new(Error::EstimatingCycles(err))),
                );
                set_metrics(entry, self.metrics);
                None
            }
        };

        let elapsed = estimation_start.elapsed();
        info!(estimating_cycles_elapsed_time = ?elapsed, "Cycle estimation lasted");

        estimated_cycles.ok_or(())
    }

    async fn refund(&self, estimated_vgas: u64) -> Result<(), ()> {
        match self
            .gas_meter_client
            .refund(ComputationStage::Preflight, estimated_vgas)
            .await
        {
            Ok(()) => {
                info!("Preflight refund succeeded for {estimated_vgas} vgas");
                Ok(())
            }
            Err(err) => {
                error!("Preflight refund failed with error: {err}");
                let entry = set_state(
                    &self.app_state,
                    self.call_hash,
                    State::PreflightError(Error::AllocateGasRpc(err).into()),
                );
                set_metrics(entry, self.metrics);
                Err(())
            }
        }
    }

    async fn send_metadata(&self, metadata: Box<[Metadata]>) -> Result<(), ()> {
        match self.gas_meter_client.send_metadata(metadata).await {
            Ok(()) => {
                info!("Send metadata succeeded");
                Ok(())
            }
            Err(err) => {
                error!("Send metadata failed with error: {err}");
                let entry = set_state(
                    &self.app_state,
                    self.call_hash,
                    State::PreflightError(Error::AllocateGasRpc(err).into()),
                );
                set_metrics(entry, self.metrics);
                Err(())
            }
        }
    }

    fn validate_vgas_limit(&self, estimated_cycles: u64) -> Result<(), ()> {
        let estimated_vgas = to_vgas(estimated_cycles);
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
            Err(())
        } else {
            Ok(())
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
