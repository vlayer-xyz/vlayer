// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {ExecutionCommitment} from "./ExecutionCommitment.sol";

import {Seal} from "./Seal.sol";

struct Proof {
    uint256 length;
    Seal seal;
    ExecutionCommitment commitment;
}

library ProofLib {
    uint256 constant LENGTH_LEN = 32;
    uint256 constant SEAL_LEN = 256 + 32;

    uint256 public constant LENGTH_OFFSET = 0;
    uint256 public constant SEAL_OFFSET = LENGTH_LEN;
    uint256 public constant COMMITMENT_OFFSET = SEAL_OFFSET + SEAL_LEN;
}
