// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {IRiscZeroVerifier} from "risc0-ethereum/IRiscZeroVerifier.sol";

contract Simple {

    IRiscZeroVerifier verifier;
    uint256 public latestSum;

    constructor(IRiscZeroVerifier _verifier) {
        verifier = _verifier;
    }

    function updateSum(bytes32 proof, bytes calldata journal, uint256 sum) public {

        assert(proof == keccak256(journal));
        assert(uint(uint8(journal[0])) == sum);

        latestSum = sum;
    }

}
