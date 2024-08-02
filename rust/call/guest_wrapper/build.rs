fn main() {
    #[cfg(not(clippy))]
    {
        use risc0_build_ethereum::generate_solidity_files;
        use std::env;

        // relative path, where to put ImageID.sol file
        const SOLIDITY_IMAGE_ID_PATH: &str = "../../contracts/src/ImageID.sol";

        // Generate Solidity source files for use with Forge.
        let out_dir = env::var_os("OUT_DIR").unwrap().into_string().unwrap();
        let solidity_opts = risc0_build_ethereum::Options::default()
            .with_image_id_sol_path(SOLIDITY_IMAGE_ID_PATH)
            .with_elf_sol_path(format!("{out_dir}/Elf.sol"));

        let guests = risc0_build::embed_methods();
        generate_solidity_files(guests.as_slice(), &solidity_opts).unwrap();
    }
}
