// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {ChainIdLibrary, InvalidChainId} from "./ChainId.sol";
import {FakeProofVerifier} from "./FakeProofVerifier.sol";
import {ProofVerifierRouter} from "./ProofVerifierRouter.sol";
import {IProofVerifier} from "./IProofVerifier.sol";
import {ImageID} from "../ImageID.sol";
import {Repository} from "../Repository.sol";
import {TestnetStableDeployment} from "../TestnetStableDeployment.sol";

library ProofVerifierFactory {
    function produce() internal returns (IProofVerifier) {
        if (ChainIdLibrary.isMainnet()) {
            return IProofVerifier(address(0));
        } else if (ChainIdLibrary.isTestnet()) {
            (,, ProofVerifierRouter proofVerifierRouter) = TestnetStableDeployment.verifiers();
            return proofVerifierRouter;
        } else if (ChainIdLibrary.isDevnet()) {
            Repository repository = new Repository(address(this), address(this));
            repository.addImageIdSupport(ImageID.RISC0_CALL_GUEST_ID);
            return new FakeProofVerifier(repository);
        }

        revert InvalidChainId();
    }
}
