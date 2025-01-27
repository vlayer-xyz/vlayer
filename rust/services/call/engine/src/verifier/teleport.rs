use alloy_primitives::ChainId;
use async_trait::async_trait;
use derive_new::new;

use super::sealing::sealed_with_test_mock;
use crate::evm::env::{location::ExecutionLocation, BlocksByChain};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unsupported chain id: {0}")]
    UnsupportedChainId(ChainId),
}

pub type Result = std::result::Result<(), Error>;
sealed_with_test_mock!(async IVerifier (blocks_by_chain: BlocksByChain, start_exec_location: ExecutionLocation) -> Result);

#[derive(new)]
pub struct Verifier {}

impl seal::Sealed for Verifier {}
#[async_trait]
impl IVerifier for Verifier {
    async fn verify(
        &self,
        blocks_by_chain: BlocksByChain,
        _start_exec_location: ExecutionLocation,
    ) -> Result {
        let chains = blocks_by_chain.chain_ids();
        #[allow(clippy::match_same_arms)]
        match chains.as_ref() {
            [] => unreachable!(
                "Empty list of execution locations. At least start one should always be there"
            ),
            [_] => {
                Ok(()) // No teleportation
            }
            _ => {
                Ok(()) // TODO: Implement teleportation
            }
        }
    }
}
