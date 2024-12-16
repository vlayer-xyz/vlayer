// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Test, console} from "forge-std-1.9.4/src/Test.sol";
import {VTest, VTEST_CHAIN_ID} from "../../src/testing/VTest.sol";

contract VTestIntegration is Test {
    function test_vTestRevertsWhenCreatedWithInvalidChainID() public {
        if (block.chainid == VTEST_CHAIN_ID) {
            console.log("This test should be run with the `forge test` command");
            return;
        }
        vm.expectRevert("Incorrect test chain ID. Make sure you call the tests using `vlayer test` command");
        new VTest();
    }
}
