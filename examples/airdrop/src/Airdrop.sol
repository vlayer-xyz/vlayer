// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

interface IERC721 {
    function ownerOf(uint256 tokenId) external view returns (address owner);
}

contract Airdrop {
    address public targetNftAddr;

    mapping(uint256 => bool) public claimed;

    constructor (address _targetNftAddr) {
        targetNftAddr = _targetNftAddr;
    } 

    function claim(uint tokenId) public {
        require(!claimed[tokenId], "NFT already claimed");
        require(
            IERC721(targetNftAddr).ownerOf(tokenId) == msg.sender, 
            "You are not the owner of the given NFT"
        );

        claimed[tokenId] = true;
    }
}
