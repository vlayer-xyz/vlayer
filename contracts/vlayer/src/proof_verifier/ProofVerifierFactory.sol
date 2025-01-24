// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {ChainIdLibrary, InvalidChainId} from "./ChainId.sol";
import {IProofVerifier} from "./IProofVerifier.sol";
import {ProofVerifierRouter} from "./ProofVerifierRouter.sol";

library ProofVerifierFactory {
    function produce() internal returns (IProofVerifier) {
        if (ChainIdLibrary.isMainnet()) {
            return IProofVerifier(address(0));
        } else if (ChainIdLibrary.isDevnet() || ChainIdLibrary.isTestnet()) {
            return new ProofVerifierRouter();
        }

        revert InvalidChainId();
    }
}
