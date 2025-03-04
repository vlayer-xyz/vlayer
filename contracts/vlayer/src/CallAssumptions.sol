// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.21;

/// @notice A CallAssumptions struct representing a block number and its block hash.
struct CallAssumptions {
    address proverContractAddress;
    bytes4 functionSelector;
    uint256 settleChainId; // Chain id for which assumptions was made.
    uint256 settleBlockNumber; // Block number for which assumptions was made.
    bytes32 settleBlockHash; // Hash of the block at the specified block number.
}

library CallAssumptionsLib {
    uint256 public constant ETH_WORD_SIZE = 32;

    uint256 public constant PROVER_CONTRACT_ADDRESS_ENCODING_LENGTH = ETH_WORD_SIZE;
    uint256 public constant FUNCTION_SELECTOR_ENCODING_LENGTH = ETH_WORD_SIZE;
    uint256 public constant SETTLE_CHAIN_ID_ENCODING_LENGTH = ETH_WORD_SIZE;
    uint256 public constant SETTLE_BLOCK_NUMBER_ENCODING_LENGTH = ETH_WORD_SIZE;
    uint256 public constant SETTLE_BLOCK_HASH_ENCODING_LENGTH = ETH_WORD_SIZE;

    uint256 public constant CALL_ASSUMPTIONS_ENCODING_LENGTH = PROVER_CONTRACT_ADDRESS_ENCODING_LENGTH
        + FUNCTION_SELECTOR_ENCODING_LENGTH + SETTLE_CHAIN_ID_ENCODING_LENGTH + SETTLE_BLOCK_NUMBER_ENCODING_LENGTH
        + SETTLE_BLOCK_HASH_ENCODING_LENGTH;
}
