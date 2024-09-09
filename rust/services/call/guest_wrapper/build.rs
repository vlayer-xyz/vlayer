use anyhow::Result;

fn main() -> Result<()> {
    #[cfg(not(clippy))]
    {
        use risc0_build_ethereum::{generate_solidity_files, Options};
        use std::fs::{create_dir_all, remove_file};
        use std::path::Path;

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
    }

    Ok(())
}
