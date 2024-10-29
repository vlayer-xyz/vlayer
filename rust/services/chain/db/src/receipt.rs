use bytes::Bytes;
use derive_more::{From, Into};
use risc0_zkvm::{AssumptionReceipt, Receipt};

#[derive(Debug, Clone, From, Into)]
pub struct ChainProof(Receipt);

impl From<ChainProof> for Bytes {
    fn from(receipt: ChainProof) -> Self {
        bincode::serialize(&receipt.0)
            .expect("failed to serialize receipt")
            .into()
    }
}

impl From<&Bytes> for ChainProof {
    fn from(bytes: &Bytes) -> Self {
        ChainProof(bincode::deserialize(bytes).expect("failed to deserialize receipt"))
    }
}

impl From<ChainProof> for AssumptionReceipt {
    fn from(receipt: ChainProof) -> Self {
        receipt.0.into()
    }
}
