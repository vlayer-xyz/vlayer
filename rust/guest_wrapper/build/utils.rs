use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};

use lazy_static::lazy_static;
use risc0_build::{embed_methods_with_options, DockerOptions, GuestListEntry, GuestOptions};
use risc0_zkp::core::digest::Digest;

lazy_static! {
    pub static ref RISC0_SKIP_BUILD: bool = use_bool_var("RISC0_SKIP_BUILD");
    pub static ref RISC0_USE_DOCKER: bool = use_bool_var("RISC0_USE_DOCKER");
    pub static ref UPDATE_GUEST_ELF_ID: bool = use_bool_var("UPDATE_GUEST_ELF_ID");
}

pub fn use_var(key: &str) -> Option<String> {
    println!("cargo:rerun-if-env-changed={key}");
    env::var(key).ok()
}

/// Parse boolean env variable:
///   * true if variable is one of "1", "true", "yes",
///   * false if variables is one of "0", "false", "no",
///   * false if variable is unset,
///   * panic if variable is set to unexpected value.
pub fn use_bool_var(key: &str) -> bool {
    use_var(key)
        .map(parse_bool_str)
        .transpose()
        .expect(&format!("Invalid value for {key}"))
        .unwrap_or(false)
}

fn parse_bool_str(s: String) -> anyhow::Result<bool> {
    match s.to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" => Ok(true),
        "0" | "false" | "no" => Ok(false),
        s => anyhow::bail!("Invalid value: '{s}' accepted: 1/0/true/false/yes/no"),
    }
}

fn remove_file_if_exists(path: &PathBuf) -> io::Result<()> {
    match fs::remove_file(path) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        res => res,
    }
}

/// Get path to target directory. If `CARGO_TARGET_DIR` is set use that,
/// otherwise default to ../../target
fn get_target_dir() -> PathBuf {
    if let Ok(target_dir) = env::var("CARGO_TARGET_DIR") {
        target_dir.into()
    } else {
        PathBuf::from("../../target")
    }
}

/// Create directory for assets, clean up old artifacts.
/// Returns paths for solidity outputs: (image_id_path, elf_path).
pub fn prepare_assets_dir() -> io::Result<(PathBuf, PathBuf)> {
    let assets_dir = get_target_dir().join("assets");
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

/// Remove `GuestListEntry` with given name from vector.
fn remove_guest(guests: &mut Vec<GuestListEntry>, name: &str) -> anyhow::Result<GuestListEntry> {
    let idx = guests
        .iter()
        .position(|g| g.name == name)
        .ok_or_else(|| anyhow::anyhow!("Guest {name} not found"))?;
    Ok(guests.remove(idx))
}

/// Build guests. Returns (call_guest, chain_guest) tuple.
pub fn build_guests() -> anyhow::Result<(GuestListEntry, GuestListEntry)> {
    // Configure docker build
    let use_docker = RISC0_USE_DOCKER.then_some(DockerOptions {
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
    let call_guest = remove_guest(&mut guests, "risc0_call_guest")?;
    let chain_guest = remove_guest(&mut guests, "risc0_chain_guest")?;
    Ok((call_guest, chain_guest))
}

/// Verify that chain guest ID is unchanged, or generate new one if `UPDATE_GUEST_ELF_ID` is set.
pub fn check_or_update_chain_guest_id(chain_guest: &GuestListEntry) -> anyhow::Result<()> {
    if *UPDATE_GUEST_ELF_ID {
        anyhow::ensure!(*RISC0_USE_DOCKER, "`UPDATE_GUEST_ELF_ID` requires `RISC0_USE_DOCKER`");
        let chain_guest_elf_id: Digest = chain_guest.image_id.into();
        File::create("chain_guest_elf_id")?.write_all(chain_guest_elf_id.as_bytes())?;
    } else if *RISC0_USE_DOCKER {
        println!("cargo::rerun-if-changed=chain_guest_elf_id");
        let chain_guest_elf_id: Digest = (*include_bytes!("../chain_guest_elf_id")).into();
        anyhow::ensure!(
            chain_guest_elf_id == chain_guest.image_id.into(),
            "Chain guest ELF ID mismatch. Run with `UPDATE_GUEST_ELF_ID=1` to update."
        );
    }
    Ok(())
}

/// Generate solidity files with call guest image ID and ELF.
pub fn generate_guest_sol_files(
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
