// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {ERC721} from "@openzeppelin-contracts/token/ERC721/ERC721.sol";

contract HodlerBadgeNFT is ERC721 {
    uint256 public currentTokenId = 1;

    constructor() ERC721("HodlerBadgeNFT", "HODL") {}

    function mint(address to) public {
        _mint(to, currentTokenId);
        currentTokenId++;
    }
}
