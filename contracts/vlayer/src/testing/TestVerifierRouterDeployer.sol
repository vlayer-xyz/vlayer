// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {ProofVerifierRouter, FakeProofVerifier, Groth16ProofVerifier} from "../proof_verifier/ProofVerifierRouter.sol";
import {Repository} from "../Repository.sol";
import {ImageID} from "../ImageID.sol";
import {Verifier} from "../Verifier.sol";

contract TestVerifierRouterDeployer {
    function swapProofVerifier(Verifier verifier) external {
        Repository repository = new Repository(msg.sender, msg.sender);
        repository.addImageIdSupport(ImageID.RISC0_CALL_GUEST_ID);

        verifier._setTestVerifier(
            new ProofVerifierRouter(new FakeProofVerifier(repository), new Groth16ProofVerifier(repository))
        );
    }
}
