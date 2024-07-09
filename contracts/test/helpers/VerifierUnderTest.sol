// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {IRiscZeroVerifier} from "risc0-ethereum/IRiscZeroVerifier.sol";
import {VerifierBase} from "../../src/Verifier.sol";

contract VerifierUnderTest is VerifierBase {
    function setVerifier(IRiscZeroVerifier _verifier) public {
        verifier = _verifier;
    }
}
