use std::sync::Arc;

use chain_common::{GetSyncStatus, SyncStatus};
use chain_db::ChainDb;
use parking_lot::RwLock;

use crate::error::AppError;

pub async fn v_sync_status(
    chain_db: Arc<RwLock<ChainDb>>,
    GetSyncStatus { chain_id }: GetSyncStatus,
) -> Result<SyncStatus, AppError> {
    chain_db
        .read()
        .get_chain_info(chain_id)?
        .ok_or(AppError::UnsupportedChainId(chain_id))
        .map(Into::into)
}

#[cfg(test)]
mod tests {

    use chain_db::{ChainInfo, ChainUpdate};
    use common::GuestElf;
    use u64_range::NonEmptyRange;

    use super::*;

    #[tokio::test]
    async fn empty_db() {
        let params = GetSyncStatus { chain_id: 1 };
        let chain_db = Arc::new(RwLock::new(ChainDb::in_memory(GuestElf::default())));
        assert_eq!(
            v_sync_status(chain_db, params).await.unwrap_err(),
            AppError::UnsupportedChainId(1)
        );
    }

    #[tokio::test]
    async fn single_block() {
        let params = GetSyncStatus { chain_id: 1 };
        let chain_db = Arc::new(RwLock::new(ChainDb::in_memory(GuestElf::default())));
        let chain_info = ChainInfo::new(
            NonEmptyRange::from_single_value(0),
            Default::default(),
            Default::default(),
        );
        chain_db
            .write()
            .update_chain(1, ChainUpdate::new(chain_info, [], []))
            .expect("update_chain failed");
        assert_eq!(
            v_sync_status(chain_db, params).await.unwrap(),
            SyncStatus {
                first_block: 0,
                last_block: 0
            }
        );
    }
}
