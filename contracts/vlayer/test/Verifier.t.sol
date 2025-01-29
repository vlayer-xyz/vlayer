// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Test} from "forge-std-1.9.4/src/Test.sol";
import {TestHelpers} from "./helpers/TestHelpers.sol";

import {IRiscZeroVerifier, Receipt, VerificationFailed} from "risc0-ethereum-1.2.0/src/IRiscZeroVerifier.sol";
import {RiscZeroMockVerifier} from "risc0-ethereum-1.2.0/src/test/RiscZeroMockVerifier.sol";

import {CallAssumptions} from "../src/CallAssumptions.sol";
import {Proof} from "../src/Proof.sol";

import {FakeProofVerifier} from "../src/proof_verifier/FakeProofVerifier.sol";
import {Verifier, IProofVerifier} from "../src/Verifier.sol";
import {IImageIdRepository} from "../src/Repository.sol";
import {Groth16ProofVerifier} from "../src/proof_verifier/Groth16ProofVerifier.sol";

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

    function verifyWithString(Proof calldata, string calldata value)
        external
        view
        onlyVerified(PROVER, SIMPLE_PROVER_SELECTOR)
        returns (string memory)
    {
        return value;
    }
}

contract Verifier_OnlyVerified_Modifier_Tests is Test {
    ExampleVerifier exampleVerifier = new ExampleVerifier();
    TestHelpers helpers = new TestHelpers();

    CallAssumptions callAssumptions;

    function setUp() public {
        vm.roll(100); // have some historical blocks

        callAssumptions = CallAssumptions(
            exampleVerifier.PROVER(), ExampleProver.doSomething.selector, block.number - 1, blockhash(block.number - 1)
        );
    }

    function test_verifySuccess() public view {
        (Proof memory proof,) = helpers.createProof(callAssumptions);
        exampleVerifier.verifySomething(proof);
    }

    function test_proofAndJournalDoNotMatch() public {
        (Proof memory proof,) = helpers.createProof(callAssumptions);
        proof.callAssumptions.settleBlockNumber -= 1;
        proof.callAssumptions.settleBlockHash = blockhash(proof.callAssumptions.settleBlockNumber);

        vm.expectRevert(VerificationFailed.selector);
        exampleVerifier.verifySomething(proof);
    }

    function test_journaledParams() public view {
        bool value = true;
        (Proof memory proof,) = helpers.createProof(callAssumptions, value);

        assertEq(exampleVerifier.verifySomethingElse(proof, value), value);
    }

    function test_journaledParamCannotBeChanged() public {
        bool value = true;
        (Proof memory proof,) = helpers.createProof(callAssumptions, value);

        value = !value;

        vm.expectRevert(VerificationFailed.selector);
        assertEq(exampleVerifier.verifySomethingElse(proof, value), value);
    }

    function test_functionCanHaveNonJournaledParams() public view {
        (Proof memory proof,) = helpers.createProof(callAssumptions);

        assertEq(exampleVerifier.verifySomethingElse(proof, true), true);
        assertEq(exampleVerifier.verifySomethingElse(proof, false), false);
    }

    function test_journaledStringParam() public view {
        string memory userParam = "abc";
        (Proof memory proof,) = helpers.createProof(callAssumptions, userParam);

        assertEq(exampleVerifier.verifyWithString(proof, userParam), userParam);
    }

    function test_functionCanHaveNonJournaledStringParams() public view {
        (Proof memory proof,) = helpers.createProof(callAssumptions);

        assertEq(exampleVerifier.verifyWithString(proof, "xyz"), "xyz");
    }

    function test_journaledStringParamCannotBeChanged() public {
        string memory value = "abc";
        (Proof memory proof,) = helpers.createProof(callAssumptions, value);

        value = "def";

        vm.expectRevert(VerificationFailed.selector);
        exampleVerifier.verifyWithString(proof, value);
    }
}

contract Verifier_SetTestVerifier is Test {
    ExampleVerifier exampleVerifier = new ExampleVerifier();
    TestHelpers helpers = new TestHelpers();

    CallAssumptions callAssumptions;

    function setUp() external {
        vm.roll(100); // have some historical blocks

        callAssumptions = CallAssumptions(
            exampleVerifier.PROVER(), ExampleProver.doSomething.selector, block.number - 1, blockhash(block.number - 1)
        );
    }

    function test_RevertsIf_NotCalledOnDevChain() external {
        vm.chainId(1);
        vm.expectRevert("Changing verifiers is only allowed on devnet");
        exampleVerifier._setTestVerifier(IProofVerifier(address(123)));

        vm.chainId(420);
        vm.expectRevert("Changing verifiers is only allowed on devnet");
        exampleVerifier._setTestVerifier(IProofVerifier(address(123)));

        vm.chainId(11155111);
        vm.expectRevert("Changing verifiers is only allowed on devnet");
        exampleVerifier._setTestVerifier(IProofVerifier(address(123)));

        vm.chainId(8453);
        vm.expectRevert("Changing verifiers is only allowed on devnet");
        exampleVerifier._setTestVerifier(IProofVerifier(address(123)));
    }

    function test_RevertsIf_RepositoryIsNotSetForVerifier() external {
        FakeProofVerifier newVerifier = new FakeProofVerifier(IImageIdRepository(address(0)));
        vm.expectRevert("Verifier's repository address is not set");
        exampleVerifier._setTestVerifier(newVerifier);
    }

    function test_ReplacesInternalVerifier() external {
        Groth16ProofVerifier newVerifier = new Groth16ProofVerifier(exampleVerifier.verifier().imageIdRepository());
        exampleVerifier._setTestVerifier(newVerifier);
        assertEq(address(exampleVerifier.verifier()), address(newVerifier));
    }
}
