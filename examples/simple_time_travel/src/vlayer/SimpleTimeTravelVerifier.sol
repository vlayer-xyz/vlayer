// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.23;

import {SimpleTimeTravelProver} from "./SimpleTimeTravelProver.sol";
import {ExampleNFT} from "./ExampleNFT.sol";

import {Proof} from "vlayer/Proof.sol";
import {Verifier} from "vlayer/Verifier.sol";

contract SimpleTimeTravel is Verifier {
    address public prover;
    mapping(address => bool) public claimed;
    ExampleNFT public reward;

    constructor(address _prover, ExampleNFT _nft) {
        prover = _prover;
        reward = _nft;
    }

    function claim(Proof calldata, address claimer, uint256 average)
        public
        onlyVerified(prover, SimpleTimeTravelProver.averageBalanceOf.selector)
    {
        require(!claimed[claimer], "Already claimed");

        if (average >= 10_000_000) {
            claimed[claimer] = true;
            reward.mint(claimer);
        }
    }
}
