// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {ExecutionCommitment} from "vlayer-engine/contracts/Vlayer.sol";

import {Seal} from "./Seal.sol";

struct Proof {
    uint256 length;
    Seal seal;
    ExecutionCommitment commitment;
}
