// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Prover} from "vlayer/Prover.sol";
import {Web, WebProof, WebProofLib, WebLib} from "vlayer/WebProof.sol";

import "@openzeppelin-contracts-5.0.1/utils/Strings.sol";

contract UnconditionalProver is Prover {
    using Strings for string;
    using WebProofLib for WebProof;
    using WebLib for Web;

    string private constant DATA_URL = "https://lotr-api.online:3011/regular_json";
    string private constant NOTARY_PUB_KEY =
        "-----BEGIN PUBLIC KEY-----\nMFYwEAYHKoZIzj0CAQYFK4EEAAoDQgAEe0jxnBObaIj7Xjg6TXLCM1GG/VhY5650\nOrS/jgcbBufo/QDfFvL/irzIv1JSmhGiVcsCHCwolhDXWcge7v2IsQ==\n-----END PUBLIC KEY-----\n";

    constructor() {}

    function web_proof(WebProof calldata webProof) public view returns (bool) {
        Web memory web = WebProofLib.recover(webProof);

        require(NOTARY_PUB_KEY.equal(web.notaryPubKey), "Incorrect notary public key");

        return true;
    }
}
