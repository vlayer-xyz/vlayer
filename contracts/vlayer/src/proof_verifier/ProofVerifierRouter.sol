// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Proof} from "../Proof.sol";
import {ProofMode, Seal, SealLib} from "../Seal.sol";
import {IProofVerifier} from "./IProofVerifier.sol";
import {FakeProofVerifier} from "./FakeProofVerifier.sol";
import {Groth16ProofVerifier} from "./Groth16ProofVerifier.sol";

contract ProofVerifierRouter is IProofVerifier {
    using SealLib for Seal;

    FakeProofVerifier public immutable fakeProofVerifier;
    Groth16ProofVerifier public immutable groth16ProofVerifier;

    constructor(FakeProofVerifier _fakeProofVerifier, Groth16ProofVerifier _groth16ProofVerifier) {
        fakeProofVerifier = _fakeProofVerifier;
        groth16ProofVerifier = _groth16ProofVerifier;
    }

    function verify(Proof calldata proof, bytes32 journalHash, address expectedProver, bytes4 expectedSelector)
        external
        view
    {
        if (proof.seal.proofMode() == ProofMode.FAKE) {
            fakeProofVerifier.verify(proof, journalHash, expectedProver, expectedSelector);
        } else if (proof.seal.proofMode() == ProofMode.GROTH16) {
            groth16ProofVerifier.verify(proof, journalHash, expectedProver, expectedSelector);
        }
    }
}
