/// The code in this module is a skeleton and is not up to our quality standards.
use std::{collections::HashMap, fmt::Debug};

use alloy_primitives::{BlockHash, BlockNumber, ChainId, B256, U256};
use anyhow::anyhow;
use async_trait::async_trait;
use chain::{ChainSpec, OptimismSpec};
use common::Hashable;
use derive_more::Deref;
use derive_new::new;
use lazy_static::lazy_static;
use optimism::{OpRpcClient, OutputResponse};
use revm::DatabaseRef;

use crate::evm::env::{cached::CachedEvmEnv, location::ExecutionLocation, BlocksByChain};

mod optimism;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("EvmEnvFactory: {0}")]
    Factory(#[from] crate::evm::env::factory::Error),
    #[error("Output hash mismatch")]
    L2OutputHashMismatch,
    #[error("Header hash mismatch")]
    HeaderHashMismatch,
    #[error("Teleport on unconfirmed")]
    TeleportOnUnconfirmed,
    #[error("Database error: {0}")]
    Database(anyhow::Error),
    #[error("Teleport from chain {src} to chain {dest} is not supported as it is anchored into {anchor}")]
    WrongAnchor {
        src: ChainId,
        dest: ChainId,
        anchor: ChainId,
    },
    #[error("Can't teleport onto {0} as it is not an optimistic chain")]
    NotAnOptimism(ChainId),
}

pub type Result<T> = std::result::Result<T, Error>;
mod seal {
    pub trait Sealed<D> {}
}

#[cfg(any(test, feature = "testing"))]
impl<F, D> seal::Sealed<D> for F
where
    D: DatabaseRef,
    F: Fn(&CachedEvmEnv<D>, ExecutionLocation) -> Result<()> + Send + Sync,
{
}

#[async_trait]
pub trait IVerifier<D>: seal::Sealed<D> + Send + Sync
where
    D: DatabaseRef + Send + Sync,
{
    async fn verify(
        &self,
        evm_envs: &CachedEvmEnv<D>,
        start_exec_location: ExecutionLocation,
    ) -> Result<()>;
}

#[cfg(any(test, feature = "testing"))]
#[async_trait]
impl<F, D> IVerifier<D> for F
where
    D: DatabaseRef + Send + Sync,
    F: Fn(&CachedEvmEnv<D>, ExecutionLocation) -> Result<()> + Send + Sync,
{
    async fn verify(
        &self,
        evm_envs: &CachedEvmEnv<D>,
        start_exec_location: ExecutionLocation,
    ) -> Result<()> {
        self(evm_envs, start_exec_location)
    }
}

#[derive(new, Deref, Default)]
struct MultiOpRpcClient {
    clients: HashMap<ChainId, Box<dyn OpRpcClient>>,
}

#[derive(Default)]
pub struct Verifier {
    multi_op_rpc_client: MultiOpRpcClient,
}

impl<D> seal::Sealed<D> for Verifier {}
#[async_trait]
impl<D> IVerifier<D> for Verifier
where
    D: DatabaseRef + Send + Sync,
    D::Error: std::error::Error + Send + Sync + 'static,
{
    async fn verify(
        &self,
        evm_envs: &CachedEvmEnv<D>,
        start_exec_location: ExecutionLocation,
    ) -> Result<()> {
        let source_evm_env = evm_envs.get(start_exec_location)?;
        let blocks_by_chain = evm_envs.blocks_by_chain();
        let destinations = get_destinations(blocks_by_chain, start_exec_location);
        for (chain_id, blocks) in destinations {
            let l2_output = get_l2_output(
                start_exec_location.chain_id,
                &source_evm_env.db,
                chain_id,
                &self.multi_op_rpc_client,
            )
            .await?;

            let latest_confirmed_location =
                (chain_id, l2_output.block_ref.l1_block_info.number).into();
            let latest_confirmed_evm_env = evm_envs.get(latest_confirmed_location)?;
            if latest_confirmed_evm_env.header.hash_slow() != l2_output.block_ref.l1_block_info.hash
            {
                return Err(Error::HeaderHashMismatch);
            }
            ensure_latest_teleport_location_is_confirmed(
                &blocks,
                l2_output.block_ref.l1_block_info.number,
            )?;
        }
        Ok(())
    }
}

fn ensure_latest_teleport_location_is_confirmed(
    destination_blocks: &[(u64, B256)],
    latest_confirmed_block: BlockNumber,
) -> Result<()> {
    let latest_destination_block = destination_blocks
        .iter()
        .max()
        .expect("Empty list of destination blocks")
        .0;

    if latest_confirmed_block < latest_destination_block {
        return Err(Error::TeleportOnUnconfirmed);
    }

    Ok(())
}

fn get_destinations(
    blocks_by_chain: BlocksByChain,
    start_exec_location: ExecutionLocation,
) -> impl Iterator<Item = (ChainId, Vec<(BlockNumber, BlockHash)>)> {
    blocks_by_chain
        .into_iter()
        .filter(move |(chain_id, _)| *chain_id != start_exec_location.chain_id)
}

lazy_static! {
    // https://etherscan.deth.net/address/0x18DAc71c228D1C32c99489B7323d441E1175e443#readProxyContract
    // mapping: GameType -> OutputRoot
    // struct OutputRoot {
    //     hash: bytes32,
    //     blockNumber: uint256,
    // }
    static ref L2_OUTPUT_HASH_SLOT: U256 = U256::from_str_radix(
        // keccak(key . position).
        // Key = game type = 0
        // Position = 1
        "a6eef7e35abe7026729641147f7915573c7e97b47efa546f5f6e3230263bcb4a",
        16
    )
    .unwrap();
    // Second field of a struct
    static ref L2_BLOCK_NUMBER_SLOT: U256 = *L2_OUTPUT_HASH_SLOT + U256::from(1);
}

async fn get_l2_output<D>(
    source_chain_id: ChainId,
    source_db: D,
    dest_chain_id: ChainId,
    multi_op_rpc_client: &MultiOpRpcClient,
) -> Result<OutputResponse>
where
    D: DatabaseRef + Send + Sync,
    D::Error: Debug + std::error::Error + Send + Sync + 'static,
{
    let op_spec = ensure_teleport_possible(dest_chain_id, source_chain_id)?;
    let anchor_state_registry = op_spec.anchor_state_registry();

    let root = source_db
        .storage_ref(anchor_state_registry, *L2_OUTPUT_HASH_SLOT)
        .map_err(|err| Error::Database(anyhow!(err)))?;
    let l2_block_number = source_db
        .storage_ref(anchor_state_registry, *L2_BLOCK_NUMBER_SLOT)
        .map_err(|err| Error::Database(anyhow!(err)))?;

    let l2_output = multi_op_rpc_client
        .get(&dest_chain_id)
        .unwrap()
        .get_output_at_block(l2_block_number)
        .await;

    if l2_output.hash_slow() != B256::from(root) {
        return Err(Error::L2OutputHashMismatch);
    }
    Ok(l2_output)
}

fn ensure_teleport_possible(
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
mod validate_anchored_against {
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
