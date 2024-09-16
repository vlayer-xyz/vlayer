// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.23;

import {SimpleTravelProver} from "./SimpleTravelProver.sol";
import {ExampleNFT} from "./ExampleNFT.sol";

import {Proof} from "vlayer-0.1.0/src/Proof.sol";
import {Verifier} from "vlayer-0.1.0/src/Verifier.sol";

contract SimpleTravel is Verifier {
    address public prover;
    mapping(address => bool) public claimed;
    ExampleNFT public reward;

    constructor(address _prover, ExampleNFT _nft) {
        prover = _prover;
        reward = _nft;
    }

    function claim(Proof calldata, address claimer, uint256 crossChainBalance)
        public
        onlyVerified(prover, SimpleTravelProver.crossChainBalanceOf.selector)
    {
        require(!claimed[claimer], "Already claimed");

        if (crossChainBalance >= 10_000_000) {
            claimed[claimer] = true;
            reward.mint(claimer);
        }
    }
}
