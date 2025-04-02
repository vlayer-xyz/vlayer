// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {console} from "forge-std-1.9.4/src/console.sol";

import {RiscZeroMockVerifier} from "risc0-ethereum-2.0.0/src/test/RiscZeroMockVerifier.sol";

import {CallAssumptions} from "../../src/CallAssumptions.sol";
import {Proof, ProofLib} from "../../src/Proof.sol";
import {ProofMode, Seal, SealLib} from "../../src/Seal.sol";
import {ImageID} from "../../src/ImageID.sol";
import {ProofVerifierFactory, IProofVerifier} from "../../src/proof_verifier/ProofVerifierFactory.sol";

import {FAKE_VERIFIER_SELECTOR} from "../../src/proof_verifier/FakeProofVerifier.sol";

address constant PROVER = address(1);
bytes4 constant SELECTOR = bytes4(0x01020304);

contract TestHelpers {
    RiscZeroMockVerifier public immutable mockVerifier = new RiscZeroMockVerifier(FAKE_VERIFIER_SELECTOR);

    uint256 private constant LENGTH_ABI_FIELD_LEN = 0x20;

    function createProof(CallAssumptions memory assumptions, bool journalBoolParam)
        public
        view
        returns (Proof memory, bytes32)
    {
        return createProof(assumptions, abi.encode(ProofLib.emptyProof(), journalBoolParam));
    }

    function createProof(CallAssumptions memory assumptions, string memory journalStringParam)
        public
        view
        returns (Proof memory, bytes32)
    {
        return createProof(assumptions, abi.encode(ProofLib.emptyProof(), journalStringParam));
    }

    function createProof(CallAssumptions memory assumptions) public view returns (Proof memory, bytes32) {
        return createProof(assumptions, abi.encode(ProofLib.emptyProof()));
    }

    function createProof() public view returns (Proof memory, bytes32) {
        CallAssumptions memory assumptions =
            CallAssumptions(PROVER, SELECTOR, block.chainid, block.number - 1, blockhash(block.number - 1));
        return createProof(assumptions);
    }

    function createProof(CallAssumptions memory assumptions, bytes memory journalParamsWithProofPrefix)
        public
        view
        returns (Proof memory, bytes32)
    {
        bytes memory journal = bytes.concat(abi.encode(assumptions), journalParamsWithProofPrefix);
        bytes32 journalHash = sha256(journal);

        bytes memory seal = mockVerifier.mockProve(ImageID.RISC0_CALL_GUEST_ID, journalHash).seal;
        Proof memory proof = Proof(encodeSeal(seal), ImageID.RISC0_CALL_GUEST_ID, journal.length, assumptions);

        return (proof, journalHash);
    }

    function setSealProofMode(Seal memory seal, ProofMode proofMode) public pure returns (Seal memory) {
        return encodeSeal(SealLib.decode(seal), proofMode);
    }

    function encodeSeal(bytes memory seal) public pure returns (Seal memory) {
        return encodeSeal(seal, ProofMode.FAKE);
    }

    function encodeSeal(bytes memory seal, ProofMode proofMode) public pure returns (Seal memory) {
        bytes32[8] memory words;
        if (proofMode == ProofMode.FAKE) {
            words = encodeWordsFake(seal);
        }
        return Seal(FAKE_VERIFIER_SELECTOR, words, proofMode);
    }

    function encodeWordsFake(bytes memory seal) private pure returns (bytes32[8] memory) {
        bytes32[8] memory words;
        uint256 rawSeal;

        require(seal.length == SealLib.FAKE_SEAL_LENGTH, "Invalid seal length");

        for (uint256 i = SealLib.VERIFIER_SELECTOR_LENGTH - 1; i < seal.length; ++i) {
            rawSeal <<= 8;
            rawSeal += uint8(seal[i]);
        }

        words[0] = bytes32(rawSeal);

        return words;
    }

    function produceProofVerifier() public returns (IProofVerifier) {
        return ProofVerifierFactory.produce();
    }
}
