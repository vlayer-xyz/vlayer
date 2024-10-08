// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

error InvalidChainId();

library ChainIdLibrary {
    function is_devnet() internal view returns (bool) {
        return block.chainid == 31337; // Anvil local network
    }

    function is_testnet() internal view returns (bool) {
        return block.chainid == 11155111 // Ethereum Sepolia testnet
            || block.chainid == 11155420; // Optimism Sepolia testnet
    }

    function is_mainnet() internal view returns (bool) {
        return block.chainid == 1 // Ethereum
            || block.chainid == 10; // Optimism
    }
}
