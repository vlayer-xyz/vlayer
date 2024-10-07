// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

library RegexLib {
    address private constant REGEX_MATCH_PRECOMPILE = address(0x110);

    function matches(string memory source, string memory pattern) internal view returns (bool) {
        (bool success, bytes memory matchResult) = REGEX_MATCH_PRECOMPILE.staticcall(abi.encode([source, pattern]));
        require(success, "regex match precompile call failed");
        bool isMatch = abi.decode(matchResult, (bool));
        return isMatch;
    }
}
