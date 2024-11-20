use alloy_primitives::Address;
use revm::db::CacheDB;

use crate::config::DEFAULT_CALLER;

static EMPTY_ACCOUNTS: &[Address] = &[DEFAULT_CALLER];

pub fn seed_cache_db_with_trusted_data<D>(db: &mut CacheDB<D>) {
    for address in EMPTY_ACCOUNTS {
        db.insert_account_info(*address, Default::default());
    }
}
