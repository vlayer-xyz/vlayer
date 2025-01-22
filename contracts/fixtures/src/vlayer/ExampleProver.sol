// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Strings} from "@openzeppelin-contracts-5.0.1/utils/Strings.sol";

import {Prover} from "vlayer/Prover.sol";
import {Web, WebProof, WebProofLib, WebLib} from "vlayer/WebProof.sol";

/*
 * This contract is used in rust/server integration tests. The test fixture
 * (compiled contract) is placed in rust/server/testdata/ExampleProver.json.
 *
 * In order to update the test fixture:
 * 1. Modify this contract below.
 * 2. cd contracts/fixtures && forge build
 * 3. cp out/ExampleProver.sol/ExampleProver.json ../../rust/services/call/server/testdata
 */

contract ExampleProver is Prover {
    using Strings for string;
    using WebProofLib for WebProof;
    using WebLib for Web;

    constructor() {}

    function sum(uint256 lhs, uint256 rhs) public pure returns (uint256) {
        return lhs + rhs;
    }

    function web_proof(WebProof calldata webProof) public view returns (bool) {
        Web memory web = webProof.verify("https://api.x.com/1.1/account/settings.json");

        require(web.jsonGetString("screen_name").equal("wktr0"), "Invalid screen name");

        return true;
    }
}
