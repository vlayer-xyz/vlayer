use alloy_primitives::Address;
use revm::db::CacheDB;

use crate::config::DEFAULT_CALLER;

static EMPTY_ACCOUNTS: &[Address] = &[DEFAULT_CALLER];

/// Preloads trusted data into the CacheDB to reduce preflight network requests.
///
/// # Safety
/// Every piece of data guest uses should be verified.
/// Accounts and storage are verified against the block hash using proofs.
/// Some accounts are not that important. For example, the default caller account or miner address. Also precompile accounts.
/// Therefore - we decided to hardcode their state here. (it's included in guest_id)
/// The consequences are that if user's code checks the state of those accounts - our simulation will yield different results than the real network.
pub fn seed_cache_db_with_trusted_data<D>(db: &mut CacheDB<D>) {
    for address in EMPTY_ACCOUNTS {
        db.insert_account_info(*address, Default::default());
    }
}
