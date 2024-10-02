// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.13;

/// @notice A CallAssumptions struct representing a block number and its block hash.
struct CallAssumptions {
    address proverContractAddress;
    bytes4 functionSelector;
    uint256 settleBlockNumber; // Block number for which assumptions was made.
    bytes32 settleBlockHash; // Hash of the block at the specified block number.
}

library CallAssumptionsLib {
    uint256 constant ETH_WORD_SIZE = 32;

    uint256 public constant PROVER_CONTRACT_ADDRESS_ENCODING_LENGTH = ETH_WORD_SIZE;
    uint256 public constant FUNCTION_SELECTOR_ENCODING_LENGTH = ETH_WORD_SIZE;
    uint256 public constant SETTLE_BLOCK_NUMBER_ENCODING_LENGTH = ETH_WORD_SIZE;
    uint256 public constant SETTLE_BLOCK_HASH_ENCODING_LENGTH = ETH_WORD_SIZE;

    uint256 public constant CALL_ASSUMPTIONS_ENCODING_LENGTH = PROVER_CONTRACT_ADDRESS_ENCODING_LENGTH
        + FUNCTION_SELECTOR_ENCODING_LENGTH + SETTLE_BLOCK_NUMBER_ENCODING_LENGTH + SETTLE_BLOCK_HASH_ENCODING_LENGTH;

    /// @notice Validates if the provided CallAssumptions matches the block hash of the given block number.
    /// @param assumptions The CallAssumptions struct to validate.
    /// @return isValid True if the assumptions's block hash matches the block hash of the block number, false otherwise.
    function validateAssumptions(CallAssumptions memory assumptions) internal view returns (bool isValid) {
        return assumptions.settleBlockHash == blockhash(assumptions.settleBlockNumber);
    }
}
