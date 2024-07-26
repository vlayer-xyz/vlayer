// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {WebProofProver} from "./WebProofProver.sol";
import {Verifier} from "vlayer/Verifier.sol";

contract WebProofVerifier is Verifier {
    address public prover;

    constructor(address _prover) {
        prover = _prover;
    }

    function verify()
        public
        onlyVerified(prover, WebProofProver.main.selector)
    {}
}
