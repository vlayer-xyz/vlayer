// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Web} from "./WebProof.sol";
import {Precompiles} from "./PrecompilesAddresses.sol";

library JsonParserLib {
    function jsonGetString(Web memory web, string memory jsonPath) internal view returns (string memory) {
        require(bytes(web.body).length > 0, "Body is empty");

        bytes memory encodedParams = abi.encode([web.body, jsonPath]);
        (bool success, bytes memory returnData) = Precompiles.JSON_GET_STRING_PRECOMPILE.staticcall(encodedParams);
        require(success, "json_get_string precompile call failed");

        return abi.decode(returnData, (string));
    }

    function jsonGetInt(Web memory web, string memory jsonPath) internal view returns (int256) {
        require(bytes(web.body).length > 0, "Body is empty");

        bytes memory encodedParams = abi.encode([web.body, jsonPath]);
        (bool success, bytes memory returnData) = Precompiles.JSON_GET_INT_PRECOMPILE.staticcall(encodedParams);
        require(success, "json_get_string precompile call failed");

        return abi.decode(returnData, (int256));
    }

    function jsonGetBool(Web memory web, string memory jsonPath) internal view returns (bool) {
        require(bytes(web.body).length > 0, "Body is empty");

        bytes memory encodedParams = abi.encode([web.body, jsonPath]);
        (bool success, bytes memory returnData) = Precompiles.JSON_GET_BOOL_PRECOMPILE.staticcall(encodedParams);
        require(success, "json_get_string precompile call failed");

        return abi.decode(returnData, (bool));
    }

    function jsonGetArrayLength(Web memory web, string memory jsonPath) internal view returns (uint256) {
        require(bytes(web.body).length > 0, "Body is empty");

        bytes memory encodedParams = abi.encode([web.body, jsonPath]);
        (bool success, bytes memory returnData) = Precompiles.JSON_GET_ARRAY_LENGTH.staticcall(encodedParams);
        require(success, "json_get_array_length precompile call failed");

        return abi.decode(returnData, (uint256));
    }
}