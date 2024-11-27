// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Address} from "@openzeppelin-contracts-5.0.1/utils/Address.sol";

import {Precompiles} from "./PrecompilesAddresses.sol";

library URLPatternLib {
    function test(string memory source, string memory pattern) internal view returns (bool) {
        (bool success, bytes memory returnData) = Precompiles.URL_PATTERN_TEST.staticcall(abi.encode([source, pattern]));
        Address.verifyCallResult(success, returnData);

        bool isMatch = abi.decode(returnData, (bool));
        return isMatch;
    }
}
