use anyhow::Result;

fn main() -> Result<()> {
    #[cfg(not(clippy))]
    {
        use std::{
            env,
            fs::{create_dir_all, remove_file, copy},
            path::Path,
        };

        use risc0_build_ethereum::{generate_solidity_files, Options};

        if env::var("RISC0_SKIP_BUILD").is_ok() {
            return Ok(());
        }

        let guests = risc0_build::embed_methods();

        let assets_dir = Path::new("../../../target/assets");
        let image_id_path = assets_dir.join("ImageID.sol");
        let elf_path = assets_dir.join("Elf.sol");

        create_dir_all(&assets_dir)?;

        let _ = remove_file(&image_id_path);
        let _ = remove_file(&elf_path);

        let solidity_opts = Options::default()
            .with_image_id_sol_path(&image_id_path)
            .with_elf_sol_path(&elf_path);

        generate_solidity_files(&*guests, &solidity_opts)?;

        if env::var("RISC0_REPLACE_METHOD_RS_PATH").is_ok() {
            let method_rs_replacement = env::var("RISC0_REPLACE_METHOD_RS_PATH").unwrap();
            let method_rs_replacement_path = Path::new(&method_rs_replacement);

            let out_dir = env::var("OUT_DIR").unwrap();
            let out_dir_path = Path::new(&out_dir);
            copy(&method_rs_replacement_path, out_dir_path.join("methods.rs"))?;
        }
    }

    Ok(())
}
