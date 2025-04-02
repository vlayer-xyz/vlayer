// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {ControlID, RiscZeroGroth16Verifier} from "risc0-ethereum-2.0.0/src/groth16/RiscZeroGroth16Verifier.sol";

import {ProofMode} from "../Seal.sol";

import {ProofVerifierBase} from "./ProofVerifierBase.sol";
import {IImageIdRepository} from "../Repository.sol";

contract Groth16ProofVerifier is ProofVerifierBase {
    constructor(IImageIdRepository _repository) ProofVerifierBase(_repository) {
        PROOF_MODE = ProofMode.GROTH16;
        VERIFIER = new RiscZeroGroth16Verifier(ControlID.CONTROL_ROOT, ControlID.BN254_CONTROL_ID);
    }
}
