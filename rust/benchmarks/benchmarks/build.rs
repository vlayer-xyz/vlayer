use anyhow::Result;

fn main() -> Result<()> {
    #[cfg(not(clippy))]
    {
        use std::env;

        if env::var("RISC0_SKIP_BUILD").is_ok() {
            println!("cargo::warning=Skipped build of benchmark_guest");
            return Ok(());
        }
        let _ = risc0_build::embed_methods();
    }

    Ok(())
}
