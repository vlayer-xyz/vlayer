// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {ExecutionCommitment} from "./ExecutionCommitment.sol";

import {Seal} from "./Seal.sol";

struct Proof {
    uint256 length;
    Seal seal;
    ExecutionCommitment commitment;
}
