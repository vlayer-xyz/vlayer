// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Test, console} from "forge-std-1.9.4/src/Test.sol";

import {IRiscZeroVerifier, Receipt, VerificationFailed} from "risc0-ethereum-2.0.0/src/IRiscZeroVerifier.sol";
import {RiscZeroMockVerifier} from "risc0-ethereum-2.0.0/src/test/RiscZeroMockVerifier.sol";

import {Repository} from "../../src/Repository.sol";
import {ProofVerifierBase} from "../../src/proof_verifier/ProofVerifierBase.sol";
import {CallAssumptions} from "../../src/CallAssumptions.sol";
import {ImageID} from "../../src/ImageID.sol";
import {Proof} from "../../src/Proof.sol";
import {ProofMode} from "../../src/Seal.sol";

import {TestHelpers, PROVER, SELECTOR} from "../helpers/TestHelpers.sol";

contract ProofVerifierUnderTest is ProofVerifierBase {
    constructor(IRiscZeroVerifier _verifier, ProofMode _proofMode)
        ProofVerifierBase(new Repository(address(this), address(this)))
    {
        VERIFIER = _verifier;
        PROOF_MODE = _proofMode;

        IMAGE_ID_REPOSITORY.addImageIdSupport(ImageID.RISC0_CALL_GUEST_ID);
    }
}

contract ProofVerifier_Verify_Tests is Test {
    TestHelpers helpers = new TestHelpers();
    ProofVerifierUnderTest verifier = new ProofVerifierUnderTest(helpers.mockVerifier(), ProofMode.FAKE);

    CallAssumptions assumptions;

    function setUp() public {
        vm.roll(100); // have some historical blocks

        assumptions = CallAssumptions(PROVER, SELECTOR, block.chainid, block.number - 1, blockhash(block.number - 1));
    }

    function test_verifySuccess() public view {
        (Proof memory proof, bytes32 journalHash) = helpers.createProof(assumptions);
        verifier.verify(proof, journalHash, PROVER, SELECTOR);
    }

    function test_invalidProofMode() public {
        (Proof memory proof, bytes32 journalHash) = helpers.createProof(assumptions);

        // Use Groth16 proof for Fake proof verifier
        proof.seal.mode = ProofMode.GROTH16;

        vm.expectRevert("Invalid proof mode");
        verifier.verify(proof, journalHash, PROVER, SELECTOR);
    }

    function test_invalidProver() public {
        assumptions.proverContractAddress = address(0x0000000000000000000000000000000000deadbeef);
        (Proof memory proof, bytes32 journalHash) = helpers.createProof(assumptions);

        vm.expectRevert("Invalid prover");
        verifier.verify(proof, journalHash, PROVER, SELECTOR);
    }

    function test_invalidSelector() public {
        assumptions.functionSelector = 0xdeadbeef;
        (Proof memory proof, bytes32 journalHash) = helpers.createProof(assumptions);

        vm.expectRevert("Invalid selector");
        verifier.verify(proof, journalHash, PROVER, SELECTOR);
    }

    function test_blockFromFuture() public {
        assumptions.settleBlockNumber = block.number;
        (Proof memory proof, bytes32 journalHash) = helpers.createProof(assumptions);

        vm.expectRevert("Invalid block number: block from future");
        verifier.verify(proof, journalHash, PROVER, SELECTOR);
    }

    function test_blockOlderThanLast256Blocks() public {
        vm.roll(block.number + 256); // forward block number
        (Proof memory proof, bytes32 journalHash) = helpers.createProof(assumptions);

        vm.expectRevert("Invalid block number: block too old");
        verifier.verify(proof, journalHash, PROVER, SELECTOR);
    }

    function test_invalidBlockHash() public {
        assumptions.settleBlockHash = blockhash(assumptions.settleBlockNumber - 1);
        (Proof memory proof, bytes32 journalHash) = helpers.createProof(assumptions);

        vm.expectRevert("Invalid block hash");
        verifier.verify(proof, journalHash, PROVER, SELECTOR);
    }

    function test_invalidChainId() public {
        assumptions.settleChainId = assumptions.settleChainId + 1;
        (Proof memory proof, bytes32 journalHash) = helpers.createProof(assumptions);

        vm.expectRevert("Invalid chain id");
        verifier.verify(proof, journalHash, PROVER, SELECTOR);
    }

    function test_invalidCallImageId() public {
        (Proof memory proof, bytes32 journalHash) = helpers.createProof(assumptions);

        uint256 fakeGuestId = uint256(proof.callGuestId) + 1;
        proof.callGuestId = bytes32(fakeGuestId);

        vm.expectRevert("Unsupported CallGuestId");
        verifier.verify(proof, journalHash, PROVER, SELECTOR);
    }
}
