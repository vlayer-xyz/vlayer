// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

// Below contract is executed on vlayer zkEVM infrastructure

contract VlayerProver {
  uint currentChainId = 1; 
  uint currentBlockNumber;

  constructor() {
    currentBlockNumber = block.number;
  }

  function setChainId(uint chainId) public {
    currentChainId = chainId;
  }

  function setBlockNumber(uint blockNumber) public {
    currentBlockNumber = blockNumber;
  }

  function latestBlock() public view returns (uint) {
    return block.number;
  }
}