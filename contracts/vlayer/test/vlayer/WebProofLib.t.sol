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

    function test_verifiesWebProof() public {
        WebProof memory webProof = WebProof(vm.readFile("testdata/web_proof.json"));

        callProver();
        WebProofLibWrapper wrapper = new WebProofLibWrapper();
        Web memory web = wrapper.verify(webProof, DATA_URL);
        assertEq(bytes(web.body)[0], "{");
    }

    function test_incorrectUrl() public {
        WebProof memory webProof = WebProof(vm.readFile("testdata/web_proof.json"));

        callProver();

        WebProofLibWrapper wrapper = new WebProofLibWrapper();

        try wrapper.verify(webProof, "https://bad_api.x.com/1.1/account/settings.json") returns (Web memory) {
            revert("Expected error");
        } catch Error(string memory reason) {
            assertEq(reason, "Preflight(Engine(TransactError(Revert(\"revert: Incorrect URL\"))))");
        }
    }

    function test_missingPresentationJson() public {
        WebProof memory webProof = WebProof("{}");

        callProver();

        WebProofLibWrapper wrapper = new WebProofLibWrapper();

        try wrapper.verify(webProof, DATA_URL) returns (Web memory) {
            revert("Expected error");
        } catch Error(string memory reason) {
            assertEq(reason, "Preflight(Engine(TransactError(Revert(\"missing field `version` at line 1 column 2\"))))");
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
                "Preflight(Engine(Panic(\"called `Result::unwrap()` on an `Err` value: Custom(\\\"invalid length 64, expected an array at most 64 bytes long\\\")\")))"
            );
        }
    }
}
