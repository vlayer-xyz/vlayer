#![allow(clippy::expect_used, clippy::panic)]

pub mod config;
pub mod consts;
mod db;
pub mod evm;
mod io;
pub mod sol;
pub mod travel_call;
pub mod utils;
pub use db::seed_cache_db_with_trusted_data;
pub use io::{Call, CallGuestId, GuestOutput, GuestOutputError, HostOutput, Input};
pub use sol::{
    call_assumptions::CallAssumptions,
    proof::Proof,
    seal::{ProofMode, Seal},
};
pub mod verifier;
