use thiserror::Error;

pub(crate) trait BlockDatabase {
    #[allow(unused)]
    fn get_proof(&self, block_numbers: impl AsRef<[u8]>) -> Result<&Vec<u8>, BlockDbError>;
}

#[derive(Error, Debug, PartialEq)]
pub enum BlockDbError {
    #[error("Block {0} is outside of the current range {1:?}")]
    OutsideOfRange(u32, (u32, u32)),
    #[error("Proof not found for block {0}")]
    ProofNotFound(u32),
    #[error("Empty block numbers")]
    EmptyBlockNumbers,
}
