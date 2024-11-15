mod private {
    alloy_sol_types::sol!(
        #[sol(all_derives = true)]
        "../../../../contracts/vlayer/src/Seal.sol"
    );
}

pub use private::{ProofMode, Seal};
