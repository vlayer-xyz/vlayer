fn main() {
    #[cfg(not(clippy))]
    guest_wrapper_utils::build_risc0_guest().unwrap();
}
