// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Proof} from "../Proof.sol";
import {ProofMode, Seal, SealLib} from "../Seal.sol";
import {IImageIdRepository} from "../Repository.sol";
import {IProofVerifier} from "./IProofVerifier.sol";
import {FakeProofVerifier} from "./FakeProofVerifier.sol";
import {Groth16ProofVerifier} from "./Groth16ProofVerifier.sol";

contract ProofVerifierRouter is IProofVerifier {
    using SealLib for Seal;

    FakeProofVerifier public immutable FAKE_PROOF_VERIFIER;
    Groth16ProofVerifier public immutable GROTH16_PROOF_VERIFIER;

    constructor(FakeProofVerifier _fakeProofVerifier, Groth16ProofVerifier _groth16ProofVerifier) {
        require(
            _groth16ProofVerifier.imageIdRepository() == _fakeProofVerifier.imageIdRepository(),
            "Verifiers should use same repository"
        );

        FAKE_PROOF_VERIFIER = _fakeProofVerifier;
        GROTH16_PROOF_VERIFIER = _groth16ProofVerifier;
    }

    function imageIdRepository() external view returns (IImageIdRepository) {
        return GROTH16_PROOF_VERIFIER.imageIdRepository();
    }

    function verify(Proof calldata proof, bytes32 journalHash, address expectedProver, bytes4 expectedSelector)
        external
        view
    {
        if (proof.seal.proofMode() == ProofMode.FAKE) {
            FAKE_PROOF_VERIFIER.verify(proof, journalHash, expectedProver, expectedSelector);
        } else if (proof.seal.proofMode() == ProofMode.GROTH16) {
            GROTH16_PROOF_VERIFIER.verify(proof, journalHash, expectedProver, expectedSelector);
        }
    }
}
