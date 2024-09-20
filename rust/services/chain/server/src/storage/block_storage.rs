use alloy_primitives::Bytes;

#[allow(unused)]
pub(crate) trait BlockStorage {
    fn get_proof(&self, chain_id: &str, start: u32, end: u32) -> Option<Bytes>;
}
