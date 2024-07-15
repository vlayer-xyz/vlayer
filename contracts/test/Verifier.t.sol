// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {TestHelpers} from "./helpers/TestHelpers.sol";
import {VerifierUnderTest} from "./helpers/VerifierUnderTest.sol";

import {IRiscZeroVerifier, Receipt, VerificationFailed} from "risc0-ethereum/IRiscZeroVerifier.sol";
import {RiscZeroMockVerifier} from "risc0-ethereum/test/RiscZeroMockVerifier.sol";

import {ExecutionCommitment} from "../src/ExecutionCommitment.sol";
import {Proof} from "../src/Proof.sol";

import {FakeProofVerifier} from "../src/proof_verifier/FakeProofVerifier.sol";

contract Prover {}

contract ExampleProver is Prover {
    function doSomething() public pure returns (uint256) {
        return 42;
    }
}

contract ExampleVerifier is VerifierUnderTest {
    address public immutable PROVER;

    bytes4 constant SIMPLE_PROVER_SELECTOR = ExampleProver.doSomething.selector;

    constructor() {
        PROVER = address(new ExampleProver());
    }

    function verifySomething(Proof calldata) external onlyVerified(PROVER, SIMPLE_PROVER_SELECTOR) returns (bool) {
        return true;
    }

    function verifySomethingElse(Proof calldata, bool value)
        external
        onlyVerified(PROVER, SIMPLE_PROVER_SELECTOR)
        returns (bool)
    {
        return value;
    }
}

contract Verifier_OnlyVerified_Modifier_Tests is Test {
    ExampleVerifier exampleVerifier = new ExampleVerifier();
    RiscZeroMockVerifier mockVerifier = new RiscZeroMockVerifier(bytes4(0));
    FakeProofVerifier mockProofVerifier = new FakeProofVerifier();

    ExecutionCommitment commitment;

    function setUp() public {
        vm.roll(100); // have some historical blocks

        commitment = ExecutionCommitment(
            exampleVerifier.PROVER(), ExampleProver.doSomething.selector, block.number - 1, blockhash(block.number - 1)
        );

        exampleVerifier.setVerifier(mockProofVerifier);
    }

    function createProof(bytes memory journalParams) public view returns (Proof memory) {
        bytes memory journal = TestHelpers.concat(abi.encode(commitment), journalParams);

        bytes memory seal = mockVerifier.mockProve(mockProofVerifier.GUEST_ID(), sha256(journal)).seal;
        return Proof(journal.length, TestHelpers.encodeSeal(seal), commitment);
    }

    function createProof() public view returns (Proof memory) {
        bytes memory emptyBytes = new bytes(0);
        return createProof(emptyBytes);
    }

    function test_verifySuccess() public {
        Proof memory proof = createProof();
        exampleVerifier.verifySomething(proof);
    }

    function test_invalidProver() public {
        commitment.startContractAddress = address(0x0000000000000000000000000000000000deadbeef);
        Proof memory proof = createProof();

        vm.expectRevert("Invalid prover");
        exampleVerifier.verifySomething(proof);
    }

    function test_invalidSelector() public {
        commitment.functionSelector = 0xdeadbeef;
        Proof memory proof = createProof();

        vm.expectRevert("Invalid selector");
        exampleVerifier.verifySomething(proof);
    }

    function test_blockFromFuture() public {
        commitment.settleBlockNumber = block.number;
        Proof memory proof = createProof();

        vm.expectRevert("Invalid block number: block from future");
        exampleVerifier.verifySomething(proof);
    }

    function test_blockOlderThanLast256Blocks() public {
        vm.roll(block.number + 256); // forward block number
        Proof memory proof = createProof();

        vm.expectRevert("Invalid block number: block too old");
        exampleVerifier.verifySomething(proof);
    }

    function test_invalidBlockHash() public {
        commitment.settleBlockHash = blockhash(commitment.settleBlockNumber - 1);
        Proof memory proof = createProof();

        vm.expectRevert("Invalid block hash");
        exampleVerifier.verifySomething(proof);
    }

    function test_proofAndJournalDoNotMatch() public {
        Proof memory proof = createProof();
        proof.commitment.settleBlockNumber -= 1;
        proof.commitment.settleBlockHash = blockhash(proof.commitment.settleBlockNumber);

        vm.expectRevert(VerificationFailed.selector);
        exampleVerifier.verifySomething(proof);
    }

    function test_functionCanJournaledParams() public {
        bool value = true;
        Proof memory proof = createProof(abi.encode(value));

        assertEq(exampleVerifier.verifySomethingElse(proof, value), value);
    }

    function test_journaledParamCannotBeChanged() public {
        bool value = true;
        Proof memory proof = createProof(abi.encode(value));

        value = !value;

        vm.expectRevert(VerificationFailed.selector);
        assertEq(exampleVerifier.verifySomethingElse(proof, value), value);
    }

    function test_functionCanHaveNonJournaledParams() public {
        Proof memory proof = createProof();

        assertEq(exampleVerifier.verifySomethingElse(proof, true), true);
        assertEq(exampleVerifier.verifySomethingElse(proof, false), false);
    }
}
