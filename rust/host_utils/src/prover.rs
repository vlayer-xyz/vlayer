use std::time::Duration;

use derivative::Derivative;
use risc0_zkvm::{
    BonsaiProver, ExecutorEnv, ExternalProver, ProveInfo, Prover as ProverTrait, ProverOpts,
    SessionStats,
};
use thiserror::Error;
use tracing::info;

use crate::ProofMode;

#[derive(Debug, Default, Clone, Copy)]
pub struct Prover {
    pub mode: ProofMode,
}

#[derive(Debug, Error, Derivative)]
#[derivative(PartialEq, Eq)]
#[error(transparent)]
pub struct Error(
    #[from]
    #[derivative(PartialEq = "ignore")]
    anyhow::Error,
);
pub type Result<T> = std::result::Result<T, Error>;

impl Prover {
    pub fn try_new(mode: ProofMode) -> Result<Self> {
        if mode == ProofMode::Fake && !risc0_dev_mode_on() {
            Err(anyhow::anyhow!("fake proofs require `RISC0_DEV_MODE=1`"))?
        }
        Ok(Self { mode })
    }

    pub fn prove(&self, env: ExecutorEnv<'_>, elf: &[u8]) -> Result<ProveInfo> {
        let (prove_info, elapsed) = match self.mode {
            ProofMode::Groth16 => prove_bonsai(env, elf, &ProverOpts::groth16()),
            ProofMode::Succinct => prove_bonsai(env, elf, &ProverOpts::succinct()),
            ProofMode::Fake => prove_fake(env, elf),
        }?;
        log_stats(&prove_info.stats, &elapsed);
        Ok(prove_info)
    }
}

fn risc0_dev_mode_on() -> bool {
    matches!(
        std::env::var("RISC0_DEV_MODE")
            .unwrap_or_default()
            .to_ascii_lowercase()
            .as_str(),
        "1" | "true" | "yes"
    )
}

pub fn set_risc0_dev_mode() {
    // Fake proof mode cannot be forced in any other way, since all risc0-zkvm modules, that could be reused here, are only crate-public.
    // Following is a temporary solution, that sets RISC0_DEV_MODE always to the same value, so race conditions are not a risk here.
    // Setting this env variable will be moved directly to ExternalProver, once it supports injection of config.
    // Note that setting this env var is required to correctly instrument fake proving *and* verifying.
    // https://github.com/risc0/risc0/issues/2814
    unsafe {
        std::env::set_var("RISC0_DEV_MODE", "1");
    }
}

fn log_stats(stats: &SessionStats, elapsed: &Duration) {
    let SessionStats {
        total_cycles,
        user_cycles,
        segments,
        ..
    } = stats;
    let elapsed_sec = elapsed.as_secs_f32();
    info!(
        "Prover stats. Segments: {segments}, cycles: {total_cycles}, user cycles: {user_cycles} elapsed: {elapsed_sec:.1} s"
    );
}

fn prove_bonsai(
    env: ExecutorEnv<'_>,
    elf: &[u8],
    opts: &ProverOpts,
) -> Result<(ProveInfo, Duration)> {
    info!("Proving with Bonsai");
    let bonsai_prover = BonsaiProver::new("vlayer: bonsai");
    let start = tokio::time::Instant::now();
    // block_in_place is used to avoid tokio runtime panic, since bonsai_prover.prove_with_opts is blocking.
    // https://github.com/risc0/risc0/issues/2049
    let prove_info = tokio::task::block_in_place(|| bonsai_prover.prove_with_opts(env, elf, opts))?;
    info!("Proving with Bonsai done");
    Ok((prove_info, start.elapsed()))
}

fn prove_fake(env: ExecutorEnv<'_>, elf: &[u8]) -> Result<(ProveInfo, Duration)> {
    let start = tokio::time::Instant::now();
    let prove_info = ExternalProver::new("vlayer: ipc", "r0vm").prove(env, elf)?;
    Ok((prove_info, start.elapsed()))
}
