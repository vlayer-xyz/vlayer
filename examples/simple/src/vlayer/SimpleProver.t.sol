// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {SimpleProver} from "./SimpleProver.sol";
import {VTest} from "vlayer/testing/VTest.sol";
import {Proof} from "vlayer/Proof.sol";

contract ProverTest is VTest {
    function test_sum() public {
        SimpleProver prover = new SimpleProver();
        assertEq(prover.sum(1, 2), 3);
    }

    function test_setBlockWillNotRevert() public {
        SimpleProver prover = new SimpleProver();
        callProver();
        prover.setBlock(420);
        Proof memory proof = getProof();
        assertEq(proof.commitment.functionSelector, bytes4(0));
    }

}