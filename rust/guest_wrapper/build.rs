#[cfg(not(clippy))]
pub fn main() -> anyhow::Result<()> {
    use std::{collections::HashMap, env, fs, io, path::{PathBuf, Path}};

    use risc0_build::{embed_methods_with_options, DockerOptions, GuestOptions};
    use risc0_build_ethereum::{generate_solidity_files, Options};

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
