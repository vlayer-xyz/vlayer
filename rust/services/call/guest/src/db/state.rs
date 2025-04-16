use std::{collections::HashMap, sync::Arc};

use alloy_primitives::{Address, B256, Bytes, TxNumber, U256, b256, keccak256};
use alloy_rlp_derive::{RlpDecodable, RlpEncodable};
use block_header::Hashable;
use mpt::KeccakMerkleTrie as MerkleTrie;

/// Hash of an empty byte array, i.e. `keccak256([])`.
pub const KECCAK_EMPTY: B256 =
    b256!("c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470");

/// Represents an account within the state trie.
#[derive(Debug, Clone, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct StateAccount {
    /// The number of transactions sent from this account's address.
    pub nonce: TxNumber,
    /// The number of Wei owned by this account's address.
    pub balance: U256,
    /// The root of the account's storage trie.
    pub storage_root: B256,
    /// The hash of the EVM code of this account.
    pub code_hash: B256,
}

impl Default for StateAccount {
    /// Provides default values for a [StateAccount].
    fn default() -> Self {
        Self {
            nonce: 0,
            balance: U256::ZERO,
            storage_root: mpt::EMPTY_ROOT_HASH,
            code_hash: KECCAK_EMPTY,
        }
    }
}

/// A simple read-only EVM database.
///
/// It is backed by a single [MerkleTrie] for the accounts and one [MerkleTrie] each for the
/// accounts' storages. It panics when data is queried that is not contained in the tries.
#[derive(Default, Debug)]
pub struct StateDb {
    state_trie: MerkleTrie,
    storage_tries: HashMap<B256, Arc<MerkleTrie>>,
    contracts: HashMap<B256, Bytes>,
    block_hashes: HashMap<u64, B256>,
}

impl StateDb {
    /// Creates a new state database from the given tries.
    pub fn new(
        state_trie: MerkleTrie,
        storage_tries: impl IntoIterator<Item = MerkleTrie>,
        contracts: impl IntoIterator<Item = Bytes>,
        block_hashes: HashMap<u64, B256>,
    ) -> Self {
        let contracts = contracts
            .into_iter()
            .map(|code| (keccak256(&code), code))
            .collect();
        let storage_tries = storage_tries
            .into_iter()
            .map(|trie| (trie.hash_slow(), Arc::new(trie)))
            .collect();
        Self {
            state_trie,
            contracts,
            storage_tries,
            block_hashes,
        }
    }

    #[allow(clippy::expect_used)]
    pub fn account(&self, address: Address) -> Option<StateAccount> {
        self.state_trie
            .get_rlp(keccak256(address))
            .expect("invalid state value")
    }

    #[allow(clippy::panic)]
    pub fn code_by_hash(&self, hash: B256) -> &Bytes {
        self.contracts
            .get(&hash)
            .unwrap_or_else(|| panic!("code not found: {hash}"))
    }

    #[allow(clippy::panic)]
    pub fn block_hash(&self, number: u64) -> B256 {
        let hash = self
            .block_hashes
            .get(&number)
            .unwrap_or_else(|| panic!("block not found: {number}"));
        *hash
    }

    pub fn storage_trie(&self, root: &B256) -> Option<&Arc<MerkleTrie>> {
        self.storage_tries.get(root)
    }
}
