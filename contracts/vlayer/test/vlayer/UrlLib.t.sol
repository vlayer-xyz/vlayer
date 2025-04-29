// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../../src/testing/VTest.sol";

import {UrlLib} from "../../src/Url.sol";

contract UrlLibWrapper {
    function starts_with(string memory source, string memory prefix) public pure returns (bool) {
        return UrlLib.startsWith(source, prefix);
    }
}

contract UrlTest is VTest {
    function test_exact_match() public {
        UrlLibWrapper urlPattern = new UrlLibWrapper();
        assertTrue(urlPattern.starts_with("https://example.com", "https://example.com"));
    }

    function test_prefix() public {
        UrlLibWrapper urlPattern = new UrlLibWrapper();
        assertTrue(urlPattern.starts_with("https://example.com", "https://exam"));
    }

    function test_returns_false_when_not_matching() public {
        UrlLibWrapper urlPattern = new UrlLibWrapper();
        assertFalse(urlPattern.starts_with("https://example.com", "https://wrong.com/"));
    }
}
