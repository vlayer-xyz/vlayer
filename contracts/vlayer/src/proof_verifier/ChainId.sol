// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

error InvalidChainId();

library ChainIdLibrary {
    function is_devnet() internal view returns (bool) {
        return block.chainid == 31337; // Anvil local network
    }

    function is_testnet() internal view returns (bool) {
        return block.chainid == 11155111 // Ethereum Sepolia
            || block.chainid == 11155420 // Optimism Sepolia
            || block.chainid == 84532 // Base Sepolia
            || block.chainid == 80002 // Polygon Amoy
            || block.chainid == 421614 // Arbitrum Sepolia
            || block.chainid == 300; // zkSync Sepolia
    }

    function is_mainnet() internal view returns (bool) {
        return block.chainid == 1 // Ethereum
            || block.chainid == 10 // Optimism
            || block.chainid == 8453 // Base
            || block.chainid == 42161 // Arbitrum One
            || block.chainid == 42170 // Arbitrum Nova
            || block.chainid == 137 // Polygon
            || block.chainid == 324; // zkSync
    }
}
