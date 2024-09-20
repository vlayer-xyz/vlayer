use alloy_primitives::Bytes;

#[allow(unused)]
pub(crate) trait BlockStorageBackend {
    fn insert_hash_to_node(&mut self, chain_id: &str, hash: &str, node: &[u8]);
    fn get_node_by_hash(&self, chain_id: &str, hash: &str) -> Option<Vec<u8>>;
    fn insert_block_range_to_root(&mut self, chain_id: &str, range: (u32, u32), root: &str);
    fn get_root_by_block_range(&self, chain_id: &str, range: &(u32, u32)) -> Option<String>;
    fn insert_block_range_to_proof(&mut self, chain_id: &str, range: (u32, u32), proof: Bytes);
    fn get_proof_by_block_range(&self, chain_id: &str, range: &(u32, u32)) -> Option<Bytes>;
    fn set_current_block_range(&mut self, chain_id: &str, range: (u32, u32));
    fn get_current_block_range(&self, chain_id: &str) -> (u32, u32);
}
