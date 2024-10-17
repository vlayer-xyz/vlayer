// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {CallAssumptions, CallAssumptionsLib} from "./CallAssumptions.sol";

import {Seal, SealLib} from "./Seal.sol";

uint256 constant WORD_SIZE = 32;

struct Proof {
    uint256 length;
    Seal seal;
    CallAssumptions callAssumptions;
}

library ProofLib {
    uint256 private constant LENGTH_LEN = WORD_SIZE;

    uint256 public constant CALL_ASSUMPTIONS_OFFSET = LENGTH_LEN + SealLib.SEAL_ENCODING_LENGTH;

    uint256 public constant PROOF_ENCODING_LENGTH =
        LENGTH_LEN + SealLib.SEAL_ENCODING_LENGTH + CallAssumptionsLib.CALL_ASSUMPTIONS_ENCODING_LENGTH;

    function emptyProof() internal pure returns (Proof memory) {
        Proof memory proof;
        return proof;
    }
}
