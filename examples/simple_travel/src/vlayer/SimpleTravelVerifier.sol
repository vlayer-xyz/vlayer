// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.23;

import {SimpleTravelProver} from "./SimpleTravelProver.sol";
import {ExampleNFT} from "./ExampleNFT.sol";

import {Proof} from "vlayer/Proof.sol";
import {Verifier} from "vlayer/Verifier.sol";

contract SimpleTravel is Verifier {
    address public prover;
    mapping(address => bool) public claimed;
    ExampleNFT public reward;

    constructor(address _prover, ExampleNFT _nft) {
        prover = _prover;
        reward = _nft;
    }

    function claim(Proof calldata, address claimer, uint256 sum)
        public
        onlyVerified(prover, SimpleTravelProver.multichainBalanceOf.selector)
    {
        require(!claimed[claimer], "Already claimed");

        if (sum >= 10_000_000) {
            claimed[claimer] = true;
            reward.mint(claimer);
        }
    }
}
