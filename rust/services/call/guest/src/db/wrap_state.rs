use std::{
    collections::HashMap,
    convert::Infallible,
    sync::{Arc, RwLock},
};

use alloy_primitives::{Address, B256, U256, keccak256};
use call_engine::evm::input::EvmInput;
use mpt::KeccakMerkleTrie as MerkleTrie;
#[allow(clippy::disallowed_types)]
use revm::{
    DatabaseRef,
    primitives::{AccountInfo, Bytecode},
};

use super::state::StateDb;

#[derive(Default, Debug)]
pub struct WrapStateDb {
    inner: StateDb,
    account_storage: RwLock<HashMap<Address, Option<Arc<MerkleTrie>>>>,
}

impl WrapStateDb {
    /// Creates a new [Database] from the given [StateDb].
    pub(crate) fn new(inner: StateDb) -> Self {
        Self {
            inner,
            account_storage: RwLock::new(HashMap::new()),
        }
    }
}

#[allow(clippy::expect_used)]
impl DatabaseRef for WrapStateDb {
    /// The database does not return any errors.
    type Error = Infallible;

    /// Get basic account information.
    fn basic_ref(&self, address: Address) -> Result<Option<AccountInfo>, Self::Error> {
        let account = self.inner.account(address);
        match account {
            Some(account) => {
                // link storage trie to the account, if it exists
                if let Some(storage_trie) = self.inner.storage_trie(&account.storage_root) {
                    self.account_storage
                        .write()
                        .expect("poisoned lock")
                        .insert(address, Some(storage_trie.clone()));
                }

                Ok(Some(AccountInfo {
                    balance: account.balance,
                    nonce: account.nonce,
                    code_hash: account.code_hash,
                    code: None, // we don't need the code here, `code_by_hash` will be used instead
                }))
            }
            None => {
                self.account_storage
                    .write()
                    .expect("poisoned lock")
                    .insert(address, None);

                Ok(None)
            }
        }
    }

    /// Get account code by its hash.
    fn code_by_hash_ref(&self, code_hash: B256) -> Result<Bytecode, Self::Error> {
        let code = self.inner.code_by_hash(code_hash);
        Ok(Bytecode::new_raw(code.clone()))
    }

    /// Get storage value of address at index.
    #[allow(clippy::panic)]
    fn storage_ref(&self, address: Address, index: U256) -> Result<U256, Self::Error> {
        let account_storage = self.account_storage.read().expect("poisoned lock");
        let storage = account_storage
            .get(&address)
            .unwrap_or_else(|| panic!("storage not found: {address:?}@{index}"));
        match storage {
            Some(storage) => {
                let val = storage
                    .get_rlp(keccak256(index.to_be_bytes::<32>()))
                    .expect("invalid storage value");
                Ok(val.unwrap_or_default())
            }
            None => Ok(U256::ZERO),
        }
    }

    /// Get block hash by block number.
    fn block_hash_ref(&self, number: u64) -> Result<B256, Self::Error> {
        Ok(self.inner.block_hash(number))
    }
}

impl From<EvmInput> for WrapStateDb {
    fn from(input: EvmInput) -> Self {
        let block_hashes = input.block_hashes();
        let state_db =
            StateDb::new(input.state_trie, input.storage_tries, input.contracts, block_hashes);

        WrapStateDb::new(state_db)
    }
}

#[cfg(test)]
mod storage_ref {
    use super::*;

    #[test]
    #[should_panic(expected = "storage not found: 0x0000000000000000000000000000000000000000@0")]
    fn panics_when_storage_not_found() {
        let db = WrapStateDb::default();
        let _ = db.storage_ref(Address::default(), U256::from(0));
    }

    #[test]
    #[should_panic(expected = "invalid storage value")]
    fn panics_when_storage_value_invalid() {
        let db = WrapStateDb::default();
        let address = Address::default();
        let index = U256::from(0);

        let invalid_value = vec![0xc0];
        let storage =
            MerkleTrie::from_iter(vec![(keccak256(index.to_be_bytes::<32>()), invalid_value)]);

        db.account_storage
            .write()
            .expect("poisoned lock")
            .insert(address, Some(Arc::new(storage)));

        db.storage_ref(address, index).unwrap();
    }

    #[test]
    fn returns_zero_when_storage_not_found() {
        let db = WrapStateDb::default();
        let address = Address::default();
        let index = U256::from(0);

        db.account_storage
            .write()
            .expect("poisoned lock")
            .insert(address, None);

        let value = db.storage_ref(address, index).unwrap();

        assert_eq!(value, U256::ZERO);
    }

    #[test]
    fn success() {
        let db = WrapStateDb::default();
        let address = Address::default();
        let index = U256::from(0);

        let mut storage = MerkleTrie::default();
        storage
            .insert(keccak256(index.to_be_bytes::<32>()), [42])
            .unwrap();

        db.account_storage
            .write()
            .expect("poisoned lock")
            .insert(address, Some(Arc::new(storage)));

        let value = db.storage_ref(address, index).unwrap();

        assert_eq!(value, U256::from(42));
    }
}
