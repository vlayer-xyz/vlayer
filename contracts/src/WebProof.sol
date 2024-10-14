// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Strings} from "@openzeppelin-contracts-5.0.1/utils/Strings.sol";

struct WebProof {
    string webProofJson;
}

struct Web {
    string body;
}

library WebProofLib {
    using Strings for string;

    address private constant VERIFY_AND_PARSE_PRECOMPILE = address(0x100);

    string private constant NOTARY_PUB_KEY =
        "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAExpX/4R4z40gI6C/j9zAM39u58LJu\n3Cx5tXTuqhhu/tirnBi5GniMmspOTEsps4ANnPLpMmMSfhJ+IFHbc3qVOA==\n-----END PUBLIC KEY-----\n";

    function verify(WebProof memory webProof, string memory dataUrl) internal view returns (Web memory) {
        (bool success, bytes memory returnData) = VERIFY_AND_PARSE_PRECOMPILE.staticcall(bytes(webProof.webProofJson));

        require(success, "verify_and_parse precompile call failed");

        string[4] memory data = abi.decode(returnData, (string[4]));

        require(dataUrl.equal(data[0]), "Incorrect URL");
        require(NOTARY_PUB_KEY.equal(data[3]), "Incorrect notary public key");

        return Web(data[2]);
    }
}

library WebLib {
    address private constant JSON_GET_STRING_PRECOMPILE = address(0x102);
    address private constant JSON_GET_INT_PRECOMPILE = address(0x103);
    address private constant JSON_GET_BOOL_PRECOMPILE = address(0x104);

    function jsonGetString(Web memory web, string memory jsonPath) internal view returns (string memory) {
        require(bytes(web.body).length > 0, "Body is empty");

        bytes memory encodedParams = abi.encode([web.body, jsonPath]);
        (bool success, bytes memory returnData) = JSON_GET_STRING_PRECOMPILE.staticcall(encodedParams);
        require(success, "json_get_string precompile call failed");

        return abi.decode(returnData, (string));
    }

    function jsonGetInt(Web memory web, string memory jsonPath) internal view returns (int) {
        require(bytes(web.body).length > 0, "Body is empty");

        bytes memory encodedParams = abi.encode([web.body, jsonPath]);
        (bool success, bytes memory returnData) = JSON_GET_INT_PRECOMPILE.staticcall(encodedParams);
        require(success, "json_get_string precompile call failed");

        return abi.decode(returnData, (int));
    }

    function jsonGetBool(Web memory web, string memory jsonPath) internal view returns (bool) {
        require(bytes(web.body).length > 0, "Body is empty");

        bytes memory encodedParams = abi.encode([web.body, jsonPath]);
        (bool success, bytes memory returnData) = JSON_GET_BOOL_PRECOMPILE.staticcall(encodedParams);
        require(success, "json_get_string precompile call failed");

        return abi.decode(returnData, (bool));
    }
}
