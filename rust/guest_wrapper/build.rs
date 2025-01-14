#[cfg(not(clippy))]
mod utils {
    use std::{
        collections::HashMap,
        env,
        fs::{self, File},
        io::{self, Write},
        path::{Path, PathBuf},
    };

    use risc0_build::{embed_methods_with_options, DockerOptions, GuestListEntry, GuestOptions};
    use risc0_zkp::core::digest::Digest;

    pub fn use_var(key: &str) -> Option<String> {
        println!("cargo:rerun-if-env-changed={key}");
        env::var(key).ok()
    }

    fn remove_file_if_exists(path: &PathBuf) -> io::Result<()> {
        match fs::remove_file(path) {
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
            res => res,
        }
    }

    pub fn prepare_assets_dir() -> io::Result<(PathBuf, PathBuf)> {
        let assets_dir = if let Ok(target_dir) = env::var("CARGO_TARGET_DIR") {
            PathBuf::from(target_dir).join("assets")
        } else {
            PathBuf::from("../../target/assets")
        };
        let image_id_path = assets_dir.join("ImageID.sol");
        let elf_path = assets_dir.join("Elf.sol");

        fs::create_dir_all(&assets_dir)?;
        remove_file_if_exists(&image_id_path)?;
        remove_file_if_exists(&elf_path)?;

        Ok((image_id_path, elf_path))
    }

    pub fn copy_existing_guest_artifacts(
        artifacts_path: &Path,
        image_id_path: &Path,
    ) -> io::Result<()> {
        println!(
            "cargo::warning=Using existing guest artifacts from {}",
            artifacts_path.display()
        );
        let out_dir: PathBuf = env::var("OUT_DIR").expect("`OUT_DIR` not set").into();

        fs::copy(artifacts_path.join("methods.rs"), out_dir.join("methods.rs"))?;
        fs::copy(artifacts_path.join("ImageID.sol"), image_id_path)?;

        Ok(())
    }

    fn get_guest(guests: &mut Vec<GuestListEntry>, name: &str) -> anyhow::Result<GuestListEntry> {
        let idx = guests
            .iter()
            .position(|g| g.name == name)
            .ok_or_else(|| anyhow::anyhow!("Guest {name} not found"))?;
        Ok(guests.remove(idx))
    }

    pub fn build_guests() -> anyhow::Result<(GuestListEntry, GuestListEntry)> {
        // Configure docker build
        let use_docker = use_var("RISC0_USE_DOCKER").map(|_| DockerOptions {
        root_dir: Some("../../".into()),
        env: vec![
                ("CC_riscv32im_risc0_zkvm_elf".to_string(), "clang".to_string()),
                (
                    "CFLAGS_riscv32im_risc0_zkvm_elf".to_string(),
                    "-nostdlibinc -DRING_CORE_NOSTDLIBINC=1 -target riscv32-unknown-elf -march=rv32im -D__ILP32__=1".to_string()
                ),
            ],
        });
        let guest_options = GuestOptions {
            use_docker,
            ..Default::default()
        };

        // Build guests
        let mut guests = embed_methods_with_options(HashMap::from([
            ("risc0_call_guest", guest_options.clone()),
            ("risc0_chain_guest", guest_options),
        ]));
        let call_guest = get_guest(&mut guests, "risc0_call_guest")?;
        let chain_guest = get_guest(&mut guests, "risc0_chain_guest")?;
        Ok((call_guest, chain_guest))
    }

    pub fn check_or_update_chain_guest_id(chain_guest: &GuestListEntry) -> anyhow::Result<()> {
        if use_var("UPDATE_GUEST_ELF_ID").is_some() {
            let chain_guest_elf_id: Digest = chain_guest.image_id.into();
            File::create("chain_guest_elf_id")?.write_all(chain_guest_elf_id.as_bytes())?;
        } else {
            println!("cargo::rerun-if-changed=chain_guest_elf_id");
            let chain_guest_elf_id: Digest = (*include_bytes!("chain_guest_elf_id")).into();
            anyhow::ensure!(
                chain_guest_elf_id == chain_guest.image_id.into(),
                "Chain guest ELF ID mismatch. Run with `UPDATE_GUEST_ELF_ID=1` to update."
            );
        }
        Ok(())
    }

    pub fn generate_solidity_files(
        call_guest: GuestListEntry,
        image_id_path: &Path,
        elf_path: &Path,
    ) -> anyhow::Result<()> {
        let solidity_opts = risc0_build_ethereum::Options::default()
            .with_image_id_sol_path(image_id_path)
            .with_elf_sol_path(elf_path);
        risc0_build_ethereum::generate_solidity_files(&[call_guest], &solidity_opts)?;
        Ok(())
    }
}

#[cfg(not(clippy))]
pub fn main() -> anyhow::Result<()> {
    use std::path::Path;

    use utils::*;

    if use_var("RISC0_SKIP_BUILD").is_some() {
        println!("cargo::warning=Skipped build of guest_wrapper");
        return Ok(());
    }

    let (image_id_path, elf_path) = prepare_assets_dir()?;

    if let Some(artifacts_path) = use_var("RISC0_EXISTING_GUEST") {
        copy_existing_guest_artifacts(Path::new(&artifacts_path), &image_id_path)?;
        return Ok(());
    }

    let (call_guest, chain_guest) = build_guests()?;

    if use_var("RISC0_USE_DOCKER").is_some() {
        check_or_update_chain_guest_id(&chain_guest)?;
    }

    generate_solidity_files(call_guest, &image_id_path, &elf_path)?;

    Ok(())
}

#[cfg(clippy)]
pub fn main() {}
