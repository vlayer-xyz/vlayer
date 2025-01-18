// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Proof} from "vlayer-0.1.0/Proof.sol";
import {Prover} from "vlayer-0.1.0/Prover.sol";

contract NoopProver is Prover {
    constructor() {}

    function noop() public returns (Proof memory) {
        return (proof());
    }
}
