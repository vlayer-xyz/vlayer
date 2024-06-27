// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

// Below contract is executed off-chain to generate proof that the prover owns certain NFT.
// tokenId and msg.sender would be privately provided to the prover
// Such proof can be used by on-chain smart contract to claim airdrop or any other logic

import { VlayerProver } from "@vlayer/VlayerProver.v.sol";

interface IERC721 {
    function balanceOf(address owner) external view returns (uint256 balance);
}


address constant BYAC_NFT_ADDR = 0x1744aC92e0Ff310Ff836bB68d56D4159E37D0BdF;

contract NftOwnership is VlayerProver  {

    function require_byac_nft() public view {
      // Terminate proving if NFT is not owned by the prover
      require(
        IERC721(BYAC_NFT_ADDR).balanceOf(msg.sender) > 0, 
        "You are not owning any specified NFT"
      );
    } 

    function main() public returns (address) {  
      // 🔥 Teleport to chain on which the verification is happening
      setChainId(1); 
      // set block no in future 
      setBlockNumber(21_000_000); 
      
      require_byac_nft();

      // anything returned here would be visible to the public
      return (msg.sender); 
    }
}
