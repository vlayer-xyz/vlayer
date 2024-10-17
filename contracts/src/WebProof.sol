// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Strings} from "@openzeppelin-contracts-5.0.1/utils/Strings.sol";
import {Precompiles} from "./PrecompilesAddresses.sol";

struct WebProof {
    string webProofJson;
}

struct Web {
    string body;
    string notaryPubKey;
}

library WebProofLib {
    using Strings for string;

    string private constant NOTARY_PUB_KEY =
        "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAExpX/4R4z40gI6C/j9zAM39u58LJu\n3Cx5tXTuqhhu/tirnBi5GniMmspOTEsps4ANnPLpMmMSfhJ+IFHbc3qVOA==\n-----END PUBLIC KEY-----\n";

    function verify(WebProof memory webProof, string memory dataUrl) internal view returns (Web memory) {
        Web memory web = recover(webProof, dataUrl);

        require(NOTARY_PUB_KEY.equal(web.notaryPubKey), "Incorrect notary public key");

        return web;
    }

    function recover(WebProof memory webProof, string memory dataUrl) internal view returns (Web memory) {
        (bool success, bytes memory returnData) =
            Precompiles.VERIFY_AND_PARSE_PRECOMPILE.staticcall(bytes(webProof.webProofJson));

        require(success, "verify_and_parse precompile call failed");

        string[4] memory data = abi.decode(returnData, (string[4]));

        require(dataUrl.equal(data[0]), "Incorrect URL");

        return Web(data[2], data[3]);
    }
}
