// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Proof} from "vlayer-0.1.0/Proof.sol";
import {Verifier} from "vlayer-0.1.0/Verifier.sol";

import {SimpleProver} from "./SimpleProver.sol";
import {ExampleNFT} from "./ExampleNFT.sol";

contract SimpleVerifier is Verifier {
    address public prover;
    ExampleNFT public whaleNFT;

    mapping(address => bool) public claimed;

    constructor(address _prover, address _nft) {
        prover = _prover;
        whaleNFT = ExampleNFT(_nft);
    }

    function claimWhale(Proof calldata, address claimer, uint256 balance)
        public
        onlyVerified(prover, SimpleProver.balance.selector)
    {
        require(!claimed[claimer], "Already claimed");

        if (balance > 10_000_000) {
            claimed[claimer] = true;
            whaleNFT.mint(claimer);
        }
    }
}
