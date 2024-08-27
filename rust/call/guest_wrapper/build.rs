fn main() {
    #[cfg(not(clippy))]
    {
        use risc0_build_ethereum::generate_solidity_files;
        use std::fs;
        use std::path::Path;

        // Generate Solidity source files for use with Forge.
        let assets_path_str = format!("../../target/assets");
        let assets_path = Path::new(&assets_path_str);

        if !assets_path.exists() {
            println!("{:?}", assets_path);
            fs::create_dir(assets_path).expect("Could not create assets directory");
        }

        let image_id_path = assets_path.join("ImageID.sol");
        let elf_path = assets_path.join("Elf.sol");

        fs::remove_file(&image_id_path)
            .unwrap_or_else(|err| println!("Did not remove old ImageID file: {}", err));
        fs::remove_file(&elf_path)
            .unwrap_or_else(|err| println!("Did not remove old elf file: {}", err));

        let solidity_opts = risc0_build_ethereum::Options::default()
            .with_image_id_sol_path(image_id_path)
            .with_elf_sol_path(elf_path);

        let guests = risc0_build::embed_methods();
        generate_solidity_files(guests.as_slice(), &solidity_opts).unwrap();
    }
}
