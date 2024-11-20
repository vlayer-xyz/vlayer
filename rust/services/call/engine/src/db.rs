use alloy_primitives::address;
use revm::db::CacheDB;

pub fn seed_cache_db_with_trusted_data<D>(db: &mut CacheDB<D>) {
    db.insert_account_info(
        address!("1111111111111111111111111111111111111111"),
        Default::default(),
    );
}
