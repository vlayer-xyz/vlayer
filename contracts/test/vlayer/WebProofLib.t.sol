// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {VTest} from "../../src/testing/VTest.sol";
import {Web, WebProof, WebProofLib} from "../../src/WebProof.sol";

contract WebProofLibWrapper {
    using WebProofLib for WebProof;

    function verify(WebProof calldata webProof, string memory dataUrl) public view returns (Web memory) {
        return webProof.verify(dataUrl);
    }
}

contract WebProverTest is VTest {
    using WebProofLib for WebProof;

    string public constant DATA_URL =
        "https://api.x.com/1.1/account/settings.json?include_ext_sharing_audiospaces_listening_data_with_followers=true&include_mention_filter=true&include_nsfw_user_flag=true&include_nsfw_admin_flag=true&include_ranked_timeline=true&include_alt_text_compose=true&ext=ssoConnections&include_country_code=true&include_ext_dm_nsfw_media_filter=true";

    function test_verifiesWebProof() public {
        WebProof memory webProof = WebProof(vm.readFile("testdata/web_proof.json"));

        callProver();
        Web memory web = webProof.verify(DATA_URL);

        assertEq(bytes(web.body)[0], "{");
    }

    function test_incorrectUrl() public {
        WebProof memory webProof = WebProof(vm.readFile("testdata/web_proof.json"));

        callProver();

        WebProofLibWrapper wrapper = new WebProofLibWrapper();

        try wrapper.verify(webProof, "") returns (Web memory web) {
            revert("Expected error");
        } catch Error(string memory reason) {
            assertEq(reason, "Engine(TransactError(\"revert: Incorrect URL\"))");
        }
    }

    function test_missingNotaryPubKey() public {
        WebProof memory webProof = WebProof("{}");

        callProver();

        WebProofLibWrapper wrapper = new WebProofLibWrapper();

        try wrapper.verify(webProof, DATA_URL) returns (Web memory web) {
            revert("Expected error");
        } catch Error(string memory reason) {
            assertEq(reason, "Engine(TransactError(\"missing field `notary_pub_key` at line 1 column 2\"))");
        }
    }

    function test_missingSignature() public {
        WebProof memory webProof = WebProof(vm.readFile("testdata/web_proof_missing_signature.json"));

        callProver();

        WebProofLibWrapper wrapper = new WebProofLibWrapper();

        try wrapper.verify(webProof, DATA_URL) returns (Web memory web) {
            revert("Expected error");
        } catch Error(string memory reason) {
            assertEq(
                reason,
                "Engine(TransactError(\"Verification error: Session proof error: session proof is missing notary signature\"))"
            );
        }
    }

    function test_invalidNotaryPubKey() public {
        WebProof memory webProof = WebProof(vm.readFile("testdata/web_proof_invalid_notary_pub_key.json"));

        callProver();

        WebProofLibWrapper wrapper = new WebProofLibWrapper();

        try wrapper.verify(webProof, DATA_URL) returns (Web memory web) {
            revert("Expected error");
        } catch Error(string memory reason) {
            assertEq(
                reason,
                "Engine(TransactError(\"unknown/unsupported algorithm OID: 1.2.840.10045.2.1 at line 8444 column 1\"))"
            );
        }
    }
}
