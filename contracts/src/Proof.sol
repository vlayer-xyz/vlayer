// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {ExecutionCommitment, ExecutionCommitmentLib} from "./ExecutionCommitment.sol";

import {Seal, SealLib} from "./Seal.sol";

uint256 constant MAX_NUMBER_OF_DYNAMIC_PARAMS = 10;
uint256 constant WORD_SIZE = 32;

struct Proof {
    uint256 length;
    Seal seal;
    uint16 numberOfDynamicParams;
    // Using 10 instead of MAX_NUMBER_OF_DYNAMIC_PARAMS due to `sol!` macro not supporting generic expressions in array sizes.
    // TODO: Optimize space usage by packing values more efficiently
    uint16[10] dynamicParamsOffsets;
    ExecutionCommitment commitment;
}

library ProofLib {
    uint256 private constant LENGTH_LEN = WORD_SIZE;
    uint256 private constant NUMBER_OF_DYNAMIC_PARAMS_LEN = WORD_SIZE;
    uint256 private constant DYNAMIC_PARAMS_OFFSETS_LEN = MAX_NUMBER_OF_DYNAMIC_PARAMS * WORD_SIZE;

    uint256 public constant COMMITMENT_OFFSET =
        LENGTH_LEN + SealLib.SEAL_ENCODING_LENGTH + NUMBER_OF_DYNAMIC_PARAMS_LEN + DYNAMIC_PARAMS_OFFSETS_LEN;

    uint256 public constant PROOF_ENCODING_LENGTH = LENGTH_LEN + SealLib.SEAL_ENCODING_LENGTH
        + NUMBER_OF_DYNAMIC_PARAMS_LEN + DYNAMIC_PARAMS_OFFSETS_LEN
        + ExecutionCommitmentLib.EXECUTION_COMMITMENT_ENCODING_LENGTH;
}
