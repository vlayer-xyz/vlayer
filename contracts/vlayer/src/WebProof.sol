// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Strings} from "@openzeppelin-contracts-5.0.1/utils/Strings.sol";
import {Address} from "@openzeppelin-contracts-5.0.1/utils/Address.sol";
import {ChainIdLibrary} from "./proof_verifier/ChainId.sol";
import {UrlLib} from "./Url.sol";
import {Precompiles} from "./PrecompilesAddresses.sol";
import {MainnetStableDeployment} from "./MainnetStableDeployment.sol";
import {TestnetStableDeployment} from "./TestnetStableDeployment.sol";

struct WebProof {
    string webProofJson;
}

struct Web {
    string body;
    string notaryPubKey;
    string url;
}

library WebProofLib {
    using Strings for string;
    using UrlLib for string;

    enum UrlTestMode {
        Full,
        Prefix
    }

    enum BodyRedactionMode {
        Disabled,
        Enabled_UNSAFE
    }

    // Generated using command `curl -s https://notary.pse.dev/v0.1.0-alpha.7/info | jq -r '.publicKey' | openssl ec -pubin -inform PEM -pubout -conv_form uncompressed`
    string private constant NOTARY_PUB_KEY =
        "-----BEGIN PUBLIC KEY-----\nMFYwEAYHKoZIzj0CAQYFK4EEAAoDQgAEe0jxnBObaIj7Xjg6TXLCM1GG/VhY5650\nOrS/jgcbBufo/QDfFvL/irzIv1JSmhGiVcsCHCwolhDXWcge7v2IsQ==\n-----END PUBLIC KEY-----\n";

    function verify(WebProof memory webProof, string memory url) internal view returns (Web memory) {
        Web memory web = recover(webProof, UrlTestMode.Full, BodyRedactionMode.Disabled);
        verifyNotaryKey(web.notaryPubKey);
        require(web.url.equal(url), "URL mismatch");
        return web;
    }

    function verifyWithUrlPrefix(WebProof memory webProof, string memory urlPrefix)
        internal
        view
        returns (Web memory)
    {
        Web memory web = recover(webProof, UrlTestMode.Prefix, BodyRedactionMode.Disabled);
        verifyNotaryKey(web.notaryPubKey);
        require(web.url.startsWith(urlPrefix), "URL prefix mismatch");
        return web;
    }

    function unsafeVerifyWithRedactedBody(
        WebProof memory webProof,
        string memory urlOrUrlPrefix,
        UrlTestMode urlTestMode
    ) internal view returns (Web memory) {
        Web memory web = recover(webProof, UrlTestMode.Prefix, BodyRedactionMode.Enabled_UNSAFE);
        verifyNotaryKey(web.notaryPubKey);
        if (urlTestMode == UrlTestMode.Full) {
            require(web.url.equal(urlOrUrlPrefix), "URL mismatch");
        } else if (urlTestMode == UrlTestMode.Prefix) {
            require(web.url.startsWith(urlOrUrlPrefix), "URL prefix mismatch");
        }
        return web;
    }

    function recover(WebProof memory webProof, UrlTestMode urlTestMode, BodyRedactionMode bodyRedactionMode)
        internal
        view
        returns (Web memory)
    {
        (bool success, bytes memory returnData) =
            Precompiles.VERIFY_AND_PARSE.staticcall(abi.encode(webProof, urlTestMode, bodyRedactionMode));

        Address.verifyCallResult(success, returnData);

        string[4] memory data = abi.decode(returnData, (string[4]));

        return Web(data[2], data[3], data[0]);
    }

    function verifyNotaryKey(string memory pubKey) internal view {
        if (ChainIdLibrary.isTestEnv()) {
            require(NOTARY_PUB_KEY.equal(pubKey), "Invalid notary public key");
        } else if (ChainIdLibrary.isMainnet()) {
            require(MainnetStableDeployment.repository().isNotaryKeyValid(pubKey), "Invalid notary public key");
        } else {
            require(TestnetStableDeployment.repository().isNotaryKeyValid(pubKey), "Invalid notary public key");
        }
    }
}

library WebLib {
    function jsonGetString(Web memory web, string memory jsonPath) internal view returns (string memory) {
        require(bytes(web.body).length > 0, "Body is empty");

        bytes memory encodedParams = abi.encode([web.body, jsonPath]);
        (bool success, bytes memory returnData) = Precompiles.JSON_GET_STRING.staticcall(encodedParams);
        Address.verifyCallResult(success, returnData);

        return abi.decode(returnData, (string));
    }

    function jsonGetInt(Web memory web, string memory jsonPath) internal view returns (int256) {
        require(bytes(web.body).length > 0, "Body is empty");

        bytes memory encodedParams = abi.encode([web.body, jsonPath]);
        (bool success, bytes memory returnData) = Precompiles.JSON_GET_INT.staticcall(encodedParams);
        Address.verifyCallResult(success, returnData);

        return abi.decode(returnData, (int256));
    }

    function jsonGetBool(Web memory web, string memory jsonPath) internal view returns (bool) {
        require(bytes(web.body).length > 0, "Body is empty");

        bytes memory encodedParams = abi.encode([web.body, jsonPath]);
        (bool success, bytes memory returnData) = Precompiles.JSON_GET_BOOL.staticcall(encodedParams);
        Address.verifyCallResult(success, returnData);

        return abi.decode(returnData, (bool));
    }
}
