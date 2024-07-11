// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Prover} from "vlayer/Prover.sol";

interface IExample {
    function exampleFunction() external returns (uint256);
}

contract SimpleTravelProver is Prover {
    address constant EXAMPLE_ADDR = 0x1111111111111111111111111111111111111111;
    constructor() {}

    function aroundTheWorld() public returns (uint256) {
        setBlock(1);
        return IExample(EXAMPLE_ADDR).exampleFunction();
    }
}
