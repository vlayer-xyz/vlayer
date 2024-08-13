use web_proof_methods::WEB_PROOF_ELF;
use risc0_zkvm::{default_executor, ExecutorEnv};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();
    ();

    let x: u8 = 3;

    let env = ExecutorEnv::builder().write(&x)?.build()?;
    let exec = default_executor();
    exec.execute(env, WEB_PROOF_ELF)?;

    Ok(())
}