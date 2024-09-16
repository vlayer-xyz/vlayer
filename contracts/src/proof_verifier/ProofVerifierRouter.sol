// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Proof} from "../Proof.sol";
import {ProofMode, Seal, SealLib} from "../Seal.sol";
import {IProofVerifier} from "./IProofVerifier.sol";
import {FakeProofVerifier} from "./FakeProofVerifier.sol";
import {Groth16ProofVerifier} from "./Groth16ProofVerifier.sol";

contract ProofVerifierRouter is IProofVerifier {
    using SealLib for Seal;

    FakeProofVerifier public fakeProofVerifier = new FakeProofVerifier();
    Groth16ProofVerifier public groth16ProofVerifier = new Groth16ProofVerifier();

    function call_guest_id() external view returns (bytes32) {
        return groth16ProofVerifier.call_guest_id();
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
