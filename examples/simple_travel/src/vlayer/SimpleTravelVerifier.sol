// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.23;

import {SimpleTravelProver} from "./SimpleTravelProver.sol";

import {Proof} from "vlayer/Proof.sol";
import {Verifier} from "vlayer/Verifier.sol";

contract SimpleTravel is Verifier {
    address public prover;
    mapping(address => bool) public claimed;
    mapping(address => uint256) public claimedSum;

    constructor(address _prover) {
        prover = _prover;
    }

    function claim(
        Proof calldata, address claimer, uint256 sum
    ) public onlyVerified(prover, SimpleTravelProver.proveMultiChainOwnership.selector) {
        require(!claimed[claimer], "Already claimed");
        claimed[claimer] = true;
        claimedSum[claimer] = sum;
    }
}
