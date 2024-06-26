// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {Simple} from "../src/vlayer/Simple.v.sol";

contract SimpleTest is Test {

    Simple public simple;

    bytes proof; 
    bytes32 journalHash;

    function setUp() public {
        simple = new Simple();
        proof = "hello world";
        journalHash = keccak256(proof);
    }


    function test_updateSum() public {
        simple.updateSum(proof, 4, journalHash);
        assertEq(simple.latestSum(), 4);
    }
}
