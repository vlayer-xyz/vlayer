// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {EmailDomainProver} from "./EmailDomainProver.sol";

import {Proof} from "vlayer-0.1.0/Proof.sol";
import {Verifier} from "vlayer-0.1.0/Verifier.sol";
import {ERC721} from "@openzeppelin-contracts-5.0.1/token/ERC721/ERC721.sol";

contract EmailDomainVerifier is Verifier, ERC721 {
    address public prover;
    CompanyNFT public nft;

    mapping(bytes32 => address) public emailHashToAddr;
    mapping(address => string) public addrToEmailDomain;
    uint256 public currentTokenId;

    constructor(address _prover) ERC721("EmailNFT", "EML"){
        prover = _prover;
    }

    function verify(Proof calldata, bytes32 _emailHash, address _targetWallet, string memory _emailDomain)
        public
        onlyVerified(prover, EmailDomainProver.main.selector)
    {
        require(emailHashToAddr[_emailHash] == address(0), "email taken");
        emailHashToAddr[_emailHash] = _targetWallet;
        addrToEmailDomain[_targetWallet] = _emailDomain;
        currentTokenId++;
        _safeMint(_targetWallet, currentTokenId);
    }
}

