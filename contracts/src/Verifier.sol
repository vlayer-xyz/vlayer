// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {IRiscZeroVerifier} from "risc0-ethereum/IRiscZeroVerifier.sol";
import {Steel} from "vlayer-engine/Steel.sol";

contract Verifier {

    bytes32 public constant GUEST_ID = bytes32(0xb7079f57c71b4e1d95b8b1254303e13f78914599a8c119534c4c947c996b4d7d);

    uint256 public constant SELECTOR_LENGTH = 4; 

    IRiscZeroVerifier public _verifier;

    constructor(IRiscZeroVerifier verifier){
        _verifier = verifier;
    }

    modifier onlyVerified() {

        uint256 journal_offset = SELECTOR_LENGTH; 
        uint256 journal_length = 160; // TODO: make it dynamic
        uint256 journal_end = journal_offset + journal_length;

        uint256 seal_length_offset = SELECTOR_LENGTH + abi.decode(msg.data[journal_end:], (uint256));
        uint256 seal_offset = seal_length_offset + 32;
        uint256 seal_length = abi.decode(msg.data[seal_length_offset:], (uint256));
        uint256 seal_end = seal_offset + seal_length;

        bytes memory seal = msg.data[seal_offset:seal_end];        
        bytes32 computedJournalHash = keccak256(msg.data[journal_offset:journal_end]);


        _verifier.verify(seal, GUEST_ID, computedJournalHash);

        _;
    }
    
}
