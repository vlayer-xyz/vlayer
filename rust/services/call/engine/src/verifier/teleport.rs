/// The code in this module is a skeleton and is not up to our quality standards.
use std::{fmt::Debug, sync::Arc};

use alloy_primitives::{B256, BlockHash, BlockNumber, ChainId};
use async_trait::async_trait;
use call_common::{ExecutionLocation, RevmDB};
use common::Hashable;
use derivative::Derivative;
use optimism::{
    NumHash,
    anchor_state_registry::{AnchorStateRegistry, L2Commitment},
    types::SequencerOutput,
};
use tracing::info;

use crate::evm::env::{BlocksByChain, cached::CachedEvmEnv};

#[derive(thiserror::Error, Debug, Derivative)]
#[derivative(PartialEq, Eq)]
pub enum Error {
    #[error("EvmEnvFactory: {0}")]
    Factory(#[from] crate::evm::env::factory::Error),
    #[error("Output hash mismatch")]
    L2OutputHashMismatch,
    #[error("Header hash mismatch")]
    HeaderHashMismatch,
    #[error(
        "Teleport to unconfirmed block: {target_block} on chain id: {chain_id}. Latest confirmed block: {latest_confirmed_block}. Transaction dispute window is 7 days."
    )]
    TeleportOnUnconfirmed {
        target_block: BlockNumber,
        chain_id: ChainId,
        latest_confirmed_block: BlockNumber,
    },
    #[error("Anchor state registry: {0}")]
    AnchorStateRegistry(
        #[from]
        #[derivative(PartialEq = "ignore")]
        optimism::anchor_state_registry::Error,
    ),
    #[error(transparent)]
    OptimismConversion(#[from] chain::optimism::ConversionError),
    #[error(transparent)]
    Conversion(#[from] chain::ConversionError),
    #[error("Commit error: {0}")]
    Commit(#[from] chain::optimism::CommitError),
    #[error("Client factory error: {0}")]
    OptimismClientFactory(#[from] optimism::client::FactoryError),
    #[error("Client error: {0}")]
    OptimismClient(#[from] optimism::ClientError),
}

pub type Result<T> = std::result::Result<T, Error>;
mod seal {
    pub trait Sealed<D> {}
}

#[cfg(any(test, feature = "testing"))]
impl<F, D> seal::Sealed<D> for F
where
    D: RevmDB,
    F: Fn(&CachedEvmEnv<D>, ExecutionLocation) -> Result<()> + Send + Sync,
{
}

#[async_trait]
pub trait IVerifier<D: RevmDB>: seal::Sealed<D> + Send + Sync {
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
    D: RevmDB,
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

pub struct Verifier {
    sequencer_client_factory: Arc<dyn optimism::client::IFactory>,
}

impl Verifier {
    pub fn new(factory: impl optimism::client::IFactory + 'static) -> Self {
        Self {
            sequencer_client_factory: Arc::new(factory),
        }
    }
}

impl<D> seal::Sealed<D> for Verifier {}
#[async_trait]
impl<D: RevmDB> IVerifier<D> for Verifier {
    async fn verify(
        &self,
        evm_envs: &CachedEvmEnv<D>,
        start_exec_location: ExecutionLocation,
    ) -> Result<()> {
        let ExecutionLocation {
            chain_id: source_chain_id,
            block_number,
        } = start_exec_location;
        info!(source_chain_id, block_number, "Verifying teleport");
        let source_chain_spec = chain::ChainSpec::try_from(source_chain_id)?;
        if source_chain_spec.is_local_testnet() {
            info!("Skipping teleport verification for local testnet");
            return Ok(());
        }
        let source_evm_env = evm_envs.get(start_exec_location)?;
        let blocks_by_chain = evm_envs.blocks_by_chain();
        let destinations = get_destinations(blocks_by_chain, start_exec_location);
        for (chain_id, blocks) in destinations {
            info!("Verifying teleport to chain: {chain_id} blocks: {blocks:?}");
            let dest_chain_spec = chain::optimism::ChainSpec::try_from(chain_id)?;
            dest_chain_spec.assert_anchor(source_chain_id)?;

            let anchor_state_registry =
                AnchorStateRegistry::new(dest_chain_spec.anchor_state_registry, &source_evm_env.db);
            let sequencer_client = self.sequencer_client_factory.create(chain_id)?;
            let l2_block =
                fetch_latest_confirmed_l2_block(anchor_state_registry, &sequencer_client).await?;
            info!("Latest confirmed L2 block: {l2_block:?}");

            let latest_confirmed_location = (chain_id, l2_block.number).into();
            let latest_confirmed_evm_env = evm_envs.get(latest_confirmed_location)?;

            if latest_confirmed_evm_env.header.hash_slow() != l2_block.hash {
                return Err(Error::HeaderHashMismatch);
            }
            ensure_latest_teleport_location_is_confirmed(&blocks, l2_block.number, chain_id)?;
            info!("Teleport to chain {chain_id} verified");
        }
        Ok(())
    }
}

pub async fn fetch_latest_confirmed_l2_block<D: RevmDB>(
    anchor_state_registry: AnchorStateRegistry<D>,
    sequencer_client: &dyn optimism::IClient,
) -> Result<NumHash> {
    info!("Fetching latest confirmed L2 block");
    let L2Commitment {
        output_hash,
        block_number,
    } = anchor_state_registry.get_latest_confirmed_l2_commitment()?;
    info!(?output_hash, block_number, "L2 commitment");

    let sequencer_output = sequencer_client.get_output_at_block(block_number).await?;
    let SequencerOutput {
        version,
        state_root,
        withdrawal_storage_root,
        l2_block: NumHash { number, hash },
    } = sequencer_output;
    info!(
        ?version,
        ?state_root,
        ?withdrawal_storage_root,
        number,
        ?hash,
        "Sequencer output"
    );

    if sequencer_output.hash_slow() != output_hash {
        return Err(Error::L2OutputHashMismatch);
    }
    Ok(sequencer_output.l2_block)
}

pub fn ensure_latest_teleport_location_is_confirmed(
    destination_blocks: &[(u64, B256)],
    latest_confirmed_block: BlockNumber,
    chain_id: ChainId,
) -> Result<()> {
    let latest_destination_block = destination_blocks
        .iter()
        .max()
        .expect("Empty list of destination blocks")
        .0;

    if latest_confirmed_block < latest_destination_block {
        return Err(Error::TeleportOnUnconfirmed {
            target_block: latest_destination_block,
            chain_id,
            latest_confirmed_block,
        });
    }
    info!(
        "Teleport onto block {latest_destination_block} allowed. Latest confirmed {latest_confirmed_block}"
    );

    Ok(())
}

pub fn get_destinations(
    blocks_by_chain: BlocksByChain,
    start_exec_location: ExecutionLocation,
) -> impl Iterator<Item = (ChainId, Vec<(BlockNumber, BlockHash)>)> {
    blocks_by_chain
        .into_iter()
        .filter(move |(chain_id, _)| *chain_id != start_exec_location.chain_id)
}
