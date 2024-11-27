use std::{collections::HashMap, fmt::Debug};

use alloy_primitives::Address;
use revm::{db::CacheDB, DatabaseRef};

use crate::config::{
    Storage, ACCOUNT_TO_STORAGE, BASE_FEE_VAULT, DEFAULT_CALLER, L1_BLOCK, L1_FEE_VAULT,
    OPTIMISM_SEQUENCER_VAULT,
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
    insert_accounts_info(db, EMPTY_ACCOUNTS);
    insert_account_storages(db, &ACCOUNT_TO_STORAGE.clone());
}

fn insert_accounts_info<D>(db: &mut CacheDB<D>, accounts: &[Address])
where
    D: DatabaseRef,
{
    for account in accounts {
        db.insert_account_info(*account, Default::default());
    }
}

fn insert_account_storages<D>(db: &mut CacheDB<D>, account_to_storage: &HashMap<Address, Storage>)
where
    D: DatabaseRef,
    <D as DatabaseRef>::Error: Debug,
{
    for (&account, storage) in account_to_storage {
        for (&key, &value) in storage {
            db.insert_account_storage(account, key, value)
                .expect("failed to insert account storage");
        }
    }
}

#[cfg(test)]
mod seed_cache_db_with_trusted_data {
    use alloy_primitives::U256;
    use revm::db::EmptyDB;

    use super::*;

    #[test]
    fn success() {
        let mut db = CacheDB::new(EmptyDB::default());
        seed_cache_db_with_trusted_data(&mut db);

        let l1_block_storage = db.accounts.get(&L1_BLOCK).unwrap().storage.clone();

        assert!(db
            .accounts
            .get(&BASE_FEE_VAULT)
            .unwrap()
            .info
            .code
            .is_some());
        assert_eq!(l1_block_storage.get(&U256::from(1)).unwrap(), &U256::ZERO);
    }
}
