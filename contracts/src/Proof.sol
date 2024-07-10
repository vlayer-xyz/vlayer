// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Vlayer} from "vlayer-engine/Vlayer.sol";

import {Seal} from "./Seal.sol";

struct Proof {
    uint256 length;
    Seal seal;
    Vlayer.ExecutionCommitment commitment;
}
