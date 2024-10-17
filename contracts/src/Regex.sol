// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Precompiles} from "./PrecompilesAddresses.sol";

library RegexLib {
    function matches(string memory source, string memory pattern) internal view returns (bool) {
        (bool success, bytes memory matchResult) = Precompiles.REGEX_MATCH_PRECOMPILE.staticcall(abi.encode([source, pattern]));
        require(success, "regex match precompile call failed");
        bool isMatch = abi.decode(matchResult, (bool));
        return isMatch;
    }
}
