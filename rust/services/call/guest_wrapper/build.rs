use std::{fs, io, path::PathBuf};

use anyhow::Result;

fn main() -> Result<()> {
    #[cfg(not(clippy))]
    {
        use std::{
            env,
            fs::{copy, create_dir_all},
            path::Path,
        };

        use risc0_build_ethereum::{generate_solidity_files, Options};

        let out_dir = env::var("OUT_DIR").expect("'OUT_DIR' is not set");
        let out_dir_path = Path::new(&out_dir);
        let assets_dir = Path::new("../../../target/assets");

        let image_id_sol_output_path = assets_dir.join("ImageID.sol");
        let elf_sol_output_path = assets_dir.join("Elf.sol");
        let solidity_opts = Options::default()
            .with_image_id_sol_path(&image_id_sol_output_path)
            .with_elf_sol_path(&elf_sol_output_path);

        if env::var("RISC0_SKIP_BUILD").is_ok() {
            println!("cargo::warning=Skipped build of call_guest");
            return Ok(());
        }

        create_dir_all(&assets_dir)?;
        _remove_file_if_exists(&image_id_sol_output_path)?;
        _remove_file_if_exists(&elf_sol_output_path)?;

        if let Ok(guest_artifacts_path) = env::var("RISC0_EXISTING_CALL_GUEST") {
            println!("cargo::warning=Using existing call_guest from {}", &guest_artifacts_path);
            let guest_artifacts_path = Path::new(&guest_artifacts_path);

            let methods_rs_path = guest_artifacts_path.join("methods.rs");
            let image_id_path = guest_artifacts_path.join("ImageID.sol");

            copy(methods_rs_path, out_dir_path.join("methods.rs"))?;
            copy(image_id_path, image_id_sol_output_path)?;
        } else {
            let guests = risc0_build::embed_methods();
            generate_solidity_files(&guests, &solidity_opts)?;
        }
    }

    Ok(())
}

fn _remove_file_if_exists(path: &PathBuf) -> Result<(), io::Error> {
    match fs::remove_file(path) {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e),
    }
}
