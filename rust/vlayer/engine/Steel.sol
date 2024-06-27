// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.13;

/// @title Steel Library
/// @notice This library provides a collection of utilities to work with Steel commitments in Solidity.
library Steel {
    /// @notice An ExecutionCommitment struct representing a block number and its block hash.
    #[derive(serde::Deserialize, serde::Serialize, Debug)]
    struct ExecutionCommitment {    
        address startContractAddress;
        bytes4 functionSelector;
        uint256 settleBlockNumber; // Block number at which the commitment was made.
        bytes32 settleBlockHash; // Hash of the block at the specified block number.
    }

    /// @notice Validates if the provided ExecutionCommitment matches the block hash of the given block number.
    /// @param commitment The ExecutionCommitment struct to validate.
    /// @return isValid True if the commitment's block hash matches the block hash of the block number, false otherwise.
    function validateCommitment(ExecutionCommitment memory commitment) internal view returns (bool isValid) {
        return commitment.blockHash == blockhash(commitment.blockNumber);
    }
}
