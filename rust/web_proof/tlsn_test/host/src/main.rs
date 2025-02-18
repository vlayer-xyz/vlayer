use tlsn_test_methods::{
    TLSN_GUEST_ELF, TLSN_GUEST_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use tracing::info;
use web_proof::web_proof::WebProof;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let web_proof_json = include_str!("../web_proof.json");
    let input: WebProof = serde_json::from_str(web_proof_json).unwrap();
    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();

    info!("Proving...");
    let prove_info = prover
        .prove(env, TLSN_GUEST_ELF)
        .unwrap();
    info!("Done");

    info!("Verifying...");
    prove_info.receipt
        .verify(TLSN_GUEST_ID)
        .unwrap();
    info!("Done");
}
