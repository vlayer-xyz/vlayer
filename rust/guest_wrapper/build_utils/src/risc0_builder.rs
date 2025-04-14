use std::{collections::HashMap, fs, io};

use risc0_build::{
    DockerOptionsBuilder, GuestListEntry, GuestOptions, GuestOptionsBuilder,
    embed_methods_with_options,
};

use crate::{
    PROJECT_ROOT, chain_guest_id, data_layout, path_from_env, remove_file_if_exists, use_bool_var,
};

data_layout!(DataLayout {
    project_root: PROJECT_ROOT.into(),
    out_dir: path_from_env("OUT_DIR").unwrap(),
} {
    (project_root / "target/assets") => solidity_assets_dir,
    (project_root / "target/assets/Elf.sol") => elf_sol_output,
    (project_root / "target/assets/ImageID.sol") => image_id_sol_output,
    (out_dir / "methods.rs") => rust_methods,
});

data_layout!(ExistingGuestLayout {
    artifacts_dir: path_from_env("RISC0_EXISTING_GUEST")?,
} {
    (artifacts_dir / "methods.rs") => rust_methods,
    (artifacts_dir / "ImageID.sol") => image_id_sol,
});

pub struct Builder {
    data_layout: DataLayout,
    existing_guest: Option<ExistingGuestLayout>,
    skip_build: bool,
    use_docker: bool,
    update_guest_elf: bool,
}

impl Builder {
    pub fn from_env() -> Self {
        let use_docker = use_bool_var("RISC0_USE_DOCKER");
        let update_guest_elf = use_bool_var("UPDATE_GUEST_ELF_ID");
        assert!(
            !update_guest_elf || use_docker,
            "`UPDATE_GUEST_ELF_ID` requires `RISC0_USE_DOCKER`"
        );
        Self {
            data_layout: DataLayout::new().unwrap(),
            existing_guest: ExistingGuestLayout::new(),
            skip_build: use_bool_var("RISC0_SKIP_BUILD"),
            use_docker,
            update_guest_elf,
        }
    }

    pub fn build(&self) -> anyhow::Result<()> {
        if self.skip_build {
            println!("cargo::warning=Skipped build of guest_wrapper");
            return Ok(());
        }

        self.prepare_assets_dir()?;

        if self.copy_existing_guest_artifacts()? {
            return Ok(());
        }

        if self.update_guest_elf {
            self.update_chain_guest()?;
        }
        let (call_guest, chain_guest) = self.build_guests()?;

        if self.use_docker {
            // Assert that image ID was correctly updated, or guest was unchanged
            chain_guest_id::assert(chain_guest.image_id)?;
        }
        self.generate_guest_sol_files(call_guest)?;

        Ok(())
    }

    /// Create directory for assets, clean up old artifacts.
    fn prepare_assets_dir(&self) -> io::Result<()> {
        fs::create_dir_all(self.data_layout.solidity_assets_dir())?;
        remove_file_if_exists(&self.data_layout.image_id_sol_output())?;
        remove_file_if_exists(&self.data_layout.elf_sol_output())?;
        Ok(())
    }

    /// Copy pre-built guest artifacts, if `RISC0_EXISTING_GUEST` is set.
    /// Returns true iff artifacts were copied.
    fn copy_existing_guest_artifacts(&self) -> io::Result<bool> {
        let Some(existing_guest) = self.existing_guest.as_ref() else {
            return Ok(false);
        };

        println!(
            "cargo::warning=Using existing guest artifacts from {}",
            existing_guest.artifacts_dir().display()
        );

        fs::copy(existing_guest.rust_methods(), self.data_layout.rust_methods())?;
        fs::copy(existing_guest.image_id_sol(), self.data_layout.image_id_sol_output())?;

        Ok(true)
    }

    fn get_guest_options(&self) -> anyhow::Result<GuestOptions> {
        let mut builder = GuestOptionsBuilder::default();

        if self.use_docker {
            builder.use_docker(
                DockerOptionsBuilder::default()
                    .root_dir(self.data_layout.project_root())
                    .env(Vec::from([
                        ("CC_riscv32im_risc0_zkvm_elf".to_string(), "clang".to_string()),
                        ("CFLAGS_riscv32im_risc0_zkvm_elf".to_string(), "-nostdlibinc -DRING_CORE_NOSTDLIBINC=1 -target riscv32-unknown-elf -march=rv32im -D__ILP32__=1".to_string()),
                    ]))
                    .build()?,
            );
        }

        Ok(builder.build()?)
    }

    fn build_guests(&self) -> anyhow::Result<(GuestListEntry, GuestListEntry)> {
        let guest_options = self.get_guest_options()?;
        let mut guests = embed_methods_with_options(HashMap::from([
            ("risc0_call_guest", guest_options.clone()),
            ("risc0_chain_guest", guest_options),
        ]));
        let call_guest = remove_guest(&mut guests, "risc0_call_guest")?;
        let chain_guest = remove_guest(&mut guests, "risc0_chain_guest")?;
        Ok((call_guest, chain_guest))
    }

    /// Add current chain guest ID to history and generate a new one
    fn update_chain_guest(&self) -> anyhow::Result<()> {
        chain_guest_id::add_current_to_history()?;
        let (_, chain_guest) = self.build_guests()?;
        chain_guest_id::update(chain_guest.image_id)?;

        Ok(())
    }

    /// Generate solidity files with call guest image ID and ELF.
    fn generate_guest_sol_files(&self, call_guest: GuestListEntry) -> anyhow::Result<()> {
        let solidity_opts = risc0_build_ethereum::Options::default()
            .with_image_id_sol_path(self.data_layout.image_id_sol_output())
            .with_elf_sol_path(self.data_layout.elf_sol_output());
        risc0_build_ethereum::generate_solidity_files(&[call_guest], &solidity_opts)?;
        Ok(())
    }
}

/// Remove `GuestListEntry` with given name from vector.
fn remove_guest(guests: &mut Vec<GuestListEntry>, name: &str) -> anyhow::Result<GuestListEntry> {
    let idx = guests
        .iter()
        .position(|g| g.name == name)
        .ok_or_else(|| anyhow::anyhow!("Guest {name} not found"))?;
    Ok(guests.remove(idx))
}
