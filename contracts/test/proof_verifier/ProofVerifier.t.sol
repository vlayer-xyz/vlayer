// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";

import {IRiscZeroVerifier, Receipt, VerificationFailed} from "risc0-ethereum/IRiscZeroVerifier.sol";
import {RiscZeroMockVerifier} from "risc0-ethereum/test/RiscZeroMockVerifier.sol";

import {ProofVerifierBase} from "../../src/proof_verifier/ProofVerifierBase.sol";
import {ExecutionCommitment} from "../../src/ExecutionCommitment.sol";
import {Proof} from "../../src/Proof.sol";
import {ProofMode} from "../../src/Seal.sol";

import {TestHelpers} from "../helpers/TestHelpers.sol";

contract ProofVerifierUnderTest is ProofVerifierBase {
    constructor(IRiscZeroVerifier _verifier, ProofMode _proofMode) {
        verifier = _verifier;
        proofMode = _proofMode;
    }
}

contract ProofVerifier_Verify_Tests is Test {
    RiscZeroMockVerifier mockVerifier = new RiscZeroMockVerifier(bytes4(0));
    ProofVerifierUnderTest verifier = new ProofVerifierUnderTest(mockVerifier, ProofMode.FAKE);

    ExecutionCommitment commitment;

    address PROVER = address(1);
    bytes4 SELECTOR = 0x01020304;

    function setUp() public {
        vm.roll(100); // have some historical blocks

        commitment = ExecutionCommitment(PROVER, SELECTOR, block.number - 1, blockhash(block.number - 1));
    }

    function createProof(bytes memory journalParams) public view returns (Proof memory, bytes32) {
        bytes memory journal = TestHelpers.concat(abi.encode(commitment), journalParams);
        bytes32 journalHash = sha256(journal);

        bytes memory seal = mockVerifier.mockProve(verifier.GUEST_ID(), journalHash).seal;
        Proof memory proof = Proof(journal.length, TestHelpers.encodeSeal(seal), commitment);
        return (proof, journalHash);
    }

    function createProof() public view returns (Proof memory, bytes32) {
        bytes memory emptyBytes = new bytes(0);
        return createProof(emptyBytes);
    }

    function test_verifySuccess() public view {
        (Proof memory proof, bytes32 journalHash) = createProof();
        verifier.verify(proof, journalHash, PROVER, SELECTOR);
    }

    function test_invalidProofMode() public {
        (Proof memory proof, bytes32 journalHash) = createProof();

        // clear last byte, where ProofMode lives
        proof.seal.rhv = ((proof.seal.rhv >> 8) << 8);

        vm.expectRevert("Invalid proof mode");
        verifier.verify(proof, journalHash, PROVER, SELECTOR);
    }

    function test_invalidProver() public {
        commitment.startContractAddress = address(0x0000000000000000000000000000000000deadbeef);
        (Proof memory proof, bytes32 journalHash) = createProof();

        vm.expectRevert("Invalid prover");
        verifier.verify(proof, journalHash, PROVER, SELECTOR);
    }

    function test_invalidSelector() public {
        commitment.functionSelector = 0xdeadbeef;
        (Proof memory proof, bytes32 journalHash) = createProof();

        vm.expectRevert("Invalid selector");
        verifier.verify(proof, journalHash, PROVER, SELECTOR);
    }

    function test_blockFromFuture() public {
        commitment.settleBlockNumber = block.number;
        (Proof memory proof, bytes32 journalHash) = createProof();

        vm.expectRevert("Invalid block number: block from future");
        verifier.verify(proof, journalHash, PROVER, SELECTOR);
    }

    function test_blockOlderThanLast256Blocks() public {
        vm.roll(block.number + 256); // forward block number
        (Proof memory proof, bytes32 journalHash) = createProof();

        vm.expectRevert("Invalid block number: block too old");
        verifier.verify(proof, journalHash, PROVER, SELECTOR);
    }

    function test_invalidBlockHash() public {
        commitment.settleBlockHash = blockhash(commitment.settleBlockNumber - 1);
        (Proof memory proof, bytes32 journalHash) = createProof();

        vm.expectRevert("Invalid block hash");
        verifier.verify(proof, journalHash, PROVER, SELECTOR);
    }
}
