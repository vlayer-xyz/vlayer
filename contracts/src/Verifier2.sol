// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;


import {IProofVerifier} from "./proof_verifier/IProofVerifier.sol";
import {ProofVerifierFactory} from "./proof_verifier/ProofVerifierFactory.sol";
import {ExecutionCommitmentLib, ExecutionCommitment} from "./ExecutionCommitment.sol";
import {SealLib, Seal} from "./Seal.sol";

struct Proof2 {
    // How many args are returned from prover (and included in journal)
    uint256 argsCount;
    // Total length of dynamic args included into proof
    uint256 dynamicLength;
    // i-th bit = 1 <=> i-th arg is dynamic
    uint256 dynamicArgsMap;
    // We can even pack first 3 args into single word if we really want to optimize
    Seal seal;
    ExecutionCommitment commitment;
}

uint256 constant WORD_SIZE = 32;
uint256 constant COMMITMENT_OFFSET = WORD_SIZE * 3 + SealLib.SEAL_ENCODING_LENGTH;

contract Verifier2 {
    uint256 private constant SELECTOR_LEN = 4;
    uint256 private constant PROOF_OFFSET = SELECTOR_LEN;
    uint256 public constant JOURNAL_OFFSET = PROOF_OFFSET + COMMITMENT_OFFSET;
    uint256 private constant STATIC_PARAMS_OFFSET = JOURNAL_OFFSET + ExecutionCommitmentLib.EXECUTION_COMMITMENT_ENCODING_LENGTH;

    function formatCalldata() external pure returns (bytes memory) {
        Proof2 memory proof = abi.decode(msg.data[PROOF_OFFSET:], (Proof2));

        bytes memory commitment = msg.data[JOURNAL_OFFSET:JOURNAL_OFFSET + ExecutionCommitmentLib.EXECUTION_COMMITMENT_ENCODING_LENGTH];
        (bytes memory _staticParams, uint dynamicStart) = staticParams(proof.argsCount, proof.dynamicArgsMap);
        bytes memory dynamicData = msg.data[dynamicStart:dynamicStart + proof.dynamicLength];

        return bytes.concat(commitment, _staticParams, dynamicData);
    }

    // We know that static args (and pointers to dynamic args) are stored exactly after proof and take (argsCount * WORD_SIZE) bytes
    // Dynamic args have filled all rest of data included into journal
    // Here the same patch of dynamic data should be somewhere inside calldata
    function staticParams(uint argsCount, uint dynamicArgsMap) internal pure returns (bytes memory _staticParams, uint256 dynamicParamsStart) {
        uint dynamicOffsetChange;
        for (uint i = 0; i < argsCount; i++) {
            bool isDynamic = (dynamicArgsMap >> i) & 1 == 1;
            uint slotStart = STATIC_PARAMS_OFFSET + i * WORD_SIZE;
            uint slotEnd = STATIC_PARAMS_OFFSET + (i + 1) * WORD_SIZE;
            bytes memory slot = msg.data[slotStart:slotEnd];
            // If static, don't modify anything
            if (!isDynamic) {
                _staticParams = bytes.concat(_staticParams, slot);
                continue;
            }
            uint offset = abi.decode(slot, (uint256));
            if (dynamicParamsStart == 0) {
                // On first dynamic, remember the offset where dynamic params start in calldata (we know the length from proof)
                dynamicParamsStart = offset + 4;
                // Also remember the difference between pointers inside proof and pointers inside calldata
                dynamicOffsetChange = offset - argsCount * WORD_SIZE;
            }
            offset -= dynamicOffsetChange;
            _staticParams = bytes.concat(_staticParams, abi.encodePacked(offset));
        }
    }
}
