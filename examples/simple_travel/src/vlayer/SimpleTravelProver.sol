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
        
        (bool success, bytes memory result) = address(EXAMPLE_ADDR).call(abi.encodeWithSelector(IExample.exampleFunction.selector));
        require(success, "Call to example address should get intercepted and succeed");
        
        return abi.decode(result, (uint256));
    }
}
