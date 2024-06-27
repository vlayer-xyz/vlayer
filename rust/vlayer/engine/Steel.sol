// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.13;

/// @title Steel Library
/// @notice This library provides a collection of utilities to work with Steel commitments in Solidity.
library Steel {
    /// @notice A Commitment struct representing a block number and its block hash.
    #[derive(serde::Deserialize, serde::Serialize, Debug, alloy_rlp_derive::RlpEncodable, alloy_rlp_derive::RlpDecodable)]
    struct Commitment {
        uint16 offset;
        uint32 length;
        uint16 version;
        uint64 chainId;
        uint256 blockNumber; // Block number at which the commitment was made.
        bytes32 blockHash; // Hash of the block at the specified block number.
        bytes seal;
    }

    /// @notice Validates if the provided Commitment matches the block hash of the given block number.
    /// @param commitment The Commitment struct to validate.
    /// @return isValid True if the commitment's block hash matches the block hash of the block number, false otherwise.
    function validateCommitment(Commitment memory commitment) internal view returns (bool isValid) {
        return commitment.blockHash == blockhash(commitment.blockNumber);
    }
}
