// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {WebProofProver} from "./WebProofProver.sol";

import {Proof} from "vlayer-0.1.0/Proof.sol";
import {Verifier} from "vlayer-0.1.0/Verifier.sol";

import {ERC721} from "@openzeppelin-contracts-5.0.1/token/ERC721/ERC721.sol";

contract WebProofVerifier is Verifier {
    address public prover;
    mapping(address => bytes32) private accountHashes;
    mapping(bytes32 => bool) private registeredHashes;

    constructor(address _prover) {
        prover = _prover;
    }

    function verify(Proof calldata, bytes32 accountNumberHash, address account)
        public
        onlyVerified(prover, WebProofProver.main.selector)
    {
        require(accountHashes[account] == bytes32(0), "Account already registered");
        require(!registeredHashes[accountNumberHash], "Account number hash already registered");

        accountHashes[account] = accountNumberHash;
        registeredHashes[accountNumberHash] = true;
    }

    function isVerifiedWithIban(address account) public view returns (bool) {
        return accountHashes[account] != bytes32(0);
    }
}
