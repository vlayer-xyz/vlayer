mod private {
    alloy_sol_types::sol!(
        #[sol(all_derives = true)]
        "../../../../contracts/src/Seal.sol"
    );
}

pub use private::ProofMode;
pub use private::Seal;
