// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "./SimpleProver.sol";

contract ProverTest is Test {
    function test_sum() public {
        SimpleProver prover = new SimpleProver();
        assertEq(prover.sum(1, 2), 3);
    }
}