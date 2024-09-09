mod private {
    use crate::ExecutionCommitment;
    use crate::Seal;

    alloy_sol_types::sol!("../../../../contracts/src/Proof.sol");
}

pub use private::Proof;
