// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {ChainIdLibrary, InvalidChainId} from "./ChainId.sol";
import {FakeProofVerifier} from "./FakeProofVerifier.sol";
import {ProofVerifierRouter} from "./ProofVerifierRouter.sol";
import {IProofVerifier} from "./IProofVerifier.sol";
import {ImageID} from "../ImageID.sol";
import {Repository} from "../Repository.sol";
import {TestnetStableDeployment} from "../TestnetStableDeployment.sol";
import {MainnetStableDeployment} from "../MainnetStableDeployment.sol";

library ProofVerifierFactory {
    function produce() internal returns (IProofVerifier) {
        if (ChainIdLibrary.isTestEnv()) {
            Repository repository = new Repository(address(this), address(this));
            repository.addImageIdSupport(ImageID.RISC0_CALL_GUEST_ID);
            return new FakeProofVerifier(repository);
        } else if (ChainIdLibrary.isMainnet()) {
            IProofVerifier groth16ProofVerifier = MainnetStableDeployment.verifiers();
            return groth16ProofVerifier;
        } else if (ChainIdLibrary.isTestnet()) {
            (,, ProofVerifierRouter proofVerifierRouter) = TestnetStableDeployment.verifiers();
            return proofVerifierRouter;
        }

        revert InvalidChainId();
    }
}
