// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {ChainIdLibrary, InvalidChainId} from "./ChainId.sol";
import {Repository} from "../Repository.sol";
import {ImageID} from "../ImageID.sol";
import {IProofVerifier} from "./IProofVerifier.sol";
import {FakeProofVerifier} from "./FakeProofVerifier.sol";
import {Groth16ProofVerifier} from "./Groth16ProofVerifier.sol";
import {ProofVerifierRouter} from "./ProofVerifierRouter.sol";

library ProofVerifierFactory {
    function produce() internal returns (IProofVerifier) {
        if (ChainIdLibrary.isMainnet()) {
            return IProofVerifier(address(0));
        } else if (ChainIdLibrary.isDevnet() || ChainIdLibrary.isTestnet()) {
            Repository repository = new Repository(address(this), address(this));
            repository.addImageIdSupport(ImageID.RISC0_CALL_GUEST_ID);
            return new ProofVerifierRouter(new FakeProofVerifier(repository), new Groth16ProofVerifier(repository));
        }

        revert InvalidChainId();
    }
}
