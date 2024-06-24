// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import { ERC721 } from "@openzeppelin/contracts/token/ERC721/ERC721.sol";

interface IERC721 {
    function ownerOf(uint256 tokenId) external view returns (address owner);
}

contract Airdrop is ERC721("GameItem", "ITM") {
    address public targetNftAddr;
    uint256 public lastClaimedId = 0;

    mapping(uint256 => bool) public claimed;

    constructor (address _targetNftAddr) {
        targetNftAddr = _targetNftAddr;
    } 

    function claim(uint tokenId) public {
        require(
            IERC721(targetNftAddr).ownerOf(tokenId) == msg.sender, 
            "You are not the owner of the given NFT"
        ); // check if caller is the owner of the given NFT
        require(!claimed[tokenId], "NFT already claimed"); // check if given NFT is already claimed


        claimed[tokenId] = true;
        lastClaimedId += 1;
        _mint(msg.sender, lastClaimedId); // deliver Airdrop NFT as reward
    }
}
