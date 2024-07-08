// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Steel} from "vlayer-engine/Steel.sol";

import {Seal} from "./Seal.sol";

struct Proof {
    uint256 length;
    Seal seal;
    Steel.ExecutionCommitment commitment;
}
