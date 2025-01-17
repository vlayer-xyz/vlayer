use guest_build_utils::ChainGuestId;

pub fn main() {
    ChainGuestId::default().generate_rust().unwrap();
}
