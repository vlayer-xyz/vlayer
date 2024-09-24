// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {VTest} from "../../src/testing/VTest.sol";
import {Web, WebProof, WebProofLib, WebLib} from "../../src/WebProof.sol";

contract WebProofLibWrapper {
    using WebProofLib for WebProof;

    function verify(WebProof calldata webProof) public view returns (Web memory) {
        return webProof.verify(
            "https://api.x.com/1.1/account/settings.json?include_ext_sharing_audiospaces_listening_data_with_followers=true&include_mention_filter=true&include_nsfw_user_flag=true&include_nsfw_admin_flag=true&include_ranked_timeline=true&include_alt_text_compose=true&ext=ssoConnections&include_country_code=true&include_ext_dm_nsfw_media_filter=true"
        );
    }
}

contract EmailProverTest is VTest {
    using WebProofLib for WebProof;

    function test_verifiesWebProof() public {
        WebProofLibWrapper wrapper = new WebProofLibWrapper();
        WebProof memory webProof = WebProof(vm.readFile("testdata/web_proof.json"));

        callProver();
        Web memory web = wrapper.verify(webProof);

        assertEq(bytes(web.body)[0], "{");
    }
}
