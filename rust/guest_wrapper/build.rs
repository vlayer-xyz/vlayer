#[cfg(not(clippy))]
pub fn main() -> anyhow::Result<()> {
    use std::{collections::HashMap, env, fs::{self, File}, io::{self, Write}, path::{Path, PathBuf}};

    use risc0_build::{embed_methods_with_options, DockerOptions, GuestOptions};
    use risc0_build_ethereum::{generate_solidity_files, Options};
    use risc0_zkp::core::digest::Digest;

    fn _remove_file_if_exists(path: &PathBuf) -> io::Result<()> {
        match fs::remove_file(path) {
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
            res => res,
        }
    }

    // Prepare assets directories
    let assets_dir = Path::new("../target/assets");
    let image_id_sol_output_path = assets_dir.join("ImageID.sol");
    let elf_sol_output_path = assets_dir.join("Elf.sol");

    fs::create_dir_all(&assets_dir)?;
    _remove_file_if_exists(&image_id_sol_output_path)?;
    _remove_file_if_exists(&elf_sol_output_path)?;

    // Configure docker build
    println!("cargo:rerun-if-env-changed=RISC0_USE_DOCKER");
    let use_docker = env::var("RISC0_USE_DOCKER").ok().map(|_| DockerOptions {
        root_dir: Some("../".into()),
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
    
    // Generate or verify guest elf id
    let chain_guest_entry = guests
        .iter()
        .find(|entry| entry.name == "risc0_chain_guest")
        .ok_or_else(|| anyhow::anyhow!("Chain guest entry not found"))?;

    println!("cargo:rerun-if-env-changed=UPDATE_GUEST_ELF_ID");
    if env::var("UPDATE_GUEST_ELF_ID").is_ok() {
        let chain_guest_elf_id: Digest = chain_guest_entry.image_id.into();
        File::create("chain_guest_elf_id")?.write_all(chain_guest_elf_id.as_bytes())?;
    } else {
        println!("cargo::rerun-if-changed=chain_guest_elf_id");
        let chain_guest_elf_id: Digest = (*include_bytes!("chain_guest_elf_id")).into();
        anyhow::ensure!(
            chain_guest_elf_id == chain_guest_entry.image_id.into(),
            "Chain guest ELF ID mismatch. Run with `UPDATE_GUEST_ELF_ID=1` to update."
        );
    }

    // Generate solidity files (for call guest only)
    guests.retain(|entry| entry.name == "risc0_call_guest");
    let solidity_opts = Options::default()
        .with_image_id_sol_path(&image_id_sol_output_path)
        .with_elf_sol_path(&elf_sol_output_path);
    generate_solidity_files(&guests, &solidity_opts)?;

    Ok(())
}

#[cfg(clippy)]
pub fn main() {}
