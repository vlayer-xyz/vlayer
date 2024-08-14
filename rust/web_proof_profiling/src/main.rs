use web_proof_methods::WEB_PROOF_ELF;
use risc0_zkvm::{default_executor, ExecutorEnv};
use web_proof::{types::WebProof, verifier::_verify_proof, fixtures::{tls_proof_example_not_redacted, pub_key}};


fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();
    ();

    let x: u8 = 3;

    let env = ExecutorEnv::builder().write(&x)?.build()?;
    let exec = default_executor();
    exec.execute(env, WEB_PROOF_ELF)?;

    // let journal = _verify_proof(WebProof {
    //     tls_proof: tls_proof_example_not_redacted(),
    //     notary_pub_key: pub_key(),
    // }).unwrap();

    // println!("Sent: {}", journal.request);
    // println!("Recv: {}", journal.response);

    Ok(())
}