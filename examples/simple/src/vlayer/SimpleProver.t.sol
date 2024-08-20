// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {SimpleProver} from "./SimpleProver.sol";
import "vlayer/testing/VTest.sol";
import "./SimpleVerifier.sol";

interface IFakeCheatcode {
    function thisCheatCodeDoesNotExist() external returns (bool);
}

contract ProverTest is VTest {
    function test_ChainId() public {
        assertEq(block.chainid, 55511555);
    }

    function test_sumDoesNotRevertWithCallProver() public {
        SimpleProver prover = new SimpleProver();
        vm.roll(15537395);
        callProver();
        assertEq(prover.sum(1, 2), 3);
        Proof memory proof = getProof();
        Simple verifier = new Simple(address(prover));
        verifier.updateSum(proof, 3);
    }

    // NOTE: vm.expectRevert doesn't work correctly with errors thrown by inspectors, so we check manually
    function test_unexpectedCheatCodeCallFails() public {
        (bool result, bytes memory error) = CHEATCODES.call(abi.encodeWithSelector(IFakeCheatcode.thisCheatCodeDoesNotExist.selector));
        assertFalse(result);
        assertEq(error, abi.encodeWithSignature("Error(string)", "Unexpected vlayer cheatcode call"));
    }
}
