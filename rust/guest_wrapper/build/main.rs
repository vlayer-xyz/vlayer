#[cfg(not(clippy))]
mod utils;

#[cfg(not(clippy))]
pub fn main() -> anyhow::Result<()> {
    use std::path::Path;

    use utils::*;

    if *RISC0_SKIP_BUILD {
        println!("cargo::warning=Skipped build of guest_wrapper");
        return Ok(());
    }

    let (image_id_path, elf_path) = prepare_assets_dir()?;

    if let Some(artifacts_path) = use_var("RISC0_EXISTING_GUEST") {
        copy_existing_guest_artifacts(Path::new(&artifacts_path), &image_id_path)?;
        return Ok(());
    }

    let (call_guest, chain_guest) = build_guests()?;
    check_or_update_chain_guest_id(&chain_guest)?;
    generate_guest_sol_files(call_guest, &image_id_path, &elf_path)?;

    Ok(())
}

#[cfg(clippy)]
pub fn main() {}
