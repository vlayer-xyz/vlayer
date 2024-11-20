use std::sync::Arc;

use alloy_primitives::{BlockNumber, ChainId};
use chain_db::{ChainDb, ChainInfo};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Params {
    chain_id: ChainId,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct ChainStatus {
    first_block: BlockNumber,
    last_block: BlockNumber,
}

impl From<ChainInfo> for ChainStatus {
    fn from(chain_info: ChainInfo) -> Self {
        let block_range = chain_info.block_range();
        Self {
            first_block: block_range.start(),
            last_block: block_range.end(),
        }
    }
}

pub async fn chain_sync_status(
    chain_db: Arc<RwLock<ChainDb>>,
    Params { chain_id }: Params,
) -> Result<ChainStatus, AppError> {
    chain_db
        .read()
        .get_chain_info(chain_id)?
        .ok_or_else(|| AppError::UnsupportedChainId(chain_id))
        .map(Into::into)
}

#[cfg(test)]
mod tests {

    use chain_db::ChainUpdate;
    use u64_range::NonEmptyRange;

    use super::*;

    #[tokio::test]
    async fn empty_db() {
        let params = Params { chain_id: 1 };
        let chain_db = Arc::new(RwLock::new(ChainDb::in_memory()));
        assert_eq!(
            chain_sync_status(chain_db, params).await.unwrap_err(),
            AppError::UnsupportedChainId(1)
        );
    }

    #[tokio::test]
    async fn single_block() {
        let params = Params { chain_id: 1 };
        let chain_db = Arc::new(RwLock::new(ChainDb::in_memory()));
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
            chain_sync_status(chain_db, params).await.unwrap(),
            ChainStatus {
                first_block: 0,
                last_block: 0
            }
        );
    }
}
