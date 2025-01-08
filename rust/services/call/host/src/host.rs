use std::time::{Duration, Instant};

use alloy_sol_types::SolValue;
use bytes::Bytes;
use call_engine::{
    evm::{
        env::{cached::CachedEvmEnv, location::ExecutionLocation},
        execution_result::SuccessfulExecutionResult,
    },
    travel_call_executor::TravelCallExecutor,
    verifier::{
        chain_proof,
        guest_input::{self, Verifier},
        zk_proof,
    },
    Call, CallGuestId, GuestOutput, HostOutput, Input, Seal,
};
use chain_client::Client as ChainClient;
use common::GuestElf;
pub use config::Config;
use derive_new::new;
pub use error::{AwaitingChainProofError, BuilderError, Error, PreflightError, ProvingError};
pub use prover::Prover;
use provider::CachedMultiProvider;
use risc0_zkvm::{ProveInfo, SessionStats};
use seal::EncodableReceipt;
use tracing::{info, instrument};

use crate::{db::HostDb, evm_env::factory::HostEvmEnvFactory, into_input::into_multi_input};

mod builder;
mod config;
mod error;
mod prover;
#[cfg(test)]
mod tests;

pub struct Host {
    start_execution_location: ExecutionLocation,
    envs: CachedEvmEnv<HostDb>,
    prover: Prover,
    chain_client: chain_client::RecordingClient,
    chain_proof_verifier: chain_proof::ZkVerifier,
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
}

impl Host {
    pub fn new(
        providers: CachedMultiProvider,
        start_execution_location: ExecutionLocation,
        chain_client: Box<dyn chain_client::Client>,
        config: Config,
    ) -> Self {
        let envs = CachedEvmEnv::from_factory(HostEvmEnvFactory::new(providers));
        let prover = Prover::new(config.proof_mode, &config.call_guest_elf);
        let chain_proof_verifier =
            chain_proof::ZkVerifier::new(config.chain_guest_elf.id, zk_proof::HostVerifier);
        let chain_client = chain_client::RecordingClient::new(chain_client);

        Host {
            envs,
            start_execution_location,
            prover,
            chain_client,
            chain_proof_verifier,
            guest_elf: config.call_guest_elf,
        }
    }

    pub async fn chain_proof_ready(&self) -> Result<bool, AwaitingChainProofError> {
        let latest_indexed_block = self
            .chain_client
            .get_sync_status(self.start_execution_location.chain_id)
            .await?
            .last_block;
        Ok(latest_indexed_block >= self.start_execution_location.block_number)
    }

    #[instrument(skip_all)]
    pub async fn preflight(self, call: Call) -> Result<PreflightResult, PreflightError> {
        let now = Instant::now();
        let SuccessfulExecutionResult {
            output: host_output,
            gas_used,
        } = TravelCallExecutor::new(&self.envs).call(&call, self.start_execution_location)?;
        let elapsed_time = now.elapsed();

        info!(
            gas_used = gas_used,
            elapsed_time = elapsed_time.as_millis(),
            "preflight finished",
        );

        let multi_evm_input = into_multi_input(self.envs)?;

        let verifier = guest_input::ZkVerifier::new(self.chain_client, self.chain_proof_verifier);
        verifier.verify(&multi_evm_input).await?;
        let (chain_proof_client, _) = verifier.into_parts();
        let chain_proofs = chain_proof_client.into_cache();

        let input = Input {
            multi_evm_input,
            start_execution_location: self.start_execution_location,
            chain_proofs,
            call,
        };

        Ok(PreflightResult::new(host_output.into(), input, gas_used, elapsed_time))
    }

    #[instrument(skip_all)]
    pub fn prove(
        prover: &Prover,
        call_guest_id: CallGuestId,
        PreflightResult {
            host_output, input, ..
        }: PreflightResult,
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

        info!(
            cycles_used = cycles_used,
            elapsed_time = elapsed_time.as_millis(),
            "proving finished"
        );

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
        let preflight_result = self.preflight(call).await?;
        Ok(Host::prove(&prover, call_guest_id, preflight_result)?)
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
