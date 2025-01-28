use alloy_primitives::ChainId;
use chain::{ChainSpec, OptimismSpec};

use super::{Error, Result};

pub fn ensure_teleport_possible(
    source_chain_id: ChainId,
    dest_chain_id: ChainId,
) -> Result<OptimismSpec> {
    let dest_chain_spec: ChainSpec = dest_chain_id.try_into().unwrap();
    let Some(op_spec) = dest_chain_spec.op_spec() else {
        return Err(Error::NotAnOptimism(dest_chain_spec.id()));
    };
    if op_spec.anchor_chain() != source_chain_id {
        return Err(Error::WrongAnchor {
            src: source_chain_id,
            dest: dest_chain_spec.id(),
            anchor: op_spec.anchor_chain(),
        });
    }
    Ok(op_spec)
}

#[cfg(test)]
mod ensure_teleport_possible {
    use alloy_primitives::{address, Address};

    use super::*;

    const OP_MAINNET: ChainId = 10;
    const ETHEREUM_MAINNET: ChainId = 1;
    const ETHEREUM_SEPOLIA: ChainId = 11_155_111;
    const ANCHOR_STATE_REGISTRY_ADDRESS: Address =
        address!("18dac71c228d1c32c99489b7323d441e1175e443");

    #[test]
    fn optimism_mainnet_commits_to_eth_mainnet() -> anyhow::Result<()> {
        let registry = ensure_teleport_possible(ETHEREUM_MAINNET, OP_MAINNET)?;

        assert_eq!(registry.anchor_state_registry(), ANCHOR_STATE_REGISTRY_ADDRESS);
        Ok(())
    }

    #[test]
    fn optimism_mainnet_doesnt_commit_to_eth_sepolia() -> anyhow::Result<()> {
        let result = ensure_teleport_possible(ETHEREUM_SEPOLIA, OP_MAINNET);

        assert!(matches!(
            result,
            Err(Error::WrongAnchor {
                src: ETHEREUM_SEPOLIA,
                dest: OP_MAINNET,
                anchor: ETHEREUM_MAINNET
            })
        ));
        Ok(())
    }
}
