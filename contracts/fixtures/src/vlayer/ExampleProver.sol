// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Prover} from "vlayer/Prover.sol";
import {Web, WebProof, WebProofLib, WebLib} from "vlayer/WebProof.sol";

import "@openzeppelin-contracts-5.0.1/utils/Strings.sol";
/*
 * This contract is used in rust/server integration tests. The test fixture
 * (compiled contract) is placed in rust/server/testdata/ExampleProver.json.
 *
 * In order to update the test fixture:
 * 1. Modify this contract below.
 * 2. cd contracts/fixtures && forge build
 * 3. cp out/ExampleProver.sol/ExampleProver.json ../rust/services/call/server/testdata
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
        Web memory web = webProof.verify(
            "https://api.x.com/1.1/account/settings.json?include_ext_sharing_audiospaces_listening_data_with_followers=true&include_mention_filter=true&include_nsfw_user_flag=true&include_nsfw_admin_flag=true&include_ranked_timeline=true&include_alt_text_compose=true&ext=ssoConnections&include_country_code=true&include_ext_dm_nsfw_media_filter=true"
        );

        require(web.jsonGetString("screen_name").equal("jab68503"), "Invalid screen_name");

        return true;
    }
}
