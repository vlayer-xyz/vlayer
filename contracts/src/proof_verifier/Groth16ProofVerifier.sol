// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {ProofVerifierBase} from "./ProofVerifierBase.sol";
import {ProofMode} from "../Seal.sol";

contract Groth16ProofVerifier is ProofVerifierBase {
    constructor() {
        proofMode = ProofMode.GROTH16;
    }
}
