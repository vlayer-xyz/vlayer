// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { VerificationFailed } from "risc0-ethereum/IRiscZeroVerifier.sol";
import { SimpleProver } from "./SimpleProver.sol";
import { VTest, Proof, CHEATCODES } from "vlayer/testing/VTest.sol";
import { Simple } from "./SimpleVerifier.sol";

interface IFakeCheatcode {
    function thisCheatCodeDoesNotExist() external returns (bool);
}

contract ProverTest is VTest {
    SimpleProver private prover;
    Simple private verifier;

    function setUp() public {
        prover = new SimpleProver();
        verifier = new Simple(address(prover));
    }

    function test_ChainId() public view {
        assertEq(block.chainid, 55511555);
    }

    function test_sumDoesNotRevertWithCallProver() public {
        callProver();
        assertEq(prover.sum(1, 2), 3);

        Proof memory proof = getProof();
        verifier.updateSum(proof, 3);
        assertEq(verifier.latestSum(), 3);
    }

    function test_worksAfterRollingBlock() public {
        vm.roll(420);
        callProver();
        assertEq(prover.sum(420, 69), 489);

        Proof memory proof = getProof();
        assertEq(proof.commitment.settleBlockNumber, 420);

        verifier.updateSum(proof, 489);
        assertEq(verifier.latestSum(), 489);
    }

    function test_revertsOnIncorrectProof() public {
        callProver();
        assertEq(prover.sum(1, 2), 3);

        Proof memory proof = getProof();
        vm.expectRevert(abi.encodeWithSelector(VerificationFailed.selector));
        verifier.updateSum(proof, 4);

        assertEq(verifier.latestSum(), 0);
    }

    // NOTE: vm.expectRevert doesn't work correctly with errors thrown by inspectors, so we check manually
    function test_unexpectedCheatCodeCallFails() public {
        // solhint-disable-next-line avoid-low-level-calls
        (bool result, bytes memory error) = CHEATCODES.call(
            abi.encodeWithSelector(
                IFakeCheatcode.thisCheatCodeDoesNotExist.selector
            )
        );
        assertFalse(result);
        assertEq(error, abi.encodeWithSignature("Error(string)", "Unexpected vlayer cheatcode call"));
    }
}
