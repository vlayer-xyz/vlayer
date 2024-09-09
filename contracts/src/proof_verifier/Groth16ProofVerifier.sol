// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {ControlID, RiscZeroGroth16Verifier} from "risc0-ethereum-1.0.0/src/groth16/RiscZeroGroth16Verifier.sol";

import {ProofVerifierBase} from "./ProofVerifierBase.sol";
import {ProofMode} from "../Seal.sol";

contract Groth16ProofVerifier is ProofVerifierBase {
    constructor() {
        PROOF_MODE = ProofMode.GROTH16;
        VERIFIER = new RiscZeroGroth16Verifier(ControlID.CONTROL_ROOT, ControlID.BN254_CONTROL_ID);
    }
}
