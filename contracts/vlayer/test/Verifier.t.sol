// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Test} from "forge-std-1.9.4/src/Test.sol";
import {TestHelpers} from "./helpers/TestHelpers.sol";

import {IRiscZeroVerifier, Receipt, VerificationFailed} from "risc0-ethereum-2.0.0/src/IRiscZeroVerifier.sol";
import {RiscZeroMockVerifier} from "risc0-ethereum-2.0.0/src/test/RiscZeroMockVerifier.sol";

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
            exampleVerifier.PROVER(),
            ExampleProver.doSomething.selector,
            block.chainid,
            block.number - 1,
            blockhash(block.number - 1)
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
    Groth16ProofVerifier newVerifier;

    CallAssumptions callAssumptions;

    function setUp() external {
        vm.roll(100); // have some historical blocks

        newVerifier = new Groth16ProofVerifier(exampleVerifier.verifier().imageIdRepository());

        callAssumptions = CallAssumptions(
            exampleVerifier.PROVER(),
            ExampleProver.doSomething.selector,
            block.chainid,
            block.number - 1,
            blockhash(block.number - 1)
        );
    }

    function test_ReplacesInternalVerifier() external {
        exampleVerifier._setTestVerifier(newVerifier);
        assertEq(address(exampleVerifier.verifier()), address(newVerifier));
    }

    function test_ChangesVerifierOnTestnets() external {
        vm.chainId(11155111);
        exampleVerifier._setTestVerifier(newVerifier);

        vm.chainId(84532);
        exampleVerifier._setTestVerifier(newVerifier);

        vm.chainId(300);
        exampleVerifier._setTestVerifier(newVerifier);
    }

    function test_RevertsIf_CalledOnMainChain() external {
        vm.chainId(1);
        vm.expectRevert("Changing verifiers is only allowed on devnet or testnet");
        exampleVerifier._setTestVerifier(newVerifier);

        vm.chainId(10);
        vm.expectRevert("Changing verifiers is only allowed on devnet or testnet");
        exampleVerifier._setTestVerifier(newVerifier);
    }

    function test_RevertsIf_NotCalledByDeployer() external {
        vm.expectRevert("Only deployer can change verifier");
        vm.prank(address(1));
        exampleVerifier._setTestVerifier(newVerifier);
    }

    function test_RevertsIf_RepositoryIsNotSetForVerifier() external {
        FakeProofVerifier invalidVerifier = new FakeProofVerifier(IImageIdRepository(address(0)));
        vm.expectRevert("Verifier's repository address is not set");
        exampleVerifier._setTestVerifier(invalidVerifier);
    }
}
