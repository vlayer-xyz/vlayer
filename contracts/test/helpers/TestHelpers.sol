// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {RiscZeroMockVerifier} from "risc0-ethereum/test/RiscZeroMockVerifier.sol";

import {ExecutionCommitment} from "../../src/ExecutionCommitment.sol";
import {Proof} from "../../src/Proof.sol";
import {ProofMode, Seal, SealLib} from "../../src/Seal.sol";
import {GUEST_ID} from "../../src/GuestID.sol";

address constant PROVER = address(1);
bytes4 constant SELECTOR = bytes4(0x01020304);

contract TestHelpers {
    RiscZeroMockVerifier public immutable mockVerifier = new RiscZeroMockVerifier(bytes4(0));

    function createProof(ExecutionCommitment memory commitment, bytes memory journalParams)
        public
        view
        returns (Proof memory, bytes32)
    {
        bytes memory journal = concat(abi.encode(commitment), journalParams);
        bytes32 journalHash = sha256(journal);

        bytes memory seal = mockVerifier.mockProve(GUEST_ID, journalHash).seal;
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
        require(seal.length == SealLib.SEAL_LENGTH, "Invalid seal length");

        uint256 lhv = 0;
        uint256 rhv = 0;

        for (uint256 i = 0; i < SealLib.SEAL_MIDDLE; i++) {
            lhv <<= 8;
            lhv += uint8(seal[i]);
            rhv <<= 8;
            rhv += uint8(seal[i + SealLib.SEAL_MIDDLE]);
        }

        // set ProofMode to FAKE
        rhv <<= 8;
        rhv += uint8(proofMode);

        // shift value to most significant bytes
        lhv <<= 8 * (32 - SealLib.SEAL_MIDDLE);
        rhv <<= 8 * (32 - SealLib.SEAL_MIDDLE - 1);

        return Seal(bytes18(bytes32(lhv)), bytes19(bytes32(rhv)));
    }

    function concat(bytes memory a, bytes memory b) public pure returns (bytes memory) {
        bytes memory c = new bytes(a.length + b.length);

        for (uint256 i = 0; i < a.length; i++) {
            c[i] = a[i];
        }

        uint256 offset = a.length;
        for (uint256 i = 0; i < b.length; i++) {
            c[offset + i] = b[i];
        }

        return c;
    }
}
