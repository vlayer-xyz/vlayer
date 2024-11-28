// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {ERC721} from "@openzeppelin-contracts-5.0.1/token/ERC721/ERC721.sol";

contract WhaleBadgeNFT is ERC721 {
    uint256 public currentTokenId = 1;

    constructor() ERC721("WhaleBadgeNFT", "Whale") {}

    function mint(address to) public {
        _mint(to, currentTokenId);
        currentTokenId++;
    }
}
