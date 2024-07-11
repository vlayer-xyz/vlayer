// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {IRiscZeroVerifier} from "risc0-ethereum/IRiscZeroVerifier.sol";
import {Verifier} from "vlayer/Verifier.sol";
import {ExecutionCommitment} from "vlayer/ExecutionCommitment.sol";

import {SimpleProver} from "./SimpleProver.sol";

contract Simple is Verifier {
    address public prover;
    uint256 public latestSum;

    constructor(address _prover) {
        prover = _prover;
    }

    function updateSum(bytes calldata seal, ExecutionCommitment memory commitment, uint256 sum)
        public
        onlyVerified(prover, SimpleProver.sum.selector)
    {
        latestSum = sum;
    }
}
