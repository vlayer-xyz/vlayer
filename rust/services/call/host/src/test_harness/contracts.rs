use alloy_primitives::{address, Address};
use alloy_sol_types::sol;

pub mod web {
    use super::*;

    pub const PROVER: Address = address!("0x524f746a7e18ff210a5d64740ec57b48be11506b");
    pub const BLOCK_NO: u64 = 24_146_881;

    sol! {
        #[derive(Debug)]
        struct WebProof {
            string webProofJson;
        }
        #[derive(Debug)]
        struct Proof {
            Seal seal;
            bytes32 callGuestId;
            uint256 length;
            CallAssumptions callAssumptions;
        }
        #[derive(Debug)]
        interface WebProofProver {
            function main(WebProof calldata webProof, address account)
            public
            view
            returns (Proof memory, string memory, address);
        }
    }
}

sol! {
    #[derive(Debug)]
    enum ProofMode {
        GROTH16,
        FAKE
    }
    #[derive(Debug)]
    struct Seal {
        bytes4 verifierSelector;
        bytes32[8] seal;
        ProofMode mode;
    }
    #[derive(Debug)]
    struct CallAssumptions {
        address proverContractAddress;
        bytes4 functionSelector;
        uint256 settleBlockNumber; // Block number for which assumptions was made.
        bytes32 settleBlockHash; // Hash of the block at the specified block number.
    }
}
