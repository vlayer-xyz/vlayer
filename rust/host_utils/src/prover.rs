use anyhow::Result;
use risc0_zkvm::{
    BonsaiProver, ExecutorEnv, ExternalProver, ProveInfo, Prover as ProverTrait, ProverOpts,
};

use crate::ProofMode;

#[derive(Debug, Default, Clone, Copy)]
pub struct Prover {
    mode: ProofMode,
}

impl Prover {
    pub fn new(mode: ProofMode) -> Self {
        Prover { mode }
    }

    pub fn prove(&self, env: ExecutorEnv<'_>, elf: &[u8]) -> Result<ProveInfo> {
        match self.mode {
            ProofMode::Groth16 => prove_bonsai(env, elf),
            ProofMode::Fake => prove_fake(env, elf),
        }
    }
}

fn prove_bonsai(env: ExecutorEnv<'_>, elf: &[u8]) -> Result<ProveInfo> {
    BonsaiProver::new("vlayer: bonsai").prove_with_opts(env, elf, &ProverOpts::groth16())
}

fn prove_fake(env: ExecutorEnv<'_>, elf: &[u8]) -> Result<ProveInfo> {
    // Fake proof mode cannot be forced in any other way, since all  risc0-zkvm modules, that could be reused here, are only crate-public.
    // Following is a temporary solution, that sets RISC0_DEV_MODE always to the same value, so race conditions are not a risk here.
    // Setting this env variable will be moved directly to ExternalProver, once it supports injection of config.
    unsafe {
        std::env::set_var("RISC0_DEV_MODE", "1");
    }

    ExternalProver::new("vlayer: ipc", "r0vm").prove(env, elf)
}
