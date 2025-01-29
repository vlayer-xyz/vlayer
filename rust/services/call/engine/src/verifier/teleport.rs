/// The code in this module is a skeleton and is not up to our quality standards.
use std::{collections::HashMap, fmt::Debug};

use ::chain::optimism;
use alloy_primitives::{BlockHash, BlockNumber, ChainId, B256};
use anchor_state_registry::AnchorStateRegistry;
use async_trait::async_trait;
use common::Hashable;
use derive_more::Deref;
use derive_new::new;
use output::OpRpcClient;
use revm::DatabaseRef;

use crate::evm::env::{cached::CachedEvmEnv, location::ExecutionLocation, BlocksByChain};

mod anchor_state_registry;
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
    #[error(transparent)]
    Conversion(#[from] optimism::ConversionError),
    #[error(transparent)]
    Commit(#[from] optimism::CommitError),
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
        let source_chain_id = start_exec_location.chain_id;
        let source_evm_env = evm_envs.get(start_exec_location)?;
        let blocks_by_chain = evm_envs.blocks_by_chain();
        let destinations = get_destinations(blocks_by_chain, start_exec_location);
        for (chain_id, blocks) in destinations {
            let dest_chain_spec = optimism::ChainSpec::try_from(chain_id)?;
            dest_chain_spec.assert_anchor(source_chain_id)?;

            let anchor_state_registry =
                AnchorStateRegistry::new(dest_chain_spec.anchor_state_registry);
            let l2_commitment =
                anchor_state_registry.get_latest_confirmed_l2_commitment(&source_evm_env.db)?;

            let client = self.multi_op_rpc_client.get(&chain_id).unwrap();
            let l2_output = client.get_output_at_block(l2_commitment.block_number).await;

            if l2_output.hash_slow() != l2_commitment.output_hash {
                return Err(Error::L2OutputHashMismatch);
            }
            let l1_block = l2_output.block_ref.l1_block_info;

            let latest_confirmed_location = (chain_id, l1_block.number).into();
            let latest_confirmed_evm_env = evm_envs.get(latest_confirmed_location)?;
            if latest_confirmed_evm_env.header.hash_slow() != l1_block.hash {
                return Err(Error::HeaderHashMismatch);
            }
            ensure_latest_teleport_location_is_confirmed(&blocks, l1_block.number)?;
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
