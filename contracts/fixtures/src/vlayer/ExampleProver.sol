// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Strings} from "@openzeppelin-contracts-5.0.1/utils/Strings.sol";
import {URLPatternLib} from "vlayer/URLPattern.sol";

import {Prover} from "vlayer/Prover.sol";
import {Web, WebProof, WebProofLib, WebLib} from "vlayer/WebProof.sol";

/*
 * This contract is used in rust/server integration tests. The test fixture
 * (compiled contract) is placed in rust/server/testdata/ExampleProver.json.
 *
 * In order to update the test fixture:
 * 1. Modify this contract below.
 * 2. cd contracts/fixtures && forge build
 * 3. cp out/ExampleProver.sol/ExampleProver.json ../../rust/services/call/server_lib/testdata
 */

contract ExampleProver is Prover {
    using Strings for string;
    using URLPatternLib for string;
    using WebProofLib for WebProof;
    using WebLib for Web;

    string private constant NOTARY_PUB_KEY =
        "-----BEGIN PUBLIC KEY-----\nMFYwEAYHKoZIzj0CAQYFK4EEAAoDQgAEe0jxnBObaIj7Xjg6TXLCM1GG/VhY5650\nOrS/jgcbBufo/QDfFvL/irzIv1JSmhGiVcsCHCwolhDXWcge7v2IsQ==\n-----END PUBLIC KEY-----\n";

    function sum(uint256 lhs, uint256 rhs) public pure returns (uint256) {
        return lhs + rhs;
    }

    // solhint-disable-next-line func-name-mixedcase
    function web_proof(WebProof calldata webProof) public view returns (bool) {
        Web memory web = webProof.recover();

        require(web.url.test("https://api.x.com/1.1/account/settings.json"), "Incorrect URL");

        require(NOTARY_PUB_KEY.equal(web.notaryPubKey), "Invalid notary public key");

        require(web.jsonGetString("screen_name").equal("g_p_vlayer"), "Invalid screen name");

        return true;
    }
}
