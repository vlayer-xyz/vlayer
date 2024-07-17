// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {VerifierBase} from "../../src/Verifier.sol";
import {IProofVerifier} from "../../src/proof_verifier/IProofVerifier.sol";

contract VerifierUnderTest is VerifierBase {
    function setVerifier(IProofVerifier _verifier) public {
        verifier = _verifier;
    }
}
