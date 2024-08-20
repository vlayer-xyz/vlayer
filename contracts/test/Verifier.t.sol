// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {TestHelpers} from "./helpers/TestHelpers.sol";

import {IRiscZeroVerifier, Receipt, VerificationFailed} from "risc0-ethereum/IRiscZeroVerifier.sol";
import {RiscZeroMockVerifier} from "risc0-ethereum/test/RiscZeroMockVerifier.sol";

import {ExecutionCommitment} from "../src/ExecutionCommitment.sol";
import {Proof} from "../src/Proof.sol";

import {FakeProofVerifier} from "../src/proof_verifier/FakeProofVerifier.sol";
import {Verifier} from "../src/Verifier.sol";

contract Prover {}

contract ExampleProver is Prover {
    function doSomething() public pure returns (uint256) {
        return 42;
    }
}

contract ExampleVerifier is Verifier {
    address public immutable PROVER;
    bytes4 constant SIMPLE_PROVER_SELECTOR = ExampleProver.doSomething.selector;

    constructor() {
        PROVER = address(new ExampleProver());
    }

    function verifySomething(Proof calldata)
        external
        view
        onlyVerified(PROVER, SIMPLE_PROVER_SELECTOR)
        returns (bool)
    {
        return true;
    }

    function verifySomethingElse(Proof calldata, bool value)
        external
        view
        onlyVerified(PROVER, SIMPLE_PROVER_SELECTOR)
        returns (bool)
    {
        return value;
    }
}

contract Verifier_OnlyVerified_Modifier_Tests is Test {
    ExampleVerifier exampleVerifier = new ExampleVerifier();
    TestHelpers helpers = new TestHelpers();

    ExecutionCommitment commitment;

    function setUp() public {
        vm.roll(100); // have some historical blocks

        commitment = ExecutionCommitment(
            exampleVerifier.PROVER(), ExampleProver.doSomething.selector, block.number - 1, blockhash(block.number - 1)
        );
    }

    function test_verifySuccess() public view {
        (Proof memory proof,) = helpers.createProof(commitment);
        exampleVerifier.verifySomething(proof);
    }

    function test_proofAndJournalDoNotMatch() public {
        (Proof memory proof,) = helpers.createProof(commitment);
        proof.commitment.settleBlockNumber -= 1;
        proof.commitment.settleBlockHash = blockhash(proof.commitment.settleBlockNumber);

        vm.expectRevert(VerificationFailed.selector);
        exampleVerifier.verifySomething(proof);
    }

    function test_journaledParams() public view {
        bool value = true;
        (Proof memory proof,) = helpers.createProof(commitment, abi.encode(value));

        assertEq(exampleVerifier.verifySomethingElse(proof, value), value);
    }

    function test_journaledParamCannotBeChanged() public {
        bool value = true;
        (Proof memory proof,) = helpers.createProof(commitment, abi.encode(value));

        value = !value;

        vm.expectRevert(VerificationFailed.selector);
        assertEq(exampleVerifier.verifySomethingElse(proof, value), value);
    }

    function test_functionCanHaveNonJournaledParams() public view {
        (Proof memory proof,) = helpers.createProof(commitment);

        assertEq(exampleVerifier.verifySomethingElse(proof, true), true);
        assertEq(exampleVerifier.verifySomethingElse(proof, false), false);
    }
}
