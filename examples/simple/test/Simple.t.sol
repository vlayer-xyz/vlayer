// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {Simple} from "../src/Simple.sol";

contract SimpleTest is Test {
    Simple public simple;

    function setUp() public {
        simple = new Simple();
    }

    function test_Sum() public view {
        assertEq(simple.sum(2, 2), 4);
    }
}
