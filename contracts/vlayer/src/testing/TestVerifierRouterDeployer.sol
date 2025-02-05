// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {ProofVerifierRouter, FakeProofVerifier, Groth16ProofVerifier} from "../proof_verifier/ProofVerifierRouter.sol";
import {Repository} from "../Repository.sol";

contract TestVerifierRouterDeployer {
    ProofVerifierRouter public immutable VERIFIER_ROUTER;

    constructor(bytes32[] memory imageIds) {
        Repository repository = new Repository(address(this), address(this));
        for (uint256 i = 0; i < imageIds.length; i++) {
            repository.addImageIdSupport(imageIds[i]);
        }
        repository.transferOwnership(msg.sender);
        repository.transferAdminRole(msg.sender);

        VERIFIER_ROUTER =
            new ProofVerifierRouter(new FakeProofVerifier(repository), new Groth16ProofVerifier(repository));
    }
}
