// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {IRiscZeroVerifier} from "risc0-ethereum/IRiscZeroVerifier.sol";

contract Simple {

    IRiscZeroVerifier verifier;
    uint256 public latestSum;
    
    struct Commitment {
        uint16 offset;
        uint32 length;
        uint16 version;
        uint64 chainId;
        uint256 blockNumber; // Block number at which the commitment was made.
        bytes32 blockHash; // Hash of the block at the specified block number.
        bytes seal;
    }

    constructor(IRiscZeroVerifier _verifier) {
        verifier = _verifier;
    }

    function updateSum(Commitment memory commitment, uint256 sum, bytes32 journalHash) public {
        _verify(commitment, sum, journalHash);

        latestSum = sum;
    }

    function _verify(Commitment memory commitment, uint256 sum, bytes32 journalHash) private pure {

        bytes32 computedJournalHash = keccak256(abi.encode(commitment, sum));

        // assert(journalHash == computedJournalHash);
       
    }

}
