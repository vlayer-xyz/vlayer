// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {SimpleProver} from "./SimpleProver.sol";
import "vlayer/testing/VTest.sol";

contract ProverTest is VTest {
    function test_sum() public {
        SimpleProver prover = new SimpleProver();
        assertEq(prover.sum(1, 2), 3);
    }

    function test_setBlockWillNotRevert() public {
        SimpleProver prover = new SimpleProver();
        startProof();
        prover.setBlock(420);
    }

}