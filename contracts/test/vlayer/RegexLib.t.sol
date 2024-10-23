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
}
