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

#[cfg(not(clippy))]
pub static CHAIN_GUEST_ELF_WITH_CANONICAL_ID: GuestElf =
    GuestElf::new(to_u32_array(CHAIN_GUEST_IDS[0]), private::RISC0_CHAIN_GUEST_ELF);

#[cfg(not(clippy))]
const fn to_u32_array(bytes: [u8; 32]) -> [u32; 8] {
    let mut arr = [0u32; 8];
    let mut i = 0;
    while i < 8 {
        arr[i] = u32::from_le_bytes([
            bytes[i * 4],
            bytes[i * 4 + 1],
            bytes[i * 4 + 2],
            bytes[i * 4 + 3],
        ]);
        i += 1;
    }
    arr
}

#[cfg(clippy)]
pub static CALL_GUEST_ELF: GuestElf = GuestElf::default();

#[cfg(clippy)]
pub static CHAIN_GUEST_ELF: GuestElf = GuestElf::default();

#[cfg(clippy)]
pub static CHAIN_GUEST_ELF_WITH_CANONICAL_ID: GuestElf = GuestElf::default();
