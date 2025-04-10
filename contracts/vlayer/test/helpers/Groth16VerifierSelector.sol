// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

library Groth16VerifierSelector {
    // value ensures that versions of risc0-ethereum and risc0-zkvm deps are compatible
    // must be kept in-sync with GROTH16_VERIFIER_SELECTOR value in rust/services/call/seal/src/lib.rs
    bytes4 public constant STABLE_VERIFIER_SELECTOR = bytes4(0x9f39696c);
}
