mod private {
    use crate::{CallAssumptions, Seal};

    alloy_sol_types::sol!("../../../../contracts/vlayer/src/Proof.sol");
}

pub use private::Proof;
