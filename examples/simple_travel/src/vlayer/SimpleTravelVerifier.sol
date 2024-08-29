// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.23;

import {SimpleTravelProver} from "./SimpleTravelProver.sol";

import {Proof} from "vlayer/Proof.sol";
import {Verifier} from "vlayer/Verifier.sol";

contract SimpleTravel is Verifier {
    address public prover;

    constructor(address _prover) {
        prover = _prover;
    }

    function verify(
        Proof calldata,
        uint256 exampleReturn
    ) public onlyVerified(prover, SimpleTravelProver.aroundTheWorld.selector) {}
}
