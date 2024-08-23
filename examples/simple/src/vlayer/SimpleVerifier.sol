// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Proof} from "vlayer/Proof.sol";
import {Verifier} from "vlayer/Verifier.sol";

import {SimpleProver} from "./SimpleProver.sol";

contract Simple is Verifier {
    address public prover;
    uint256 public latestSum;

    constructor(address _prover) {
        prover = _prover;
    }

    function updateSum(Proof calldata, uint256 sum) public onlyVerified(prover, SimpleProver.sum.selector) {
        latestSum = sum;
    }
}
