/// The code in this module is a skeleton and is not up to our quality standards.
use std::{collections::HashMap, fmt::Debug};

use alloy_primitives::{BlockHash, BlockNumber, ChainId, B256, U256};
use anyhow::anyhow;
use async_trait::async_trait;
use chain::ChainSpec;
use common::Hashable;
use derive_more::Deref;
use derive_new::new;
use lazy_static::lazy_static;
use revm::DatabaseRef;

use crate::evm::env::{cached::CachedEvmEnv, location::ExecutionLocation, BlocksByChain};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unsupported chain id: {0}")]
    UnsupportedChainId(ChainId),
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

#[derive(Debug)]
struct BlockRef {
    number: u64,
    hash: B256,
}

#[derive(Debug)]
struct Output {
    block_ref: BlockRef,
}

impl Hashable for Output {
    fn hash_slow(&self) -> B256 {
        todo!()
    }
}

#[async_trait]
trait OpRpcClient: Send + Sync {
    async fn get_output_at_block(&self, block_number: U256) -> Output;
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

            let latest_confirmed_location = (chain_id, l2_output.block_ref.number).into();
            let latest_confirmed_evm_env = evm_envs.get(latest_confirmed_location)?;
            if latest_confirmed_evm_env.header.hash_slow() != l2_output.block_ref.hash {
                return Err(Error::HeaderHashMismatch);
            }
            ensure_latest_teleport_location_is_confirmed(&blocks, l2_output.block_ref.number)?;
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
) -> Result<Output>
where
    D: DatabaseRef + Send + Sync,
    D::Error: Debug + std::error::Error + Send + Sync + 'static,
{
    let destination_chain_spec: ChainSpec = dest_chain_id.try_into().unwrap();
    let anchor_state_registry_address = destination_chain_spec
        .validate_anchored_against(source_chain_id)
        .unwrap();

    let root = source_db
        .storage_ref(anchor_state_registry_address, *L2_OUTPUT_HASH_SLOT)
        .map_err(|err| Error::Database(anyhow!(err)))?;
    let l2_block_number = source_db
        .storage_ref(anchor_state_registry_address, *L2_BLOCK_NUMBER_SLOT)
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
