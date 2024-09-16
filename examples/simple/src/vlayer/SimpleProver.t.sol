// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {SimpleProver} from "./SimpleProver.sol";
import {VTest, Proof, CHEATCODES, VerificationFailed} from "vlayer/testing/VTest.sol";
import {SimpleVerifier} from "./SimpleVerifier.sol";
import {ExampleToken} from "./ExampleToken.sol";
import {ExampleNFT} from "./ExampleNFT.sol";

interface IFakeCheatcode {
    function thisCheatCodeDoesNotExist() external returns (bool);
}

contract ProverTest is VTest {
    SimpleProver private prover;
    SimpleVerifier private verifier;
    ExampleNFT private rewardNFT;
    ExampleToken private exampleErc20;

    address john = vm.addr(1);
    address harry = vm.addr(2);
    uint256 initBalance = 10_000_000_000;

    function setUp() public {
        exampleErc20 = new ExampleToken(john, initBalance);
        rewardNFT = new ExampleNFT();
        prover = new SimpleProver(exampleErc20, vm.getBlockNumber());
        verifier = new SimpleVerifier(address(prover), address(rewardNFT));
    }

    function test_mintWhaleNFT() public {
        callProver();
        (address returnedOwner, uint256 returnedBalance) = prover.balance(exampleErc20, john);
        assertEq(returnedOwner, john);
        assertEq(returnedBalance, initBalance);
        
        // (address returnedOwner, uint returnedBalance) = prover.balance(john);
        // assertEq(returnedOwner, john);
        // assertEq(returnedBalance, initBalance);

        // Proof memory proof = getProof();
        // verifier.claimWhale(proof, john, initBalance);
        // assertEq(verifier.claimed(john), true);
    }

    // function test_revertsOnIncorrectProof() public {
    //     callProver();
    //     prover.balance(harry);

    //     Proof memory proof = getProof();
    //     vm.expectRevert(abi.encodeWithSelector(VerificationFailed.selector));
    //     verifier.claimWhale(proof, john, initBalance);

    //     assertEq(verifier.claimed(john), false);
    // }

    // // NOTE: vm.expectRevert doesn't work correctly with errors thrown by inspectors, so we check manually
    // function test_unexpectedCheatCodeCallFails() public {
    //     // solhint-disable-next-line avoid-low-level-calls
    //     (bool result, bytes memory error) =
    //         CHEATCODES.call(abi.encodeWithSelector(IFakeCheatcode.thisCheatCodeDoesNotExist.selector));
    //     assertFalse(result);
    //     assertEq(error, abi.encodeWithSignature("Error(string)", "Unexpected vlayer cheatcode call"));
    // }
}
