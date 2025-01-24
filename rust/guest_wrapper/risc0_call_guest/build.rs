use guest_build_utils::chain_guest_id;

pub fn main() {
    chain_guest_id::generate_rust(true).unwrap();
}
