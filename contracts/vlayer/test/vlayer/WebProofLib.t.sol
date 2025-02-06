// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

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

    string public constant DATA_URL = "https://api.x.com/1.1/*/settings.json";

    function test_revertsIf_notaryKeyIsInvalid() public {
        WebProof memory webProof = WebProof(vm.readFile("testdata/web_proof_invalid_notary_pub_key.json"));
        WebProofLibWrapper wrapper = new WebProofLibWrapper();
        vm.expectRevert(
            abi.encodeWithSelector(
                WebProofLib.InvalidHardcodedNotaryPubKey.selector,
                "-----BEGIN PUBLIC KEY-----\nMFYwEAYHKoZIzj0CAQYFK4EEAAoDQgAEZT9nJiwhGESLjwQNnZ2MsZ1xwjGzvmhF\nxFi8Vjzanlidbsc1ngM+s1nzlRkZI5UK9BngzmC27BO0qXxPSepIwQ==\n-----END PUBLIC KEY-----\n"
            )
        );
        wrapper.verify(webProof, DATA_URL);
    }

    function test_verifiesWebProof() public {
        WebProof memory webProof = WebProof(vm.readFile("testdata/web_proof.json"));

        callProver();

        WebProofLibWrapper wrapper = new WebProofLibWrapper();

        Web memory web = wrapper.verify(webProof, DATA_URL);
        assertEq(bytes(web.body)[0], "{");
    }

    function test_incorrectUrl() public {
        WebProof memory webProof = WebProof(vm.readFile("testdata/web_proof.json"));

        WebProofLibWrapper wrapper = new WebProofLibWrapper();

        vm.expectRevert(
            abi.encodeWithSelector(
                WebProofLib.IncorrectUrl.selector,
                "https://bad_api.x.com/1.1/account/settings.json"
            )
        );
        wrapper.verify(webProof, "https://bad_api.x.com/1.1/account/settings.json");
    }

    function test_missingPresentationJson() public {
        WebProof memory webProof = WebProof("{}");

        callProver();

        WebProofLibWrapper wrapper = new WebProofLibWrapper();

        try wrapper.verify(webProof, DATA_URL) returns (Web memory) {
            revert("Expected error");
        } catch Error(string memory reason) {
            assertEq(
                reason,
                "Preflight(Engine(TransactError(Revert(\"missing field `presentationJson` at line 1 column 2\"))))"
            );
        }
    }

    function test_missingPartInSerializedWebProof() public {
        WebProof memory webProof = WebProof(vm.readFile("testdata/web_proof_missing_part.json"));

        callProver();

        WebProofLibWrapper wrapper = new WebProofLibWrapper();

        try wrapper.verify(webProof, DATA_URL) returns (Web memory) {
            revert("Expected error");
        } catch Error(string memory reason) {
            assertEq(
                reason,
                "Preflight(Engine(TransactError(Revert(\"Verification error: Deserialization error: Bincode deserialize error: invalid length 64, expected an array at most 64 bytes long\"))))"
            );
        }
    }
}
