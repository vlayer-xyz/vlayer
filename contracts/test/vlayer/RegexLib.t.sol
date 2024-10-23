// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../../src/testing/VTest.sol";
import {RegexLib} from "../../src/Regex.sol";

contract RegexWrapper {
    function matches(string memory source, string memory pattern) public view returns (bool) {
        return RegexLib.matches(source, pattern);
    }

    function capture(string memory source, string memory pattern) public view returns (string[] memory) {
        return RegexLib.capture(source, pattern);
    }
}

contract RegexTest is VTest {
    function test_matches() public {
        RegexWrapper regex = new RegexWrapper();
        callProver();
        bool isMatch = regex.matches("hello world", "hello");
        assertTrue(isMatch);
    }

    function test_capture() public {
        RegexWrapper regex = new RegexWrapper();
        callProver();
        string[] memory captures = regex.capture("hello world", "(.+) (.+)");
        assertEq(captures.length, 3);
        assertEq(captures[0], "hello world");
        assertEq(captures[1], "hello");
        assertEq(captures[2], "world");
    }

    function test_capture_returns_empty_list_when_no_match() public {
        RegexWrapper regex = new RegexWrapper();
        callProver();
        string[] memory captures = regex.capture("hello world", "goodbye world");
        assertEq(captures.length, 0);
    }

    function test_capture_returns_empty_string_when_no_match_for_group() public {
        RegexWrapper regex = new RegexWrapper();
        callProver();
        string[] memory captures = regex.capture("hello world", "hello(,)? (world)");
        assertEq(captures.length, 3);
        assertEq(captures[0], "hello world");
        assertEq(captures[1], "");
        assertEq(captures[2], "world");
    }
}
