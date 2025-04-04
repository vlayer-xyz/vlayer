use std::{borrow::BorrowMut, sync::Arc, time::Duration};

use bytes::Bytes;
use sp1_sdk::{
    EnvProver, ExecutionReport, ProverClient, SP1Proof, SP1ProofMode, SP1ProofWithPublicValues,
    SP1ProvingKey, SP1Stdin, SP1_CIRCUIT_VERSION,
};
use tracing::info;

use crate::{error::Result, ProofMode};

#[derive(Clone)]
pub struct SP1Prover {
    mode: ProofMode,
    elf: Vec<u8>,
    pk: SP1ProvingKey,
    prover: Arc<EnvProver>,
}

impl SP1Prover {
    pub fn try_new(mode: ProofMode, elf: Bytes) -> Result<Self> {
        let prover = ProverClient::from_env();

        let (pk, _) = prover.setup(&elf);

        Ok(Self {
            mode,
            elf: elf.into(),
            pk,
            prover: Arc::new(prover),
        })
    }

    pub fn prove(&self, stdin: &SP1Stdin) -> Result<ProveInfo> {
        let (prove_info, elapsed) = match self.mode {
            ProofMode::Groth16 => self.prove_network(stdin, SP1ProofMode::Groth16),
            ProofMode::Succinct => self.prove_network(stdin, SP1ProofMode::Compressed),
            ProofMode::Fake => self.execute(stdin),
        }?;
        if let Some(report) = &prove_info.report {
            log_stats(report, &elapsed);
        }

        Ok(prove_info)
    }

    fn execute(&self, stdin: &SP1Stdin) -> Result<(ProveInfo, Duration)> {
        let start = tokio::time::Instant::now();

        let (public_values, report) = self.prover.execute(&self.elf, stdin).run()?;

        let mut mock_proof = SP1ProofWithPublicValues::create_mock_proof(
            &self.pk,
            public_values.clone(),
            SP1ProofMode::Groth16,
            SP1_CIRCUIT_VERSION,
        );

        match mock_proof.proof.borrow_mut() {
            SP1Proof::Groth16(groth16_bn254_proof) => {
                groth16_bn254_proof.encoded_proof = hex::encode(&[0; 256]);
            }
            _ => {}
        };

        let proof = ProveInfo {
            proof: mock_proof,
            report: Some(report),
        };

        Ok((proof, start.elapsed()))
    }

    fn prove_network(&self, stdin: &SP1Stdin, mode: SP1ProofMode) -> Result<(ProveInfo, Duration)> {
        let start = tokio::time::Instant::now();

        let proof = self.prover.prove(&self.pk, stdin).mode(mode).run()?;

        let proof = ProveInfo {
            proof,
            report: None,
        };

        Ok((proof, start.elapsed()))
    }
}

#[derive(Debug, Clone)]
pub struct ProveInfo {
    pub proof: SP1ProofWithPublicValues,
    pub report: Option<ExecutionReport>,
}

fn log_stats(report: &ExecutionReport, elapsed: &Duration) {
    let elapsed_sec = elapsed.as_secs_f32();
    info!(
        "Prover stats. Cycles: {}, elapsed: {elapsed_sec:.1} s",
        report.total_instruction_count()
    );
}
