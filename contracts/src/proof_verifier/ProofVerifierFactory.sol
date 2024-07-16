// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {IProofVerifier} from "./IProofVerifier.sol";
import {FakeProofVerifier} from "./FakeProofVerifier.sol";

error InvalidChainId();

library ProofVerifierFactory {
    function produce() internal returns (IProofVerifier) {
        if (is_mainnet()) {
            return IProofVerifier(address(0));
        } else if (is_devnet()) {
            return new FakeProofVerifier();
        }

        revert InvalidChainId();
    }

    function is_devnet() private view returns (bool) {
        return block.chainid == 31337;
    }

    function is_mainnet() private view returns (bool) {
        uint256[2] memory MAINNETS = [
            uint256(1), // Ethereum
            uint256(10) // Optimism
        ];

        for (uint256 i = 0; i < MAINNETS.length; i++) {
            if (MAINNETS[i] == block.chainid) {
                return true;
            }
        }

        return false;
    }
}
