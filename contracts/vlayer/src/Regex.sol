// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Address} from "@openzeppelin-contracts-5.0.1/utils/Address.sol";

import {Precompiles} from "./PrecompilesAddresses.sol";

library RegexLib {
    function matches(string memory source, string memory pattern) internal view returns (bool) {
        (bool success, bytes memory returnData) = Precompiles.REGEX_MATCH.staticcall(abi.encode([source, pattern]));
        Address.verifyCallResult(success, returnData);

        bool isMatch = abi.decode(returnData, (bool));
        return isMatch;
    }

    function capture(string memory source, string memory pattern) internal view returns (string[] memory) {
        (bool success, bytes memory returnData) = Precompiles.REGEX_CAPTURE.staticcall(abi.encode([source, pattern]));
        Address.verifyCallResult(success, returnData);

        return abi.decode(returnData, (string[]));
    }
}
