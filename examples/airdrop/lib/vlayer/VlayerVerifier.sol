// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

contract VlayerVerifier {
  struct VProof {
    uint16 offset;
    uint32 length;
    uint16 version;
    uint32 chainId;
    uint128 blockNumber;
    bytes32 blockHash;
    bytes seal;    
  }

  modifier onlyVerified() {
    // TODO: Check if msg.calldata contains a valid VProof
    _;
  }
}