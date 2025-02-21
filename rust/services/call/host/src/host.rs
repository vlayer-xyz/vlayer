use std::time::{Duration, Instant};

use alloy_sol_types::SolValue;
use bytes::Bytes;
use call_common::{ExecutionLocation, Metadata};
use call_engine::{
    evm::{env::cached::CachedEvmEnv, execution_result::SuccessfulExecutionResult},
    travel_call::Executor as TravelCallExecutor,
    Call, CallGuestId, GuestOutput, HostOutput, Input, Seal,
};
use common::GuestElf;
pub use config::Config;
use derive_new::new;
use error::preflight;
pub use error::{BuilderError, Error, ProvingError};
pub use prover::Prover;
use provider::CachedMultiProvider;
use risc0_zkvm::{ProveInfo, SessionStats};
use seal::EncodableReceipt;
use tracing::instrument;

use crate::{db::HostDb, evm_env::factory::HostEvmEnvFactory, into_input::into_multi_input};

mod builder;
mod config;
pub(crate) mod error;
mod prover;
#[cfg(test)]
mod tests;

pub struct Host {
    start_execution_location: ExecutionLocation,
    envs: CachedEvmEnv<HostDb>,
    prover: Prover,
    guest_elf: GuestElf,
}

impl Host {
    #[must_use]
    pub const fn builder() -> builder::New {
        builder::New
    }

    pub fn prover(&self) -> Prover {
        self.prover.clone()
    }

    pub const fn start_execution_location(&self) -> ExecutionLocation {
        self.start_execution_location
    }

    pub fn call_guest_id(&self) -> CallGuestId {
        self.guest_elf.id.into()
    }
}

#[derive(new, Debug, Clone)]
pub struct PreflightResult {
    pub host_output: Bytes,
    pub input: Input,
    pub gas_used: u64,
    pub elapsed_time: Duration,
    pub metadata: Box<[Metadata]>,
}

#[derive(new, Debug, Clone)]
pub struct ProvingInput {
    pub host_output: Bytes,
    pub input: Input,
}

impl Host {
    pub fn try_new(
        providers: CachedMultiProvider,
        start_execution_location: ExecutionLocation,
        config: Config,
    ) -> Result<Self, crate::BuilderError> {
        let envs = CachedEvmEnv::from_factory(HostEvmEnvFactory::new(providers));
        let prover = Prover::try_new(config.proof_mode, &config.call_guest_elf)?;

        Ok(Host {
            envs,
            start_execution_location,
            prover,
            guest_elf: config.call_guest_elf,
        })
    }

    #[instrument(skip_all)]
    pub async fn preflight(self, call: Call) -> Result<PreflightResult, preflight::Error> {
        let now = Instant::now();

        let SuccessfulExecutionResult {
            output: host_output,
            gas_used,
            metadata,
        } = TravelCallExecutor::new(&self.envs).call(&call, self.start_execution_location)?;
        let input = self.prepare_input_data(call)?;

        let elapsed_time = now.elapsed();
        Ok(PreflightResult::new(
            host_output.into(),
            input,
            gas_used,
            elapsed_time,
            metadata,
        ))
    }

    #[instrument(skip_all)]
    fn prepare_input_data(self, call: Call) -> Result<Input, preflight::Error> {
        let multi_evm_input = into_multi_input(self.envs)?;
        Ok(Input {
            multi_evm_input,
            start_execution_location: self.start_execution_location,
            call,
        })
    }

    #[instrument(skip_all)]
    pub fn prove(
        prover: &Prover,
        call_guest_id: CallGuestId,
        ProvingInput { host_output, input }: ProvingInput,
    ) -> Result<HostOutput, ProvingError> {
        let EncodedProofWithStats {
            seal,
            raw_guest_output,
            stats,
            elapsed_time,
        } = provably_execute(prover, &input)?;
        let proof_len = raw_guest_output.len();
        let guest_output = GuestOutput::from_outputs(&host_output, &raw_guest_output)?;
        let cycles_used = stats.total_cycles;

        if guest_output.evm_call_result != host_output {
            return Err(ProvingError::HostGuestOutputMismatch(
                host_output.into(),
                guest_output.evm_call_result,
            ));
        }

        Ok(HostOutput {
            guest_output,
            seal,
            raw_abi: raw_guest_output,
            proof_len,
            call_guest_id,
            cycles_used,
            elapsed_time,
        })
    }

    pub async fn main(self, call: Call) -> Result<HostOutput, Error> {
        let prover = self.prover();
        let call_guest_id = self.call_guest_id();
        let PreflightResult {
            host_output, input, ..
        } = self.preflight(call).await?;
        Ok(Host::prove(&prover, call_guest_id, ProvingInput::new(host_output, input))?)
    }
}

#[derive(new)]
struct EncodedProofWithStats {
    seal: Bytes,
    raw_guest_output: Bytes,
    stats: SessionStats,
    elapsed_time: Duration,
}

#[instrument(skip_all)]
fn provably_execute(prover: &Prover, input: &Input) -> Result<EncodedProofWithStats, ProvingError> {
    let now = Instant::now();
    let ProveInfo { receipt, stats } = prover.prove(input)?;
    let elapsed_time = now.elapsed();

    let seal: Seal = EncodableReceipt::from(receipt.clone()).try_into()?;
    let seal: Bytes = seal.abi_encode().into();
    let raw_guest_output: Bytes = receipt.journal.bytes.into();

    Ok(EncodedProofWithStats::new(seal, raw_guest_output, stats, elapsed_time))
}
