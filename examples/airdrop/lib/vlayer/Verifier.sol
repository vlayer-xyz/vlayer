// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

// Below contract is executed on-chain. Verification may happen on any EVM chain.

contract Verifier {
  struct Proof {
    uint16 offset;
    uint32 length;
    uint16 version;
    uint64 chainId;
    uint256 blockNumber; // Block number at which the commitment was made.
    bytes32 blockHash; // Hash of the block at the specified block number.
    bytes seal;
  }

  modifier onlyVerified(address contractAddr, bytes4 functionSelector) {
    // TODO: Check if msg.calldata contains a valid VProof
    _;
  }
}