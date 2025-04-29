// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Address} from "@openzeppelin-contracts-5.0.1/utils/Address.sol";

import {Precompiles} from "./PrecompilesAddresses.sol";

library UrlLib {
    function startsWith(string memory source, string memory prefix) internal pure returns (bool) {
        bytes memory sourceBytes = bytes(source);
        bytes memory prefixBytes = bytes(prefix);

        if (sourceBytes.length < prefixBytes.length) {
            return false;
        }

        for (uint256 i = 0; i < prefixBytes.length; i++) {
            if (sourceBytes[i] != prefixBytes[i]) {
                return false;
            }
        }

        return true;
    }
}
