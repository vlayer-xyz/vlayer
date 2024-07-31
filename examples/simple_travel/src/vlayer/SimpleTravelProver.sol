// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Prover} from "vlayer/Prover.sol";
import {Counter} from "./Counter.sol";

contract SimpleTravelProver is Prover {
    Counter counter;

    constructor(address _counter) {
        counter = Counter(_counter);
    }

    function aroundTheWorld() public returns (uint256) {
        // setBlock(block.number);
        return counter.count();
    }
}
