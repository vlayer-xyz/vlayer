use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use alloy_sol_types::SolValue;
use bytes::Bytes;
use call_common::{ExecutionLocation, Metadata};
use call_engine::{
    Call, CallGuestId, GuestOutput, HostOutput, Input, Seal,
    evm::{env::cached::CachedEvmEnv, execution_result::SuccessfulExecutionResult},
    travel_call::Executor as TravelCallExecutor,
    verifier::{
        teleport, time_travel,
        travel_call::{self, IVerifier},
    },
};
use common::{GuestElf, verifier::zk_proof};
pub use config::Config;
use derive_new::new;
use error::preflight;
pub use error::{BuilderError, Error, ProvingError};
use optimism::client::factory::recording;
pub use prover::Prover;
use provider::CachedMultiProvider;
use risc0_zkvm::{ProveInfo, SessionStats, sha::Digest};
use seal::EncodableReceipt;
use tracing::instrument;

use crate::{HostDb, evm_env::factory::HostEvmEnvFactory, into_input::into_multi_input};

mod builder;
mod config;
pub(crate) mod error;
mod prover;
#[cfg(test)]
mod tests;

type HostTravelCallVerifier = travel_call::Verifier<
    HostDb,
    time_travel::Verifier<
        chain_client::RecordingClient,
        chain_common::verifier::Verifier<zk_proof::HostVerifier>,
    >,
    teleport::Verifier,
>;

pub struct Host {
    start_execution_location: ExecutionLocation,
    envs: CachedEvmEnv<HostDb>,
    prover: Prover,
    // None means that chain service is not available. Therefore Host runs in degrated mode. Time travel and teleport are not available
    chain_client: Option<chain_client::RecordingClient>,
    op_client_factory: recording::Factory,
    travel_call_verifier: HostTravelCallVerifier,
    guest_elf: GuestElf,
    is_vlayer_test: bool,
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
        chain_client: Option<Box<dyn chain_client::Client>>,
        op_client_factory: impl optimism::client::IFactory + 'static,
        config: Config,
    ) -> Result<Self, crate::BuilderError> {
        let envs = CachedEvmEnv::from_factory(HostEvmEnvFactory::new(providers));
        let prover = Prover::try_new(config.proof_mode, &config.call_guest_elf)?;
        let chain_client = chain_client.map(chain_client::RecordingClient::new);
        let recording_op_client_factory = recording::Factory::new(op_client_factory);

        let travel_call_verifier = Host::build_travel_call_verifier(
            config.chain_guest_ids,
            &chain_client,
            recording_op_client_factory.clone(),
        );

        Ok(Host {
            envs,
            start_execution_location,
            prover,
            chain_client,
            op_client_factory: recording_op_client_factory,
            travel_call_verifier,
            guest_elf: config.call_guest_elf,
            is_vlayer_test: config.is_vlayer_test,
        })
    }

    fn build_travel_call_verifier(
        chain_guest_ids: impl IntoIterator<Item = Digest>,
        chain_client: &Option<chain_client::RecordingClient>,
        op_client_factory: optimism::client::factory::recording::Factory,
    ) -> HostTravelCallVerifier {
        let chain_proof_verifier =
            chain_common::verifier::Verifier::new(chain_guest_ids, zk_proof::HostVerifier);
        let time_travel_verifier =
            time_travel::Verifier::new(chain_client.clone(), chain_proof_verifier);
        let teleport_verifier = teleport::Verifier::new(op_client_factory);
        travel_call::Verifier::new(time_travel_verifier, teleport_verifier)
    }

    #[instrument(skip_all)]
    pub async fn preflight(self, call: Call) -> Result<PreflightResult, preflight::Error> {
        let now = Instant::now();

        let SuccessfulExecutionResult {
            output: host_output,
            gas_used,
            metadata,
        } = TravelCallExecutor::new(&self.envs, self.start_execution_location, self.is_vlayer_test)
            .call(&call)?;

        self.travel_call_verifier
            .verify(&self.envs, self.start_execution_location)
            .await?;
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
        drop(self.travel_call_verifier); // Drop the verifier so that we can unwrap the Arc's in the clients
        let chain_proofs = self
            .chain_client
            .map_or(HashMap::new(), chain_client::RecordingClient::into_cache);
        let op_output_cache = self.op_client_factory.into_cache();
        let multi_evm_input = into_multi_input(self.envs)?;
        Ok(Input {
            multi_evm_input,
            start_execution_location: self.start_execution_location,
            chain_proofs,
            call,
            op_output_cache,
            is_vlayer_test: self.is_vlayer_test,
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
    let ProveInfo { receipt, stats, .. } = prover.prove(input)?;
    let elapsed_time = now.elapsed();

    let seal: Seal = EncodableReceipt::from(receipt.clone()).try_into()?;
    let seal: Bytes = seal.abi_encode().into();
    let raw_guest_output: Bytes = receipt.journal.bytes.into();

    Ok(EncodedProofWithStats::new(seal, raw_guest_output, stats, elapsed_time))
}
