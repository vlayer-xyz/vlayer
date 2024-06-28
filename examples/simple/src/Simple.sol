// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {IRiscZeroVerifier} from "risc0-ethereum/IRiscZeroVerifier.sol";
import {Steel} from "vlayer/Steel.sol";

contract Simple {

    IRiscZeroVerifier verifier;
    uint256 public latestSum;
    
    constructor(IRiscZeroVerifier _verifier) {
        verifier = _verifier;
    }

    function updateSum(Steel.ExecutionCommitment memory commitment, uint256 sum, bytes32 journalHash) public {
        _verify(commitment, sum, journalHash);

        latestSum = sum;
    }

    function _verify(Steel.ExecutionCommitment memory commitment, uint256 sum, bytes32 journalHash) private pure {

        bytes32 computedJournalHash = keccak256(abi.encode(commitment, sum));

        // assert(journalHash == computedJournalHash);
        assert(journalHash == computedJournalHash);
       
    }

}
