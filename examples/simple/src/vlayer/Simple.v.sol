// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Simple} from "../Simple.sol";

contract SimpleVerification {
    Simple public simple;

    constructor(Simple _simple) {
        simple = _simple;
    }

    function sum(uint256 lhs, uint256 rhs) public returns (uint256) {
        address set_block_contract = 0x1234567890AbcdEF1234567890aBcdef12345678;
        (bool _success1, ) = set_block_contract.call("");
        address exmaple_contract = 0x1111222233334444555566667777888899990000;
        (bool _success2, ) = exmaple_contract.call("");
        return lhs + rhs;
    }
}
