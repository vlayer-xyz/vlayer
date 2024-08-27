// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";

import {IRiscZeroVerifier, Receipt, VerificationFailed} from "risc0-ethereum/IRiscZeroVerifier.sol";
import {RiscZeroMockVerifier} from "risc0-ethereum/test/RiscZeroMockVerifier.sol";

import {ProofVerifierBase} from "../../src/proof_verifier/ProofVerifierBase.sol";
import {ExecutionCommitment} from "../../src/ExecutionCommitment.sol";
import {Proof} from "../../src/Proof.sol";
import {ProofMode} from "../../src/Seal.sol";

import {TestHelpers, PROVER, SELECTOR} from "../helpers/TestHelpers.sol";

contract ProofVerifierUnderTest is ProofVerifierBase {
    constructor(IRiscZeroVerifier _verifier, ProofMode _proofMode) {
        VERIFIER = _verifier;
        PROOF_MODE = _proofMode;
    }
}

contract ProofVerifier_Verify_Tests is Test {
    TestHelpers helpers = new TestHelpers();
    ProofVerifierUnderTest verifier = new ProofVerifierUnderTest(helpers.mockVerifier(), ProofMode.FAKE);

    ExecutionCommitment commitment;

    function setUp() public {
        vm.roll(100); // have some historical blocks

        commitment = ExecutionCommitment(PROVER, SELECTOR, block.number - 1, blockhash(block.number - 1));
    }

    function test_verifySuccess() public view {
        (Proof memory proof, bytes32 journalHash) = helpers.createProof(commitment);
        verifier.verify(proof, journalHash, PROVER, SELECTOR);
    }

    function test_invalidProofMode() public {
        (Proof memory proof, bytes32 journalHash) = helpers.createProof(commitment);

        // Use Groth16 proof for Fake proof verifier
        proof.seal.mode = ProofMode.GROTH16;

        vm.expectRevert("Invalid proof mode");
        verifier.verify(proof, journalHash, PROVER, SELECTOR);
    }

    function test_invalidProver() public {
        commitment.proverContractAddress = address(0x0000000000000000000000000000000000deadbeef);
        (Proof memory proof, bytes32 journalHash) = helpers.createProof(commitment);

        vm.expectRevert("Invalid prover");
        verifier.verify(proof, journalHash, PROVER, SELECTOR);
    }

    function test_invalidSelector() public {
        commitment.functionSelector = 0xdeadbeef;
        (Proof memory proof, bytes32 journalHash) = helpers.createProof(commitment);

        vm.expectRevert("Invalid selector");
        verifier.verify(proof, journalHash, PROVER, SELECTOR);
    }

    function test_blockFromFuture() public {
        commitment.settleBlockNumber = block.number;
        (Proof memory proof, bytes32 journalHash) = helpers.createProof(commitment);

        vm.expectRevert("Invalid block number: block from future");
        verifier.verify(proof, journalHash, PROVER, SELECTOR);
    }

    function test_blockOlderThanLast256Blocks() public {
        vm.roll(block.number + 256); // forward block number
        (Proof memory proof, bytes32 journalHash) = helpers.createProof(commitment);

        vm.expectRevert("Invalid block number: block too old");
        verifier.verify(proof, journalHash, PROVER, SELECTOR);
    }

    function test_invalidBlockHash() public {
        commitment.settleBlockHash = blockhash(commitment.settleBlockNumber - 1);
        (Proof memory proof, bytes32 journalHash) = helpers.createProof(commitment);

        vm.expectRevert("Invalid block hash");
        verifier.verify(proof, journalHash, PROVER, SELECTOR);
    }
}
