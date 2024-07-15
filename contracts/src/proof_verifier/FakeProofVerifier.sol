// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {RiscZeroMockVerifier} from "risc0-ethereum/test/RiscZeroMockVerifier.sol";

import {Proof} from "../Proof.sol";
import {ProofMode, SealLib, Seal} from "../Seal.sol";

import {ProofVerifierBase} from "./ProofVerifierBase.sol";

contract FakeProofVerifier is ProofVerifierBase {
    using SealLib for Seal;

    constructor() {
        verifier = new RiscZeroMockVerifier(bytes4(0));
        proofMode = ProofMode.FAKE;
    }
}
