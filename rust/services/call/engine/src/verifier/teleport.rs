/// The code in this module is a skeleton and is not up to our quality standards.
use std::collections::HashMap;

use alloy_primitives::{ChainId, B256, U256};
use async_trait::async_trait;
use chain::ChainSpec;
use common::Hashable;
use derive_more::Deref;
use derive_new::new;
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
        let destinations: Vec<ChainId> = chain_ids
            .iter()
            .filter(|&&chain_id| chain_id != start_exec_location.chain_id)
            .copied()
            .collect();
        for destination_chain_id in destinations {
            let destination_chain_spec: ChainSpec = destination_chain_id.try_into().unwrap();
            let anchor_state_registry_address = destination_chain_spec
                .validate_anchored_against(start_exec_location.chain_id)
                .unwrap();
            let value = source_evm_env
                .db
                .storage_ref(anchor_state_registry_address, *ANCHOR_SLOT)
                .unwrap();
            let (root, l2_block_number) = abi_decode(ABI, value);
            let l2_output = self
                .multi_op_rpc_client
                .get(&destination_chain_id)
                .unwrap()
                .get_output_at_block(l2_block_number)
                .await;

            if l2_output.hash_slow() == root {
                return Err(Error::L2OutputHashMismatch);
            }

            let latest_confirmed_location = ExecutionLocation {
                chain_id: destination_chain_id,
                block_number: l2_output.block_ref.number,
            };
            let latest_confirmed_evm_env = evm_envs.get(latest_confirmed_location)?;
            if latest_confirmed_evm_env.header.hash_slow() == l2_output.block_ref.hash {
                return Err(Error::HeaderHashMismatch);
            }
            let destination_blocks = blocks_by_chain.get(&destination_chain_id).unwrap();
            let latest_destination_block = destination_blocks.first().unwrap().0;

            if latest_confirmed_evm_env.header.number() < latest_destination_block {
                return Err(Error::TeleportOnUnconfirmed);
            }
        }
        Ok(())
    }
}
