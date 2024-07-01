// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {IRiscZeroVerifier} from "risc0-ethereum/IRiscZeroVerifier.sol";
import {Steel} from "vlayer/Steel.sol";

contract Simple {
    bytes32 public constant GUEST_ID = bytes32(0xb8d08f84d65bc7aadd17445d52f12be026dce5b26587534860b8a7660e8741b4);

    IRiscZeroVerifier verifier;
    uint256 public latestSum;

    constructor(IRiscZeroVerifier _verifier) {
        verifier = _verifier;
    }

    function updateSum(bytes calldata seal, Steel.ExecutionCommitment memory commitment, uint256 sum) public {
        _verify(seal, commitment, sum);

        latestSum = sum;
    }

    function _verify(bytes calldata seal, Steel.ExecutionCommitment memory commitment, uint256 sum) private view {
        bytes32 computedJournalHash = sha256(abi.encode(commitment, sum));

        verifier.verify(seal, GUEST_ID, computedJournalHash);
    }
}
