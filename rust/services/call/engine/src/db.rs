use std::fmt::Debug;

use alloy_primitives::{Address, U256};
use revm::{db::CacheDB, DatabaseRef};

use crate::config::{
    BASE_FEE_VAULT, DEFAULT_CALLER, L1_BLOCK, L1_FEE_VAULT, OPTIMISM_SEQUENCER_VAULT,
};

static EMPTY_ACCOUNTS: &[Address] =
    &[DEFAULT_CALLER, OPTIMISM_SEQUENCER_VAULT, L1_BLOCK, BASE_FEE_VAULT, L1_FEE_VAULT];

/// Preloads trusted data into the CacheDB to reduce preflight network requests.
///
/// # Safety
/// Every piece of data guest uses should be verified.
/// Accounts and storage are verified against the block hash using proofs.
/// Some accounts are not that important. For example, the default caller account or miner address. Also precompile accounts.
/// Therefore - we decided to hardcode their state here. (It's included in guest_id)
/// The consequences are that if user's code checks the state of those accounts - our simulation will yield different results than the real network.
pub fn seed_cache_db_with_trusted_data<D>(db: &mut CacheDB<D>)
where
    D: DatabaseRef,
    <D as DatabaseRef>::Error: Debug,
{
    for address in EMPTY_ACCOUNTS {
        db.insert_account_info(*address, Default::default());
    }

    let storage_keys = [1u64, 3u64, 5u64, 7u64];
    for key in &storage_keys {
        db.insert_account_storage(L1_BLOCK, U256::from(*key), Default::default())
            .expect("Failed to insert account storage");
    }
}

#[cfg(test)]
mod tests {
    use revm::db::EmptyDB;

    use super::*;

    #[test]
    fn seeds_address_to_some_data() {
        let mut db = CacheDB::new(EmptyDB::default());
        seed_cache_db_with_trusted_data(&mut db);

        for address in EMPTY_ACCOUNTS {
            assert!(db.accounts.get(address).unwrap().info.code.is_some());
        }
    }

    #[test]
    fn seed_storage_of_l1_block_to_zeros() {
        let mut db = CacheDB::new(EmptyDB::default());
        seed_cache_db_with_trusted_data(&mut db);

        let l1_block_storage = db.accounts.get(&L1_BLOCK).unwrap().storage.clone();
        assert_eq!(l1_block_storage.get(&U256::from(1)).unwrap(), &U256::ZERO);
        assert_eq!(l1_block_storage.get(&U256::from(3)).unwrap(), &U256::ZERO);
        assert_eq!(l1_block_storage.get(&U256::from(5)).unwrap(), &U256::ZERO);
        assert_eq!(l1_block_storage.get(&U256::from(7)).unwrap(), &U256::ZERO);
    }
}
