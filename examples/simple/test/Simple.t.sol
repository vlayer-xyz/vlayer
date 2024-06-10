// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {Simple} from "../src/Simple.sol";

contract SimpleTest is Test {
    Simple public simple;

    function setUp() public {
        simple = new Simple();
        simple.setNumber(0);
    }

    function test_Increment() public {
        simple.increment();
        assertEq(simple.number(), 1);
    }

    function testFuzz_SetNumber(uint256 x) public {
        simple.setNumber(x);
        assertEq(simple.number(), x);
    }
}
