use std::collections::HashMap;

use alloy_primitives::Bytes;

use super::block_storage_backend::BlockStorageBackend;

type ChainId = String;

pub struct InMemoryChainBlockStorage {
    hash_to_node: HashMap<String, Vec<u8>>,
    block_range_to_root: HashMap<(u32, u32), String>,
    block_range_to_proof: HashMap<(u32, u32), Bytes>,
    current_block_range: (u32, u32),
}

impl InMemoryChainBlockStorage {
    pub fn new() -> Self {
        InMemoryChainBlockStorage {
            hash_to_node: HashMap::new(),
            block_range_to_root: HashMap::new(),
            block_range_to_proof: HashMap::new(),
            current_block_range: (0, 0),
        }
    }

    pub fn insert_hash_to_node(&mut self, hash: &str, node: &[u8]) {
        self.hash_to_node.insert(hash.to_string(), node.to_vec());
    }

    pub fn get_node_by_hash(&self, hash: &str) -> Option<Vec<u8>> {
        self.hash_to_node.get(hash).cloned()
    }

    pub fn insert_block_range_to_root(&mut self, range: (u32, u32), root: &str) {
        self.block_range_to_root.insert(range, root.to_string());
    }

    pub fn get_root_by_block_range(&self, range: &(u32, u32)) -> Option<String> {
        self.block_range_to_root.get(range).cloned()
    }

    pub fn insert_block_range_to_proof(&mut self, range: (u32, u32), proof: Bytes) {
        self.block_range_to_proof.insert(range, proof);
    }

    pub fn get_proof_by_block_range(&self, range: &(u32, u32)) -> Option<Bytes> {
        self.block_range_to_proof.get(range).cloned()
    }

    pub fn set_current_block_range(&mut self, range: (u32, u32)) {
        self.current_block_range = range;
    }

    pub fn get_current_block_range(&self) -> (u32, u32) {
        self.current_block_range
    }
}

pub(crate) struct InMemoryBlockStorage {
    chain_storages: HashMap<ChainId, InMemoryChainBlockStorage>,
}

impl InMemoryBlockStorage {
    #[allow(unused)]
    pub fn new() -> Self {
        InMemoryBlockStorage {
            chain_storages: HashMap::new(),
        }
    }
}

impl BlockStorageBackend for InMemoryBlockStorage {
    fn insert_hash_to_node(&mut self, chain_id: &str, hash: &str, node: &[u8]) {
        let chain_storage = self
            .chain_storages
            .entry(chain_id.to_string())
            .or_insert_with(InMemoryChainBlockStorage::new);
        chain_storage.insert_hash_to_node(hash, node);
    }

    fn get_node_by_hash(&self, chain_id: &str, hash: &str) -> Option<Vec<u8>> {
        self.chain_storages
            .get(chain_id)
            .and_then(|chain_storage| chain_storage.get_node_by_hash(hash))
    }

    fn insert_block_range_to_root(&mut self, chain_id: &str, range: (u32, u32), root: &str) {
        let chain_storage = self
            .chain_storages
            .entry(chain_id.to_string())
            .or_insert_with(InMemoryChainBlockStorage::new);
        chain_storage.insert_block_range_to_root(range, root);
    }

    fn get_root_by_block_range(&self, chain_id: &str, range: &(u32, u32)) -> Option<String> {
        self.chain_storages
            .get(chain_id)
            .and_then(|chain_storage| chain_storage.get_root_by_block_range(range))
    }

    fn insert_block_range_to_proof(&mut self, chain_id: &str, range: (u32, u32), proof: Bytes) {
        let chain_storage = self
            .chain_storages
            .entry(chain_id.to_string())
            .or_insert_with(InMemoryChainBlockStorage::new);
        chain_storage.insert_block_range_to_proof(range, proof);
    }

    fn get_proof_by_block_range(&self, chain_id: &str, range: &(u32, u32)) -> Option<Bytes> {
        self.chain_storages
            .get(chain_id)
            .and_then(|chain_storage| chain_storage.get_proof_by_block_range(range))
    }

    fn set_current_block_range(&mut self, chain_id: &str, range: (u32, u32)) {
        let chain_storage = self
            .chain_storages
            .entry(chain_id.to_string())
            .or_insert_with(InMemoryChainBlockStorage::new);
        chain_storage.set_current_block_range(range);
    }

    fn get_current_block_range(&self, chain_id: &str) -> (u32, u32) {
        self.chain_storages
            .get(chain_id)
            .map_or((0, 0), |chain_storage| {
                chain_storage.get_current_block_range()
            })
    }
}
