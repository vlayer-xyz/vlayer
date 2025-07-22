// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Repository} from "./Repository.sol";
import {FakeProofVerifier} from "./proof_verifier/FakeProofVerifier.sol";
import {Groth16ProofVerifier} from "./proof_verifier/Groth16ProofVerifier.sol";
import {ProofVerifierRouter} from "./proof_verifier/ProofVerifierRouter.sol";

library TestnetStableDeployment {
    function repository() internal pure returns (Repository) {
        return Repository(address(0xE44007361170f82a5DAB427905Dd355E2CbeE7dB));
    }

    function verifiers() internal pure returns (FakeProofVerifier, Groth16ProofVerifier, ProofVerifierRouter) {
        FakeProofVerifier fakeProofVerifier = FakeProofVerifier(address(0x3bE01bee0cA51f5a84Ba10e675dD7576E20429CA));
        Groth16ProofVerifier groth16ProofVerifier =
            Groth16ProofVerifier(address(0xBabf9630bA994902E57f71Db032771f53B16C35b));
        ProofVerifierRouter proofVerifierRouter =
            ProofVerifierRouter(address(0xaB0B39778577A2536f12E53db7932859e1743605));

        return (fakeProofVerifier, groth16ProofVerifier, proofVerifierRouter);
    }
}
