use std::{cell::RefCell, convert::Infallible, rc::Rc};

use alloy_primitives::{keccak256, Address, B256, U256};
use mpt::MerkleTrie;
use revm::{
    primitives::{AccountInfo, Bytecode, HashMap},
    DatabaseRef,
};

use super::state::StateDb;

pub struct WrapStateDb {
    inner: StateDb,
    account_storage: RefCell<HashMap<Address, Option<Rc<MerkleTrie>>>>,
}

impl WrapStateDb {
    /// Creates a new [Database] from the given [StateDb].
    pub(crate) fn new(inner: StateDb) -> Self {
        Self {
            inner,
            account_storage: RefCell::new(HashMap::new()),
        }
    }
}

impl DatabaseRef for WrapStateDb {
    /// The database does not return any errors.
    type Error = Infallible;

    /// Get basic account information.
    #[inline]
    fn basic_ref(&self, address: Address) -> Result<Option<AccountInfo>, Self::Error> {
        let account = self.inner.account(address);
        match account {
            Some(account) => {
                // link storage trie to the account, if it exists
                if let Some(storage_trie) = self.inner.storage_trie(&account.storage_root) {
                    self.account_storage
                        .borrow_mut()
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
                self.account_storage.borrow_mut().insert(address, None);

                Ok(None)
            }
        }
    }

    /// Get account code by its hash.
    #[inline]
    fn code_by_hash_ref(&self, code_hash: B256) -> Result<Bytecode, Self::Error> {
        let code = self.inner.code_by_hash(code_hash);
        Ok(Bytecode::new_raw(code.clone()))
    }

    /// Get storage value of address at index.
    #[inline]
    fn storage_ref(&self, address: Address, index: U256) -> Result<U256, Self::Error> {
        let account_storage = self.account_storage.borrow();
        let storage = account_storage
            .get(&address)
            .unwrap_or_else(|| panic!("storage not found: {:?}", address));
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
    #[inline]
    fn block_hash_ref(&self, number: U256) -> Result<B256, Self::Error> {
        Ok(self.inner.block_hash(number))
    }
}
