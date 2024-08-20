// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {RiscZeroMockVerifier} from "risc0-ethereum/test/RiscZeroMockVerifier.sol";

import {ExecutionCommitment} from "../../src/ExecutionCommitment.sol";
import {Proof} from "../../src/Proof.sol";
import {ProofMode, Seal, SealLib} from "../../src/Seal.sol";
import {ImageID} from "../../src/ImageID.sol";

address constant PROVER = address(1);
bytes4 constant SELECTOR = bytes4(0x01020304);

contract TestHelpers {
    RiscZeroMockVerifier public immutable mockVerifier = new RiscZeroMockVerifier(bytes4(0));

    function createProof(ExecutionCommitment memory commitment, bytes memory journalParams)
        public
        view
        returns (Proof memory, bytes32)
    {
        bytes memory journal = bytes.concat(abi.encode(commitment), journalParams);
        bytes32 journalHash = sha256(journal);

        bytes memory seal = mockVerifier.mockProve(ImageID.RISC0_CALL_GUEST_ID, journalHash).seal;
        Proof memory proof = Proof(journal.length, encodeSeal(seal), commitment);
        return (proof, journalHash);
    }

    function createProof(ExecutionCommitment memory commitment) public view returns (Proof memory, bytes32) {
        bytes memory emptyBytes = new bytes(0);
        return createProof(commitment, emptyBytes);
    }

    function createProof() public view returns (Proof memory, bytes32) {
        ExecutionCommitment memory commitment =
            ExecutionCommitment(PROVER, SELECTOR, block.number - 1, blockhash(block.number - 1));
        bytes memory emptyBytes = new bytes(0);
        return createProof(commitment, emptyBytes);
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
        return Seal(words, proofMode);
    }

    function encodeWordsFake(bytes memory seal) private pure returns (bytes32[8] memory) {
        bytes32[8] memory words;

        require(seal.length == SealLib.FAKE_SEAL_LENGTH, "Invalid seal length");

        bytes32 firstWord = abi.decode(seal, (bytes32));
        uint32 secondWord = 0;

        for (uint256 i = 0; i < SealLib.FAKE_SEAL_LENGTH - 32; i++) {
            secondWord <<= 8;
            secondWord += uint8(seal[32 + i]);
        }

        words[0] = firstWord;
        words[1] = bytes32(bytes4(secondWord));

        return words;
    }
}
