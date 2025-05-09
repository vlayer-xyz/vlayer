// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Repository} from "./Repository.sol";
import {FakeProofVerifier} from "./proof_verifier/FakeProofVerifier.sol";
import {Groth16ProofVerifier} from "./proof_verifier/Groth16ProofVerifier.sol";
import {ProofVerifierRouter} from "./proof_verifier/ProofVerifierRouter.sol";

library TestnetStableDeployment {
    function repository() internal pure returns (Repository) {
        return Repository(address(0xc4E4dC291A5C4dEbe9Ff5a3372F3FdD2e42Bac86));
    }

    function verifiers() internal pure returns (FakeProofVerifier, Groth16ProofVerifier, ProofVerifierRouter) {
        FakeProofVerifier fakeProofVerifier = FakeProofVerifier(address(0x9E30DC3e49C4d47F982902710616DCC4b6fF7bf5));
        Groth16ProofVerifier groth16ProofVerifier =
            Groth16ProofVerifier(address(0x7E231CfC3e3B549633D5AD61C30f07Dd4d408ad3));
        ProofVerifierRouter proofVerifierRouter =
            ProofVerifierRouter(address(0x8972b6fF413AC04e61e19f07d7F3BD980D0Cc321));

        return (fakeProofVerifier, groth16ProofVerifier, proofVerifierRouter);
    }
}
