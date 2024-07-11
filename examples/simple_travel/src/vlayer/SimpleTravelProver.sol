// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Prover} from "vlayer/Prover.sol";

contract SimpleTravelProver is Prover {
    address constant EXAMPLE_ADDR = 0x742d35Cc6634C0532925a3b844Bc454e4438f44e;
    constructor() {}

    function aroundTheWorld() public returns (uint256) {
        setBlock(13);

        (bool success, bytes memory output_bytes) = EXAMPLE_ADDR.call("");
        // TODO: uncomment when we successfully intercept the call:
        // require(success, "Call to example address should be intercepted and successful");
        return abi.decode(output_bytes, (uint256));
    }
}
