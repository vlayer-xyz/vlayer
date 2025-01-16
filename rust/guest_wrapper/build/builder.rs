use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Read, Write},
};

use risc0_build::{embed_methods_with_options, DockerOptions, GuestListEntry, GuestOptions};
use risc0_zkp::core::digest::Digest;

use crate::{
    data_layout,
    utils::{remove_file_if_exists, remove_guest, use_bool_var, path_from_env},
};

data_layout!(DataLayout {
    project_root: "../..".into(),
    target_dir: path_from_env("CARGO_TARGET_DIR").unwrap_or(project_root.join("target")),
    out_dir: path_from_env("OUT_DIR").unwrap(),
} {
    (project_root / "rust/guest_wrapper") => guest_wrapper_dir,
    (guest_wrapper_dir / "artifacts/chain_guest/elf_id") => chain_guest_elf_id,
    (target_dir / "assets") => solidity_assets_dir,
    (target_dir / "assets/Elf.sol") => elf_sol_output,
    (target_dir / "assets/ImageId.sol") => image_id_sol_output,
    (out_dir / "methods.rs") => rust_methods,
});

data_layout!(ExistingGuestLayout {
    artifacts_dir: path_from_env("RISC0_EXISTING_GUEST")?,
} {
    (artifacts_dir / "methods.rs") => rust_methods,
    (artifacts_dir / "ImageId.sol") => image_id_sol,
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
        Self {
            data_layout: DataLayout::new().unwrap(),
            existing_guest: ExistingGuestLayout::new(),
            skip_build: use_bool_var("RISC0_SKIP_BUILD"),
            use_docker: use_bool_var("RISC0_USE_DOCKER"),
            update_guest_elf: use_bool_var("UPDATE_GUEST_ELF_ID"),
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

        let (call_guest, chain_guest) = self.build_guests()?;
        self.check_or_update_chain_guest_id(&chain_guest)?;
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

    fn get_guest_options(&self) -> GuestOptions {
        let use_docker = self.use_docker.then_some(DockerOptions {
            root_dir: Some(self.data_layout.project_root().into()),
            env: vec![
                ("CC_riscv32im_risc0_zkvm_elf".to_string(), "clang".to_string()),
                (
                    "CFLAGS_riscv32im_risc0_zkvm_elf".to_string(),
                    "-nostdlibinc -DRING_CORE_NOSTDLIBINC=1 -target riscv32-unknown-elf -march=rv32im -D__ILP32__=1".to_string()
                ),
            ],
        });
        GuestOptions {
            use_docker,
            ..Default::default()
        }
    }

    fn build_guests(&self) -> anyhow::Result<(GuestListEntry, GuestListEntry)> {
        let guest_options = self.get_guest_options();
        let mut guests = embed_methods_with_options(HashMap::from([
            ("risc0_call_guest", guest_options.clone()),
            ("risc0_chain_guest", guest_options),
        ]));
        let call_guest = remove_guest(&mut guests, "risc0_call_guest")?;
        let chain_guest = remove_guest(&mut guests, "risc0_chain_guest")?;
        Ok((call_guest, chain_guest))
    }

    /// Verify that chain guest ID is unchanged, or generate new one if `UPDATE_GUEST_ELF_ID` is set.
    fn check_or_update_chain_guest_id(&self, chain_guest: &GuestListEntry) -> anyhow::Result<()> {
        let guest_id_path = self.data_layout.chain_guest_elf_id();
        if self.update_guest_elf {
            anyhow::ensure!(self.use_docker, "`UPDATE_GUEST_ELF_ID` requires `RISC0_USE_DOCKER`");
            let chain_guest_elf_id: Digest = chain_guest.image_id.into();
            File::create(guest_id_path)?.write_all(chain_guest_elf_id.as_bytes())?;
        } else if self.use_docker {
            println!("cargo::rerun-if-changed={}", guest_id_path.display());
            let mut buf = [0_u8; 32];
            File::open(guest_id_path)?.read_exact(&mut buf)?;
            let chain_guest_elf_id = Digest::from_bytes(buf);
            anyhow::ensure!(
                chain_guest_elf_id == chain_guest.image_id.into(),
                "Chain guest ELF ID mismatch. Run with `UPDATE_GUEST_ELF_ID=1` to update."
            );
        }
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
