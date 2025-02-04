// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {ProofVerifierRouter, FakeProofVerifier, Groth16ProofVerifier} from "../proof_verifier/ProofVerifierRouter.sol";
import {Repository} from "../Repository.sol";
import {ImageID} from "../ImageID.sol";
import {Verifier} from "../Verifier.sol";

contract TestVerifierRouterDeployer {
    ProofVerifierRouter immutable verifierRouter;

    constructor() {
        Repository repository = new Repository(msg.sender, address(this));
        repository.addImageIdSupport(ImageID.RISC0_CALL_GUEST_ID);
        repository.transferOwnership(msg.sender);

        verifierRouter =
            new ProofVerifierRouter(new FakeProofVerifier(repository), new Groth16ProofVerifier(repository));
    }

    function swapProofVerifier(Verifier verifier) external {
        verifier._setTestVerifier(verifierRouter);
    }
}
