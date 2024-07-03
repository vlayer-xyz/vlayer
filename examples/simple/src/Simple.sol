// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {IRiscZeroVerifier} from "risc0-ethereum/IRiscZeroVerifier.sol";
import {Verifier} from "vlayer/Verifier.sol";
import {Steel} from "vlayer-engine/Steel.sol";

contract Simple is Verifier {

    uint256 public latestSum;

    constructor(IRiscZeroVerifier verifier) Verifier(verifier) {
    }

    function updateSum(bytes calldata seal, Steel.ExecutionCommitment calldata commitment, uint256 sum) public  onlyVerified(seal, commitment, sum) {
        latestSum = sum;
    }

}
