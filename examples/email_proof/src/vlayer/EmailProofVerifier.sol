// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {EmailProver} from "./EmailProver.sol";

import {Proof} from "vlayer-0.1.0/src/Proof.sol";
import {Verifier} from "vlayer-0.1.0/src/Verifier.sol";

contract EmailProofVerifier is Verifier {
    address public prover;

    constructor(address _prover) {
        prover = _prover;
    }

    function verify(Proof calldata, bool) public onlyVerified(prover, EmailProver.main.selector) {}
}
