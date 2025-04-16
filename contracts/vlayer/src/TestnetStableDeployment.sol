// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Repository} from "./Repository.sol";
import {FakeProofVerifier} from "./proof_verifier/FakeProofVerifier.sol";
import {Groth16ProofVerifier} from "./proof_verifier/Groth16ProofVerifier.sol";
import {ProofVerifierRouter} from "./proof_verifier/ProofVerifierRouter.sol";

library TestnetStableDeployment {
    function repository() internal pure returns (Repository) {
        return Repository(address(0xE9a0bF69c4A73A3Be20cb83159F9202ffFeA75dB));
    }

    function verifiers() internal pure returns (FakeProofVerifier, Groth16ProofVerifier, ProofVerifierRouter) {
        FakeProofVerifier fakeProofVerifier = FakeProofVerifier(address(0x344a9d8F3aE191ad5B4aC7DF9b9EcdA59360EF71));
        Groth16ProofVerifier groth16ProofVerifier =
            Groth16ProofVerifier(address(0xDBaE93E682F615E4AB099F64aEB3f60Dc0396803));
        ProofVerifierRouter proofVerifierRouter =
            ProofVerifierRouter(address(0x2e3DCe3ac3DE5aac6d78aa57a19Ac5f2Df9E2922));

        return (fakeProofVerifier, groth16ProofVerifier, proofVerifierRouter);
    }
}
