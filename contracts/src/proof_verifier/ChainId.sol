// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

error InvalidChainId();

library ChainIdLibrary {
    function is_devnet() internal view returns (bool) {
        return !is_mainnet();
    }

    function is_mainnet() internal view returns (bool) {
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
