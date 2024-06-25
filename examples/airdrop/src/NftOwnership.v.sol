// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

// Below contract is executed off-chain to generate proof that the prover owns certain NFT.
// tokenId and msg.sender would be privately provided to the prover
// Such proof can be used by on-chain smart contract to claim airdrop or any other logic

import { VlayerProver } from "@vlayer/VlayerProver.v.sol";

interface IERC721 {
    function ownerOf(uint256 tokenId) external view returns (address owner);
}

contract NftOwnership is VlayerProver  {
    function main(address targetNftAddr, uint tokenId, address receiver) public returns (address,address) {  
      setChainId(1); // teleport to chain on which the verification is happening
      setBlockNumber(latestBlock() - 1); // time travel to specific block number
      
      // Terminate if NFT is not owned by the prover
      require(IERC721(targetNftAddr).ownerOf(tokenId) == msg.sender, "You are not the owner of the given NFT");

      return (targetNftAddr, receiver); // anything returned here would be visible to the public
    }
}
