// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

error InvalidChainId();

library ChainIdLibrary {
    function is_devnet() internal view returns (bool) {
        return block.chainid == 31337 || block.chainid == 100001;
    }

    function is_mainnet() internal view returns (bool) {
        uint256[2] memory mainnets = [
            uint256(1), // Ethereum
            uint256(10) // Optimism
        ];

        for (uint256 i = 0; i < mainnets.length; i++) {
            if (mainnets[i] == block.chainid) {
                return true;
            }
        }

        return false;
    }
}
