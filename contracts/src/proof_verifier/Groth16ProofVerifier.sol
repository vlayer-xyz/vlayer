// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {ControlID, RiscZeroGroth16Verifier} from "risc0-ethereum/groth16/RiscZeroGroth16Verifier.sol";

import {ProofVerifierBase} from "./ProofVerifierBase.sol";
import {ProofMode} from "../Seal.sol";

contract Groth16ProofVerifier is ProofVerifierBase {
    constructor() {
        proofMode = ProofMode.GROTH16;
        verifier = new RiscZeroGroth16Verifier(ControlID.CONTROL_ROOT, ControlID.BN254_CONTROL_ID);
    }
}
