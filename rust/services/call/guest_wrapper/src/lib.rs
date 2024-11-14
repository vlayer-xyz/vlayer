use common::GuestElf;

#[cfg(not(clippy))]
#[allow(dead_code)]
mod private {
    include!(concat!(env!("OUT_DIR"), "/methods.rs"));
}

#[cfg(not(clippy))]
pub static GUEST: GuestElf =
    GuestElf::new(private::RISC0_CALL_GUEST_ID, private::RISC0_CALL_GUEST_ELF);
#[cfg(clippy)]
pub static GUEST: GuestElf = GuestElf::default();
