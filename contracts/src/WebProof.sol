// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Strings} from "openzeppelin/contracts/utils/Strings.sol";

struct WebProof {
    string webProofJson;
    string body;
}

library WebProofLib {
    using Strings for string;

    address private constant VERIFY_AND_PARSE_PRECOMPILE = address(0x100);
    address private constant JSON_GET_STRING_PRECOMPILE = address(0x101);

    function verify(
        WebProof memory webProof,
        string memory dataUrl
    ) internal view returns (WebProof memory) {
        (bool success, bytes memory returnData) = VERIFY_AND_PARSE_PRECOMPILE
            .staticcall(bytes(webProof.webProofJson));

        string[3] memory data = abi.decode(returnData, (string[3]));
        string memory serverName = "api.x.com";
        webProof.body = data[2];

        require(success, "verify_and_parse precompile call failed");
        require(dataUrl.equal(data[0]), "Incorrect URL");
        require(serverName.equal(data[1]), "Server name not found");

        return webProof;
    }

    function jsonGetString(
        WebProof memory webProof,
        string memory jsonPath
    ) internal view returns (string memory) {
        require(bytes(webProof.body).length > 0, "Body is empty");

        bytes memory encodedParams = abi.encode([webProof.body, jsonPath]);

        (bool success, bytes memory returnData) = JSON_GET_STRING_PRECOMPILE.staticcall(encodedParams);

        require(success, "json_get_string precompile call failed");

        string memory jsonValue = abi.decode(returnData, (string));

        return jsonValue;
    }
}
