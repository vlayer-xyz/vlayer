mod private {
    use crate::ExecutionCommitment;
    use crate::Seal;

    alloy_sol_types::sol!("../../../contracts/src/Proof.sol");
}

use alloy_primitives::U256;
pub use private::Proof;

use serde_json::{json, Value};
impl Proof {
    pub fn to_json(&self) -> Value {
        json!({
            "length": Self::u256_to_number(self.length),
            "seal": {
                "seal": self.seal.seal,
                "mode": Into::<u8>::into(self.seal.mode),
            },
            "commitment": {
                "functionSelector": self.commitment.functionSelector,
                "proverContractAddress": self.commitment.proverContractAddress,
                "settleBlockNumber": Self::u256_to_number(self.commitment.settleBlockNumber),
                "settleBlockHash": self.commitment.settleBlockHash,
            }
        })
    }

    fn u256_to_number(value: U256) -> u64 {
        u64::try_from(value).unwrap()
    }
}
