use anyhow::Result;
use risc0_build::{embed_methods, GuestListEntry};
use risc0_build_ethereum::{generate_solidity_files, Options};
use std::fs::{create_dir_all, remove_file};
use std::path::{Path, PathBuf};

pub struct Risc0Builder {
    assets_dir: &'static Path,
    image_id_path: PathBuf,
    elf_path: PathBuf,
}

#[allow(clippy::new_without_default)]
impl Risc0Builder {
    pub fn new() -> Self {
        let assets_dir = Path::new("../../target/assets");

        Self {
            assets_dir,
            image_id_path: assets_dir.join("ImageID.sol"),
            elf_path: assets_dir.join("Elf.sol"),
        }
    }

    pub fn build(&self) -> Result<()> {
        let guests = embed_methods();

        self.build_solidity(&guests)
    }

    // Generate Solidity source files for use with Forge.
    fn build_solidity(&self, guests: &[GuestListEntry]) -> Result<()> {
        create_dir_all(self.assets_dir)?;

        remove_file(&self.image_id_path)?;
        remove_file(&self.elf_path)?;

        let solidity_opts = Options::default()
            .with_image_id_sol_path(&self.image_id_path)
            .with_elf_sol_path(&self.elf_path);

        generate_solidity_files(guests, &solidity_opts)?;

        Ok(())
    }
}
