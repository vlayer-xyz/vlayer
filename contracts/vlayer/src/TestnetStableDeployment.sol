// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Repository} from "./Repository.sol";
import {FakeProofVerifier} from "./proof_verifier/FakeProofVerifier.sol";
import {Groth16ProofVerifier} from "./proof_verifier/Groth16ProofVerifier.sol";
import {ProofVerifierRouter} from "./proof_verifier/ProofVerifierRouter.sol";

library TestnetStableDeployment {
    function repository() internal pure returns (Repository) {
        return Repository(address(0xc9708B07ae9906b92FF19281Fd660FB19206a8fA));
    }

    function verifiers() internal pure returns (FakeProofVerifier, Groth16ProofVerifier, ProofVerifierRouter) {
        FakeProofVerifier fakeProofVerifier = FakeProofVerifier(address(0x913649a62807b2B8144a1E4d41a0E1aEe10a3985));
        Groth16ProofVerifier groth16ProofVerifier =
            Groth16ProofVerifier(address(0x41f78ef6625E119f328EC8a88D54f9da1c227E6A));
        ProofVerifierRouter proofVerifierRouter =
            ProofVerifierRouter(address(0x17C9793523c11eb0b6Da1DaeD53839146277AB4e));

        return (fakeProofVerifier, groth16ProofVerifier, proofVerifierRouter);
    }
}
