// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.13;

/// @title Steel Library
/// @notice This library provides a collection of utilities to work with Steel commitments in Solidity.
library Steel {
    /// @notice A Commitment struct representing a block number and its block hash.
    #[derive(serde::Deserialize, serde::Serialize, Debug, alloy_rlp_derive::RlpEncodable)]
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
