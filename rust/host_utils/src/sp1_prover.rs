use std::{sync::Arc, time::Duration};

use sp1_sdk::{EnvProver, ExecutionReport, ProverClient, SP1PublicValues, SP1Stdin};
use tracing::info;

use crate::{error::Result, ProofMode};

#[derive(Clone)]
pub struct SP1Prover {
    mode: ProofMode,
    elf: Vec<u8>,
    client: Arc<EnvProver>,
}

impl SP1Prover {
    pub fn try_new(mode: ProofMode, elf: Vec<u8>) -> Result<Self> {
        let client = ProverClient::from_env();
        Ok(Self {
            mode,
            elf,
            client: Arc::new(client),
        })
    }

    pub fn prove(&self, stdin: SP1Stdin) -> Result<ProveInfo> {
        let (prove_info, elapsed) = match self.mode {
            ProofMode::Groth16 => unimplemented!(),
            ProofMode::Succinct => unimplemented!(),
            ProofMode::Fake => self.execute(stdin),
        }?;
        log_stats(&prove_info.report, &elapsed);
        Ok(prove_info)
    }

    fn execute(&self, stdin: SP1Stdin) -> Result<(ProveInfo, Duration)> {
        let start = tokio::time::Instant::now();
        let (public_values, report) = self.client.execute(&self.elf, &stdin).run()?;
        let proof = ProveInfo {
            proof: None,
            public_values,
            report,
        };

        Ok((proof, start.elapsed()))
    }
}

#[derive(Debug, Clone)]
pub struct ProveInfo {
    pub proof: Option<Vec<u8>>,
    pub public_values: SP1PublicValues,
    pub report: ExecutionReport,
}

fn log_stats(report: &ExecutionReport, elapsed: &Duration) {
    let elapsed_sec = elapsed.as_secs_f32();
    info!(
        "Prover stats. Cycles: {}, elapsed: {elapsed_sec:.1} s",
        report.total_instruction_count()
    );
}
