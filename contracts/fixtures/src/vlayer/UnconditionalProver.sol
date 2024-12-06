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
        "-----BEGIN PUBLIC KEY-----\nMFYwEAYHKoZIzj0CAQYFK4EEAAoDQgAEZT9nJiwhGESLjwQNnZ2MsZ1xwjGzvmhF\nxFi8Vjzanlidbsc1ngM+s1nzlRkZI5UK9BngzmC27BO0qXxPSepIwQ==\n-----END PUBLIC KEY-----\n";

    constructor() {}

    function web_proof(WebProof calldata webProof) public view returns (bool) {
        Web memory web = WebProofLib.recover(webProof, DATA_URL);

        require(NOTARY_PUB_KEY.equal(web.notaryPubKey), "Incorrect notary public key");

        return true;
    }
}
