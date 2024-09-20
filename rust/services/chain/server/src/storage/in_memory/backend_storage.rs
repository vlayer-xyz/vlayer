use alloy_primitives::Bytes;
use std::collections::HashMap;

use crate::storage::block_storage::BlockStorageBackend;

type ChainId = String;

pub(crate) struct InMemoryChainBlockStorageBackend {
    hash_to_node: HashMap<String, Vec<u8>>,
    block_range_to_root: HashMap<(u32, u32), String>,
    block_range_to_proof: HashMap<(u32, u32), Bytes>,
    current_block_range: (u32, u32),
}

impl InMemoryChainBlockStorageBackend {
    pub fn new(block_id: u32) -> Self {
        InMemoryChainBlockStorageBackend {
            hash_to_node: HashMap::new(),
            block_range_to_root: HashMap::new(),
            block_range_to_proof: HashMap::new(),
            current_block_range: (block_id, block_id),
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

pub(crate) struct InMemoryBlockStorageBackend {
    chain_storages: HashMap<ChainId, InMemoryChainBlockStorageBackend>,
}

impl InMemoryBlockStorageBackend {
    #[allow(unused)]
    pub fn new() -> Self {
        InMemoryBlockStorageBackend {
            chain_storages: HashMap::new(),
        }
    }

    /// Initializes the chain storage with the given `chain_id` and `block_id`.
    /// This must be called before any operations on that chain.
    pub fn init_chain_storage(&mut self, chain_id: &str, block_id: u32) {
        self.chain_storages
            .entry(chain_id.to_string())
            .or_insert_with(|| InMemoryChainBlockStorageBackend::new(block_id));
    }
}

impl BlockStorageBackend for InMemoryBlockStorageBackend {
    fn insert_hash_to_node(&mut self, chain_id: &str, hash: &str, node: &[u8]) {
        let chain_storage = self
            .chain_storages
            .get_mut(chain_id)
            .expect("Chain storage not initialized. Call `init_chain_storage` first.");
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
            .get_mut(chain_id)
            .expect("Chain storage not initialized. Call `init_chain_storage` first.");
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
            .get_mut(chain_id)
            .expect("Chain storage not initialized. Call `init_chain_storage` first.");
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
            .get_mut(chain_id)
            .expect("Chain storage not initialized. Call `init_chain_storage` first.");
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
