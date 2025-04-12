// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Proof} from "vlayer-0.1.0/Proof.sol";
import {Verifier} from "vlayer-0.1.0/Verifier.sol";

import {KrakenProver} from "./KrakenProver.sol";

contract KrakenVerifier is Verifier {
    address public prover;

    string public avgPrice;

    constructor(address _prover) {
        prover = _prover;
    }

    function verify(Proof calldata, string memory _avgPrice) public onlyVerified(prover, KrakenProver.main.selector) {
        avgPrice = _avgPrice;
    }
}
