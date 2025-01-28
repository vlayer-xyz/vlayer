/// The code in this module is a skeleton and is not up to our quality standards.
use std::{collections::HashMap, fmt::Debug};

use alloy_primitives::{BlockHash, BlockNumber, ChainId, B256};
use anchor_state_registry::AnchorStateRegistry;
use async_trait::async_trait;
use chain::ensure_teleport_possible;
use common::Hashable;
use derive_more::Deref;
use derive_new::new;
use output::{OpRpcClient, OutputResponse};
use revm::DatabaseRef;

use crate::evm::env::{cached::CachedEvmEnv, location::ExecutionLocation, BlocksByChain};

mod anchor_state_registry;
mod chain;
mod output;

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
    let anchor_state_registry = AnchorStateRegistry::new(op_spec.anchor_state_registry());
    let l2_commitment = anchor_state_registry.get_latest_confirmed_l2_commitment(&source_db)?;

    let l2_output = multi_op_rpc_client
        .get(&dest_chain_id)
        .unwrap()
        .get_output_at_block(l2_commitment.block_number)
        .await;

    if l2_output.hash_slow() != l2_commitment.output_hash {
        return Err(Error::L2OutputHashMismatch);
    }
    Ok(l2_output)
}
