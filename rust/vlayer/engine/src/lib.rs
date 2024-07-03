#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

pub mod chain;
pub mod config;
pub mod engine;
pub mod ethereum;
pub mod evm;
pub mod io;
pub mod spike_inspector;

// Keep everything in the Steel library private except the commitment.
mod private {
    alloy_sol_types::sol!("./Steel.sol");
}

/// Solidity struct representing the committed block used for validation.
pub use private::Steel::ExecutionCommitment;
