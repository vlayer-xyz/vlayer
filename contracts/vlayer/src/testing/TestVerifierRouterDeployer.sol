// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {ProofVerifierRouter, FakeProofVerifier, Groth16ProofVerifier} from "../proof_verifier/ProofVerifierRouter.sol";
import {Repository} from "../Repository.sol";
import {ImageID} from "../ImageID.sol";

contract TestVerifierRouterDeployer {
    ProofVerifierRouter public immutable VERIFIER_ROUTER;

    constructor() {
        Repository repository = new Repository(address(this), address(this));
        repository.addImageIdSupport(ImageID.RISC0_CALL_GUEST_ID);
        repository.transferOwnership(msg.sender);
        repository.transferAdminRole(msg.sender);

        VERIFIER_ROUTER =
            new ProofVerifierRouter(new FakeProofVerifier(repository), new Groth16ProofVerifier(repository));
    }
}
