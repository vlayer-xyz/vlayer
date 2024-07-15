// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {RiscZeroMockVerifier} from "risc0-ethereum/test/RiscZeroMockVerifier.sol";

import {ProofVerifierBase} from "./ProofVerifierBase.sol";

contract FakeProofVerifier is ProofVerifierBase {
    constructor() {
        verifier = new RiscZeroMockVerifier(bytes4(0));
    }
}
