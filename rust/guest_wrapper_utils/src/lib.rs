use anyhow::Result;
use risc0_build::{embed_methods, GuestListEntry};
use risc0_build_ethereum::{generate_solidity_files, Options};
use std::fs::{create_dir_all, remove_file};
use std::path::Path;

pub fn build_risc0_guest() -> Result<()> {
    let guests = embed_methods();

    build_risc0_solidity(guests)
}

// Generate Solidity source files for use with Forge.
fn build_risc0_solidity(guests: Vec<GuestListEntry>) -> Result<()> {
    let assets_path_str = "../../target/assets";
    let assets_path = Path::new(&assets_path_str);

    create_dir_all(assets_path)?;

    let image_id_path = assets_path.join("ImageID.sol");
    let elf_path = assets_path.join("Elf.sol");

    remove_file(&image_id_path)?;
    remove_file(&elf_path)?;

    let solidity_opts = Options::default()
        .with_image_id_sol_path(image_id_path)
        .with_elf_sol_path(elf_path);

    generate_solidity_files(&*guests, &solidity_opts)?;

    Ok(())
}
