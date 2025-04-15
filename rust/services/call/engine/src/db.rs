use std::collections::HashMap;

use alloy_primitives::Address;
use call_common::RevmDB;
use revm::db::CacheDB;

use crate::config::{ACCOUNT_TO_STORAGE, EMPTY_ACCOUNTS, Storage};

/// Preloads trusted data into the CacheDB to reduce preflight network requests.
///
/// # Safety
/// Every piece of data guest uses should be verified.
/// Accounts and storage are verified against the block hash using proofs.
/// Some accounts are not that important. For example, the default caller account or miner address. Also precompile accounts.
/// Therefore - we decided to hardcode their state here. (It's included in guest_id)
/// The consequences are that if user's code checks the state of those accounts - our simulation will yield different results than the real network.
pub fn seed_cache_db_with_trusted_data<D: RevmDB>(db: &mut CacheDB<D>) {
    seed_accounts_info(db, EMPTY_ACCOUNTS);
    seed_storage(db, &ACCOUNT_TO_STORAGE.clone());
}

fn seed_accounts_info<D: RevmDB>(db: &mut CacheDB<D>, accounts: &[Address]) {
    for account in accounts {
        db.insert_account_info(*account, Default::default());
    }
}

fn seed_storage<D: RevmDB>(db: &mut CacheDB<D>, account_to_storage: &HashMap<Address, Storage>) {
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
    use crate::config::{BASE_FEE_VAULT, L1_BLOCK};

    #[test]
    fn success() {
        let mut db = CacheDB::new(EmptyDB::default());
        seed_cache_db_with_trusted_data(&mut db);

        let base_fee_vault_balance = db.accounts.get(&BASE_FEE_VAULT).unwrap().info.balance;

        let l1_block_storage = db.accounts.get(&L1_BLOCK).unwrap().storage.clone();
        let latest_l1_block = l1_block_storage.get(&U256::from(1)).unwrap();

        assert_eq!(base_fee_vault_balance, U256::ZERO);
        assert_eq!(latest_l1_block, &U256::ZERO);
    }
}
