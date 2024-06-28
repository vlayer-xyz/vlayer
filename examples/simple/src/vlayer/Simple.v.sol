// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Simple} from "../Simple.sol";

contract SimpleVerification {
    Simple public simple;

    constructor(Simple _simple) {
        simple = _simple;
    }

    function sum(uint256 lhs, uint256 rhs) public pure returns (uint256) {
        return lhs + rhs;
    }
}
