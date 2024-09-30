// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {EmailDomainProver} from "./EmailDomainProver.sol";
import {CompanyNFT} from "./CompanyNFT.sol";

import {Proof} from "vlayer-0.1.0/src/Proof.sol";
import {Verifier} from "vlayer-0.1.0/src/Verifier.sol";

contract EmailDomainVerifier is Verifier {
    address public prover;
    CompanyNFT public nft;

    mapping(bytes32 => address) public emailHashToAddr;

    constructor(address _prover, string memory _nftName, string memory _nftSymbol) {
        prover = _prover;
        nft = new CompanyNFT(_nftName, _nftSymbol);
    }

    function verify(Proof calldata, bytes32 _emailHash, address _targetWallet)
        public
        onlyVerified(prover, EmailDomainProver.main.selector)
    {
        require(emailHashToAddr[_emailHash] == address(0), "email taken");
        emailHashToAddr[_emailHash] = _targetWallet;
        nft.mint(_targetWallet);
    }
}
