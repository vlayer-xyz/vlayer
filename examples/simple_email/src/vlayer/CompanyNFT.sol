// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {ERC721} from "@openzeppelin-contracts-5.0.1/token/ERC721/ERC721.sol";

contract CompanyNFT is ERC721 {
    uint256 public currentTokenId = 1;

    constructor(string memory _companyName, string memory _companySymbol) ERC721(_companyName, _companySymbol) {}

    function mint(address to) public {
        _mint(to, currentTokenId);
        currentTokenId++;
    }
}
