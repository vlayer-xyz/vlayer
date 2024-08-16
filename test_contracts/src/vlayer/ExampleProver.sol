// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Prover} from "vlayer/Prover.sol";
import {WebProof} from "vlayer/WebProof.sol";

/*
 * This contract is used in rust/server integration tests. The test fixture
 * (compiled contract) is placed in rust/server/testdata/ExampleProver.json.
 *
 * In order to update the test fixture:
 * 1. Modify this contract below.
 * 2. cd test_contracts && forge build
 * 3. cp out/ExampleProver.sol/ExampleProver.json ../rust/call/server/testdata
 */

contract ExampleProver is Prover {
    constructor() {}

    function sum(uint256 lhs, uint256 rhs) public pure returns (uint256) {
        return lhs + rhs;
    }

    function web_proof(WebProof calldata webProof) public returns (bool) {
        // require(webProof.url.equal(""), "Incorrect URL");

        return true;
    }
}
