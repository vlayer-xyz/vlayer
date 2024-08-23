mod private {
    alloy_sol_types::sol!("../../../contracts/src/Seal.sol");
}

pub use private::ProofMode;
pub use private::Seal;
