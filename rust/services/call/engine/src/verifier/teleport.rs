use std::collections::HashMap;

use alloy_primitives::{BlockHash, BlockNumber, ChainId};
use async_trait::async_trait;
use derive_new::new;

use super::sealing::sealed_with_test_mock;
use crate::evm::env::location::ExecutionLocation;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unsupported chain id: {0}")]
    UnsupportedChainId(ChainId),
}

pub type Result = std::result::Result<(), Error>;
sealed_with_test_mock!(async IVerifier (blocks_by_chain: HashMap<ChainId, Vec<(BlockNumber, BlockHash)>>, start_exec_location: ExecutionLocation) -> Result);

#[derive(new)]
pub struct Verifier {}

impl seal::Sealed for Verifier {}
#[async_trait]
impl IVerifier for Verifier {
    async fn verify(
        &self,
        blocks_by_chain: HashMap<ChainId, Vec<(BlockNumber, BlockHash)>>,
        _start_exec_location: ExecutionLocation,
    ) -> Result {
        let chains = blocks_by_chain.keys().cloned().collect::<Vec<_>>();
        #[allow(clippy::match_same_arms)]
        match chains.as_slice() {
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
