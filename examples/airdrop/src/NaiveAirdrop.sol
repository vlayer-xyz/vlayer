// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import { ERC721 } from "@openzeppelin/contracts/token/ERC721/ERC721.sol";

interface IERC721 {
    function ownerOf(uint256 tokenId) external view returns (address owner);
}

// NaiveAirdrop contract is a simple implementation of an airdrop contract.
// It rewards users with an NFT for owning a specific NFT.
// No privacy is provided... 
// Contract is public and anyone can see who claimed which TargetNFT was owned.

contract NaiveAirdrop is ERC721("GameItem", "ITM") {
    address public targetNftAddr;
    uint256 public lastClaimedId = 0;

    mapping(uint256 => bool) public claimed;

    constructor (address _targetNftAddr) {
        // address of the ERC721 NFT contract that we are checking, ie. Upcade contract
        targetNftAddr = _targetNftAddr; 
    } 

    function claim(uint tokenId) public {
        // check if caller is the owner of the given NFT
        require(
            IERC721(targetNftAddr).ownerOf(tokenId) == msg.sender, 
            "You are not the owner of the given NFT"
        ); 
       // check if given NFT is already claimed
       require(!claimed[tokenId], "NFT already claimed"); 


        claimed[tokenId] = true;
        lastClaimedId += 1;

        // deliver Airdrop NFT as reward
        _mint(msg.sender, lastClaimedId); 
    }
}
