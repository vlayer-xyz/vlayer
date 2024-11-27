// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../../src/testing/VTest.sol";

import {URLPatternLib} from "../../src/UrlPattern.sol";

contract UrlPatternWrapper {
    function do_test(string memory source, string memory pattern) public view returns (bool) {
        return URLPatternLib.test(source, pattern);
    }
}

contract UrlPatternTest is VTest {
    function test_exact_match() public {
        UrlPatternWrapper urlPattern = new UrlPatternWrapper();
        callProver();
        bool isMatch = urlPattern.do_test("https://example.com", "https://example.com");
        assertTrue(isMatch);
    }

    function test_returns_false_when_not_matching() public {
        UrlPatternWrapper urlPattern = new UrlPatternWrapper();
        callProver();
        bool isMatch = urlPattern.do_test("https://elpmaxe.com", "https://example.com/");
        assertFalse(isMatch);
    }

    function test_reverts_when_invalid_pattern() public {
        UrlPatternWrapper urlPattern = new UrlPatternWrapper();
        callProver();
        try urlPattern.do_test("https://example.com", "[invalid pattern]") {
            revert("Did not revert as expected");
        } catch Error(string memory reason) {
            assertEq(reason, "Engine(TransactError(Revert(\"a relative input without a base URL is not valid\")))");
        }
    }

    function test_reverts_when_invalid_url() public {
        UrlPatternWrapper urlPattern = new UrlPatternWrapper();
        callProver();
        try urlPattern.do_test("invalid url", "https://example.com/") {
            revert("Did not revert as expected");
        } catch Error(string memory reason) {
            assertEq(reason, "Engine(TransactError(Revert(\"relative URL without a base\")))");
        }
    }
}
