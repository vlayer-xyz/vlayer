/// The code in this module is a skeleton and is not up to our quality standards.
use std::{collections::HashMap, fmt::Debug};

use alloy_primitives::{BlockNumber, ChainId, B256, U256};
use async_trait::async_trait;
use chain::ChainSpec;
use common::Hashable;
use derive_more::Deref;
use derive_new::new;
use itertools::Itertools;
use lazy_static::lazy_static;
use revm::DatabaseRef;

use crate::evm::env::{cached::CachedEvmEnv, location::ExecutionLocation};

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
}

pub type Result = std::result::Result<(), Error>;
mod seal {
    pub trait Sealed<D> {}
}

#[cfg(any(test, feature = "testing"))]
impl<F, D> seal::Sealed<D> for F
where
    D: DatabaseRef,
    F: Fn(&CachedEvmEnv<D>, ExecutionLocation) -> Result + Send + Sync,
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
    ) -> Result;
}

#[cfg(any(test, feature = "testing"))]
#[async_trait]
impl<F, D> IVerifier<D> for F
where
    D: DatabaseRef + Send + Sync,
    F: Fn(&CachedEvmEnv<D>, ExecutionLocation) -> Result + Send + Sync,
{
    async fn verify(
        &self,
        evm_envs: &CachedEvmEnv<D>,
        start_exec_location: ExecutionLocation,
    ) -> Result {
        self(evm_envs, start_exec_location)
    }
}

lazy_static! {
    static ref ANCHOR_SLOT: U256 = U256::from(0);
}

struct BlockRef {
    number: u64,
    hash: B256,
}

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

const ABI: () = ();

fn abi_decode(_abi: (), _value: U256) -> (B256, U256) {
    todo!()
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
    D::Error: std::error::Error,
{
    async fn verify(
        &self,
        evm_envs: &CachedEvmEnv<D>,
        start_exec_location: ExecutionLocation,
    ) -> Result {
        let source_evm_env = evm_envs.get(start_exec_location)?;
        let blocks_by_chain = evm_envs.blocks_by_chain();
        let chain_ids = blocks_by_chain.chain_ids();
        let destinations = get_destinations(chain_ids, start_exec_location);
        for destination_chain_id in destinations {
            let l2_output = get_l2_output(
                start_exec_location.chain_id,
                &source_evm_env.db,
                destination_chain_id,
                &self.multi_op_rpc_client,
            )
            .await?;

            let latest_confirmed_location =
                (destination_chain_id, l2_output.block_ref.number).into();
            let latest_confirmed_evm_env = evm_envs.get(latest_confirmed_location)?;
            if latest_confirmed_evm_env.header.hash_slow() == l2_output.block_ref.hash {
                return Err(Error::HeaderHashMismatch);
            }

            let destination_blocks = blocks_by_chain.get(&destination_chain_id).unwrap();
            ensure_latest_teleport_location_is_confirmed(
                destination_blocks,
                l2_output.block_ref.number,
            )?;
        }
        Ok(())
    }
}

fn ensure_latest_teleport_location_is_confirmed(
    destination_blocks: &[(u64, B256)],
    latest_confirmed_block: BlockNumber,
) -> std::result::Result<(), Error> {
    let latest_destination_block = destination_blocks
        .iter()
        .sorted()
        .last()
        .expect("Empty list of destination blocks")
        .0;

    if latest_confirmed_block < latest_destination_block {
        return Err(Error::TeleportOnUnconfirmed);
    }

    Ok(())
}

fn get_destinations(
    chain_ids: impl IntoIterator<Item = ChainId>,
    start_exec_location: ExecutionLocation,
) -> Vec<ChainId> {
    let destinations: Vec<ChainId> = chain_ids
        .into_iter()
        .filter(|&chain_id| chain_id != start_exec_location.chain_id)
        .collect();
    destinations
}

async fn get_l2_output<D>(
    source_chain_id: ChainId,
    source_db: D,
    dest_chain_id: ChainId,
    multi_op_rpc_client: &MultiOpRpcClient,
) -> std::result::Result<Output, Error>
where
    D: DatabaseRef + Send + Sync,
    D::Error: Debug,
{
    let destination_chain_spec: ChainSpec = dest_chain_id.try_into().unwrap();
    let anchor_state_registry_address = destination_chain_spec
        .validate_anchored_against(source_chain_id)
        .unwrap();
    let value = source_db
        .storage_ref(anchor_state_registry_address, *ANCHOR_SLOT)
        .unwrap();
    let (root, l2_block_number) = abi_decode(ABI, value);
    let l2_output = multi_op_rpc_client
        .get(&dest_chain_id)
        .unwrap()
        .get_output_at_block(l2_block_number)
        .await;
    if l2_output.hash_slow() == root {
        return Err(Error::L2OutputHashMismatch);
    }
    Ok(l2_output)
}
