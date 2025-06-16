// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Repository} from "./Repository.sol";
import {FakeProofVerifier} from "./proof_verifier/FakeProofVerifier.sol";
import {Groth16ProofVerifier} from "./proof_verifier/Groth16ProofVerifier.sol";
import {ProofVerifierRouter} from "./proof_verifier/ProofVerifierRouter.sol";

library TestnetStableDeployment {
    function repository() internal pure returns (Repository) {
        return Repository(address(0xAD04462241343F0775315B2873E6fe6DffCce831));
    }

    function verifiers() internal pure returns (FakeProofVerifier, Groth16ProofVerifier, ProofVerifierRouter) {
        FakeProofVerifier fakeProofVerifier = FakeProofVerifier(address(0xeF2f0Cbb90821E1C5C46CE5283c82F802F65a3f3));
        Groth16ProofVerifier groth16ProofVerifier =
            Groth16ProofVerifier(address(0x074Fc67dA733FFA5B288a9d5755552e61a1A0a06));
        ProofVerifierRouter proofVerifierRouter =
            ProofVerifierRouter(address(0x7d441696a6F095B3Cd5e144ccBCDB507e0ce124e));

        return (fakeProofVerifier, groth16ProofVerifier, proofVerifierRouter);
    }
}
