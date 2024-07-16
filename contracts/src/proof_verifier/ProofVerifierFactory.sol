// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {ChainIdLibrary, InvalidChainId} from "./ChainId.sol";
import {IProofVerifier} from "./IProofVerifier.sol";
import {FakeProofVerifier} from "./FakeProofVerifier.sol";


library ProofVerifierFactory {
    function produce() internal returns (IProofVerifier) {
        if (ChainIdLibrary.is_mainnet()) {
            return IProofVerifier(address(0));
        } else if (ChainIdLibrary.is_devnet()) {
            return new FakeProofVerifier();
        }

        revert InvalidChainId();
    }

}
