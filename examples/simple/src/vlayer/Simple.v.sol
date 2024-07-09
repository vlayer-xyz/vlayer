// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract SimpleProver {
    constructor() {}

    function sum(uint256 lhs, uint256 rhs) public pure returns (uint256) {
        return lhs + rhs;
    }
}
