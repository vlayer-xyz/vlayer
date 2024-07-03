// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {IRiscZeroVerifier} from "risc0-ethereum/IRiscZeroVerifier.sol";
import {Steel} from "vlayer-engine/Steel.sol";

contract VlayerVerifiable {

    bytes32 public constant GUEST_ID = bytes32(0xb7079f57c71b4e1d95b8b1254303e13f78914599a8c119534c4c947c996b4d7d);

    IRiscZeroVerifier public _verifier;

    constructor(IRiscZeroVerifier verifier){
        _verifier = verifier;
    }
    
    function _verify(bytes calldata seal, Steel.ExecutionCommitment memory commitment, uint256 sum) internal virtual {
        bytes32 computedJournalHash = keccak256(abi.encode(commitment, sum));
        _verifier.verify(seal, GUEST_ID, computedJournalHash);
    }

}
