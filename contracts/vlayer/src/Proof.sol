// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {CallAssumptions, CallAssumptionsLib} from "./CallAssumptions.sol";

import {Seal, SealLib} from "./Seal.sol";

uint256 constant WORD_SIZE = 32;

struct Proof {
    Seal seal;
    bytes32 callGuestId;
    uint256 length;
    CallAssumptions callAssumptions;
}

library ProofLib {
    uint256 private constant LENGTH_LEN = WORD_SIZE;
    uint256 private constant CALL_GUEST_ID_LEN = WORD_SIZE;

    uint256 public constant CALL_ASSUMPTIONS_OFFSET = SealLib.SEAL_ENCODING_LENGTH + CALL_GUEST_ID_LEN + LENGTH_LEN;

    uint256 public constant PROOF_ENCODING_LENGTH = SealLib.SEAL_ENCODING_LENGTH + CALL_GUEST_ID_LEN + LENGTH_LEN
        + CallAssumptionsLib.CALL_ASSUMPTIONS_ENCODING_LENGTH;

    function emptyProof() internal pure returns (Proof memory) {
        Proof memory proof;
        return proof;
    }
}
