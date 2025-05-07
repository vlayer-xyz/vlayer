// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Strings} from "@openzeppelin-contracts-5.0.1/utils/Strings.sol";

import {Proof} from "vlayer/Proof.sol";
import {UrlLib} from "vlayer/Url.sol";
import {Prover} from "vlayer/Prover.sol";
import {Web, WebProof, WebProofLib, WebLib} from "vlayer/WebProof.sol";

// this prover contract is used in playwright e2e tests
contract LotrApiProver is Prover {
    using Strings for string;
    using UrlLib for string;
    using WebProofLib for WebProof;
    using WebLib for Web;

    string private constant DATA_URL = "https://lotr-api.online:3011/regular_json?are_you_sure=yes";
    string private constant NOTARY_PUB_KEY =
        "-----BEGIN PUBLIC KEY-----\nMFYwEAYHKoZIzj0CAQYFK4EEAAoDQgAEe0jxnBObaIj7Xjg6TXLCM1GG/VhY5650\nOrS/jgcbBufo/QDfFvL/irzIv1JSmhGiVcsCHCwolhDXWcge7v2IsQ==\n-----END PUBLIC KEY-----\n";

    // solhint-disable-next-line func-name-mixedcase
    function web_proof(WebProof calldata webProof) public view returns (Proof memory, string memory, string memory) {
        Web memory web =
            WebProofLib.recover(webProof, WebProofLib.UrlTestMode.Prefix, WebProofLib.BodyRedactionMode.Enabled_UNSAFE);

        require(NOTARY_PUB_KEY.equal(web.notaryPubKey), "Incorrect notary public key");

        require(web.jsonGetBool("success"), "Got unsuccessful response in WebProof");

        require(
            web.url.startsWith("https://lotr-api.online:3011/regular_json?are_you_sure=yes"),
            "Incorrect redaction of URL"
        );

        string memory name = web.jsonGetString("name");
        string memory greeting = web.jsonGetString("greeting");

        return (proof(), name, greeting);
    }
}
