// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {IRiscZeroVerifier} from "risc0-ethereum/IRiscZeroVerifier.sol";
import {Steel} from "vlayer-engine/Steel.sol";

contract VlayerVerifiable {
    bytes32 public constant GUEST_ID = bytes32(0xb7079f57c71b4e1d95b8b1254303e13f78914599a8c119534c4c947c996b4d7d);

    IRiscZeroVerifier public _verifier;

    constructor(IRiscZeroVerifier verifier) {
        _verifier = verifier;
    }

    modifier onlyVerified() {
        _verify();
        _;
    }

    function _verify() internal virtual {
        require(false);
    }
}
