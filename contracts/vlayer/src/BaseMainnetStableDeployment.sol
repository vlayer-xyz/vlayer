// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Repository} from "./Repository.sol";
import {FakeProofVerifier} from "./proof_verifier/FakeProofVerifier.sol";
import {Groth16ProofVerifier} from "./proof_verifier/Groth16ProofVerifier.sol";
import {ProofVerifierRouter} from "./proof_verifier/ProofVerifierRouter.sol";

library BaseMainnetStableDeployment {
    function repository() internal pure returns (Repository) {
        return Repository(address(0xc9708B07ae9906b92FF19281Fd660FB19206a8fA));
    }

    function verifiers() internal pure returns (FakeProofVerifier, Groth16ProofVerifier, ProofVerifierRouter) {
        FakeProofVerifier fakeProofVerifier = FakeProofVerifier(address(0x44983a6Cf1f7F4DCA168005740433aF66B666A1b));
        Groth16ProofVerifier groth16ProofVerifier =
            Groth16ProofVerifier(address(0x39599aC412c14F9635f5b5Bf8f4D4C1aeeCF6307));
        ProofVerifierRouter proofVerifierRouter =
            ProofVerifierRouter(address(0x8793Fcd526cac14A2E64f08adD3f9B5c833B4463));

        return (fakeProofVerifier, groth16ProofVerifier, proofVerifierRouter);
    }
}
