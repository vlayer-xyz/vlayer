// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {IRiscZeroVerifier} from "risc0-ethereum/IRiscZeroVerifier.sol";
import {Verifier} from "vlayer/Verifier.sol";
import {Steel} from "vlayer-engine/Steel.sol";

contract Simple is Verifier {

    uint256 public latestSum;

    constructor(IRiscZeroVerifier verifier) Verifier(verifier) {
    }

    function updateSum(Steel.ExecutionCommitment calldata, uint256 sum, bytes calldata) public  onlyVerified() {
        latestSum = sum;
    }

}
