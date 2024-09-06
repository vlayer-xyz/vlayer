// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {WebProofProver} from "./WebProofProver.sol";

import {Proof} from "vlayer-contracts-0.1.0/src/Proof.sol";
import {Verifier} from "vlayer-contracts-0.1.0/src/Verifier.sol";

contract WebProofVerifier is Verifier {
    address public prover;

    constructor(address _prover) {
        prover = _prover;
    }

    function verify(
        Proof calldata,
        bool
    ) public onlyVerified(prover, WebProofProver.main.selector) {}
}
