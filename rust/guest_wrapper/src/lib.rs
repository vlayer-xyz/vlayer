use common::GuestElf;

#[cfg(not(clippy))]
#[allow(dead_code)]
mod private {
    include!(concat!(env!("OUT_DIR"), "/methods.rs"));
}

include!(concat!(env!("OUT_DIR"), "/guest_id.rs"));

#[cfg(not(clippy))]
pub static CALL_GUEST_ELF: GuestElf =
    GuestElf::new(private::RISC0_CALL_GUEST_ID, private::RISC0_CALL_GUEST_ELF);

#[cfg(not(clippy))]
pub static CHAIN_GUEST_ELF: GuestElf =
    GuestElf::new(private::RISC0_CHAIN_GUEST_ID, private::RISC0_CHAIN_GUEST_ELF);

#[cfg(clippy)]
pub static CALL_GUEST_ELF: GuestElf = GuestElf::default();

#[cfg(clippy)]
pub static CHAIN_GUEST_ELF: GuestElf = GuestElf::default();
