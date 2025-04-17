#[cfg(test)]
mod ensure_latest_teleport_location_is_confirmed {
    use alloy_primitives::{B256, ChainId};
    use anyhow::Result;

    use crate::verifier::teleport::{Error, ensure_latest_teleport_location_is_confirmed};
    const CHAIN_ID: ChainId = 1;

    #[test]
    fn success() -> Result<()> {
        let hash = B256::ZERO;
        let blocks = &[(1, hash), (2, hash)];

        ensure_latest_teleport_location_is_confirmed(blocks, 2, CHAIN_ID)?;
        Ok(())
    }

    #[test]
    fn unconfirmed() -> Result<()> {
        let hash = B256::ZERO;
        let blocks = &[(1, hash), (2, hash)];

        let err = ensure_latest_teleport_location_is_confirmed(blocks, 1, CHAIN_ID).unwrap_err();
        assert_eq!(
            err,
            Error::TeleportOnUnconfirmed {
                target_block: 2,
                chain_id: CHAIN_ID,
                latest_confirmed_block: 1
            }
        );
        Ok(())
    }
}

#[cfg(test)]
mod get_destinations {
    use std::collections::HashMap;

    use alloy_primitives::{B256, BlockHash, BlockNumber, ChainId};

    use crate::{evm::env::BlocksByChain, verifier::teleport::get_destinations};

    #[test]
    fn success_no_teleport() {
        let chain_id: ChainId = 1;
        let block: BlockNumber = 1;
        let blocks_by_chain: BlocksByChain =
            HashMap::from([(chain_id, vec![(block, B256::ZERO)])]).into();
        let start_exec_location = (chain_id, block).into();

        let destinations: HashMap<ChainId, Vec<(BlockNumber, BlockHash)>> =
            get_destinations(blocks_by_chain, start_exec_location).collect();

        assert_eq!(destinations.len(), 0);
    }

    #[test]
    fn success_teleport() {
        let src_chain_id: ChainId = 1;
        let src_block: BlockNumber = 1;
        let dest_chain_id: ChainId = 10;
        let dest_block: BlockNumber = 1;
        let blocks_by_chain: BlocksByChain = HashMap::from([
            (src_chain_id, vec![(src_block, B256::ZERO)]),
            (dest_chain_id, vec![(dest_block, B256::ZERO)]),
        ])
        .into();
        let start_exec_location = (src_chain_id, src_block).into();

        let destinations: HashMap<ChainId, Vec<(BlockNumber, BlockHash)>> =
            get_destinations(blocks_by_chain, start_exec_location).collect();

        assert_eq!(destinations.len(), 1);
        assert_eq!(destinations[&dest_chain_id], vec![(dest_block, B256::ZERO)]);
    }
}
