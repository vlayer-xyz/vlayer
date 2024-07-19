// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Prover} from "vlayer/Prover.sol";

contract SimpleProver is Prover {
    constructor() {}

    function sum(uint256 lhs, uint256 rhs) public returns (uint256) {
        return lhs + rhs;
    }
}
