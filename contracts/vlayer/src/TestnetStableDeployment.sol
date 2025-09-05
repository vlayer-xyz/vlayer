// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Repository} from "./Repository.sol";
import {FakeProofVerifier} from "./proof_verifier/FakeProofVerifier.sol";
import {Groth16ProofVerifier} from "./proof_verifier/Groth16ProofVerifier.sol";
import {ProofVerifierRouter} from "./proof_verifier/ProofVerifierRouter.sol";

library TestnetStableDeployment {
    function repository() internal pure returns (Repository) {
        return Repository(address(0x0cFfdB4e737F00Ef57b4c61dBfBb334B3a416519));
    }

    function verifiers() internal pure returns (FakeProofVerifier, Groth16ProofVerifier, ProofVerifierRouter) {
        FakeProofVerifier fakeProofVerifier = FakeProofVerifier(address(0x711B293738290768f3eD1DBf2D00e0f9eEc19E6B));
        Groth16ProofVerifier groth16ProofVerifier =
            Groth16ProofVerifier(address(0x9AdE0B5F34402AeFdcBE1a8733d5995Ff827f586));
        ProofVerifierRouter proofVerifierRouter =
            ProofVerifierRouter(address(0x7925a78734Fc7f2cb69d7E03d81467BB851f9Eb8));

        return (fakeProofVerifier, groth16ProofVerifier, proofVerifierRouter);
    }
}
