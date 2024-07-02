// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {IRiscZeroVerifier} from "risc0-ethereum/IRiscZeroVerifier.sol";
import {VlayerVerifiable} from "vlayer/VlayerVerifiable.sol";
import {Steel} from "vlayer-engine/Steel.sol";

contract Simple is VlayerVerifiable {

    uint256 public latestSum;

    constructor(IRiscZeroVerifier verifier) VlayerVerifiable(verifier) {

    }

    function updateSum(bytes calldata seal, Steel.ExecutionCommitment memory commitment, uint256 sum) public {
        _verify(seal, commitment, sum);

        latestSum = sum;
    }

}
