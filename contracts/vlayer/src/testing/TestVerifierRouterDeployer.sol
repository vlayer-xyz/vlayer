// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {ProofVerifierRouter, FakeProofVerifier, Groth16ProofVerifier} from "../proof_verifier/ProofVerifierRouter.sol";
import {Repository} from "../Repository.sol";
import {ImageID} from "../ImageID.sol";
import {Verifier} from "../Verifier.sol";

contract TestVerifierRouterDeployer {
    bool public constant IS_SCRIPT = true;

    function swapProofVerifier(Verifier verifier) external {
        Repository currentRepo = Repository(address(verifier.verifier().imageIdRepository()));

        Repository repository = new Repository(
            currentRepo.getRoleMember(currentRepo.DEFAULT_ADMIN_ROLE(), 0),
            currentRepo.getRoleMember(currentRepo.OWNER_ROLE(), 0)
        );
        repository.addImageIdSupport(ImageID.RISC0_CALL_GUEST_ID);

        verifier._setTestVerifier(
            new ProofVerifierRouter(new FakeProofVerifier(repository), new Groth16ProofVerifier(repository))
        );
    }
}
