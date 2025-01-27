// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

error InvalidChainId();

library ChainIdLibrary {
    function isDevnet() internal view returns (bool) {
        return block.chainid == 3_1337 // Anvil local network
            || block.chainid == 30_1337; // vlayer test
    }

    function isTestnet() internal view returns (bool) {
        return block.chainid == 11155111 // Ethereum Sepolia
            || block.chainid == 11155420 // Optimism Sepolia
            || block.chainid == 84532; // Base Sepolia
    }

    function isMainnet() internal view returns (bool) {
        return block.chainid == 1 // Ethereum
            || block.chainid == 10 // Optimism
            || block.chainid == 8453 // Base
            || block.chainid == 42161 // Arbitrum One
            || block.chainid == 42170 // Arbitrum Nova
            || block.chainid == 137 // Polygon
            || block.chainid == 324; // zkSync
    }
}
