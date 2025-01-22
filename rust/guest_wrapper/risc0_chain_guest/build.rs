use guest_build_utils::chain_guest_id;

pub fn main() {
    // Do **NOT** include current elf_id, as this would generate
    // an infinite build loop. Image cannot contain its own ID.
    chain_guest_id::generate_rust(false).unwrap();
}
