// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Prover} from "vlayer/Prover.sol";
import {WebProof, WebProofLib} from "vlayer/WebProof.sol";
import "openzeppelin/contracts/utils/Strings.sol";

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
    using Strings for string;
    using WebProofLib for WebProof;

    constructor() {}

    function sum(uint256 lhs, uint256 rhs) public pure returns (uint256) {
        return lhs + rhs;
    }

    function web_proof(WebProof calldata webProof) public returns (bool) {
        require(webProof.url().equal("api.x.com"), "Incorrect URL");

        return true;
    }
}
