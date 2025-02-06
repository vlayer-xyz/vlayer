// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

error InvalidChainId(uint256 chainId);

library ChainIdLibrary {
    function isDevnet() internal view returns (bool) {
        return block.chainid == 3_1337 // Anvil local network
            || block.chainid == 30_1337; // vlayer test
    }

    function isTestnet() internal view returns (bool) {
        return block.chainid == 11155111 // Ethereum Sepolia
            || block.chainid == 300 // zkSync Sepolia
            || block.chainid == 4801 // Worldchain Sepolia
            || block.chainid == 59141 // Linea Sepolia
            || block.chainid == 80002 // Polygon Amoy
            || block.chainid == 84532 // Base Sepolia
            || block.chainid == 421614 // Arbitrum Sepolia
            || block.chainid == 11155420; // Optimism Sepolia
    }

    function isMainnet() internal view returns (bool) {
        return block.chainid == 1 // Ethereum
            || block.chainid == 10 // Optimism
            || block.chainid == 137 // Polygon
            || block.chainid == 324 // zkSync
            || block.chainid == 480 // Worldchain
            || block.chainid == 8453 // Base
            || block.chainid == 42161 // Arbitrum One
            || block.chainid == 42170 // Arbitrum Nova
            || block.chainid == 59144; // Linea
    }
}
