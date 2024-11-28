// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {WebProofProver} from "./WebProofProver.sol";

import {Proof} from "vlayer-0.1.0/Proof.sol";
import {Verifier} from "vlayer-0.1.0/Verifier.sol";

import {ERC721} from "@openzeppelin-contracts-5.0.1/token/ERC721/ERC721.sol";

contract WebProofVerifier is Verifier, ERC721 {
    address public prover;

    constructor(address _prover) ERC721("TwitterNFT", "TNFT") {
        prover = _prover;
    }

    function verify(Proof calldata, string memory username, address account)
        public
        onlyVerified(prover, WebProofProver.main.selector)
    {
        uint256 tokenId = uint256(keccak256(abi.encodePacked(username)));
        require(_ownerOf(tokenId) == address(0), "User has already minted a TwitterNFT");

        _safeMint(account, tokenId);
    }
}
