// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {VTest} from "../../src/testing/VTest.sol";
import {Web, WebProof, WebProofLib, WebLib} from "../../src/WebProof.sol";
import {Strings} from "@openzeppelin-contracts-5.0.1/utils/Strings.sol";

contract JsonParsingTest is VTest {
    using Strings for string;
    using WebLib for Web;

    function test_parsingStringFromSimpleJson() public {
        Web memory web = Web("{\"asset\":\"FDUSD\",\"test\":5}", "", "");

        callProver();
        string memory assetName = web.jsonGetString("asset");

        assert(assetName.equal("FDUSD"));
    }

    function test_parsingIntFromSimpleJson() public {
        Web memory web = Web("{\"asset\":\"FDUSD\",\"test\":5}", "", "");

        callProver();
        int256 value = web.jsonGetInt("test");

        assertEq(value, 5);
    }

    function test_parsingBoolFromSimpleJson() public {
        Web memory web = Web("{\"asset\":\"FDUSD\",\"test\":true}", "", "");

        callProver();
        bool value = web.jsonGetBool("test");

        assertTrue(value);
    }

    function test_parsingStringFromArray() public {
        Web memory web = Web("{\"asset\":[\"FDUSD\",\"test\"]}", "", "");

        callProver();
        string memory assetName = web.jsonGetString("asset[0]");

        assertEq(keccak256(bytes(assetName)), keccak256(bytes("FDUSD")));
    }

    function test_parsingJsonArrayOfObjects() public {
        Web memory web = Web(
            "[ { \"asset\": \"FDUSD\", \"free\": \"0.10620008\", \"locked\": \"0\", \"freeze\": \"0\", \"withdrawing\": \"0\", \"ipoable\": \"0\", \"btcValuation\": \"0\" }, { \"asset\": \"MOVR\", \"free\": \"0.0649415\", \"locked\": \"0\", \"freeze\": \"0\", \"withdrawing\": \"0\", \"ipoable\": \"0\", \"btcValuation\": \"0\" }, { \"asset\": \"PYR\", \"free\": \"0.9991\", \"locked\": \"0\", \"freeze\": \"0\", \"withdrawing\": \"0\", \"ipoable\": \"0\", \"btcValuation\": \"0\" }, { \"asset\": \"USDC\", \"free\": \"15.00047635\", \"locked\": \"0\", \"freeze\": \"0\", \"withdrawing\": \"0\", \"ipoable\": \"0\", \"btcValuation\": \"0\" } ]",
            "",
            ""
        );

        callProver();
        string memory assetName = web.jsonGetString("[0].asset");

        assertEq(keccak256(bytes(assetName)), keccak256(bytes("FDUSD")));
    }
}
