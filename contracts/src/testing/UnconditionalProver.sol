// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Prover} from "../Prover.sol";
import {Web, WebProof, WebProofLib, WebLib} from "../WebProof.sol";

import "@openzeppelin-contracts-5.0.1/utils/Strings.sol";

contract UnconditionalProver is Prover {
    using Strings for string;
    using WebProofLib for WebProof;
    using WebLib for Web;

    string private constant DATA_URL = "https://swapi.dev/api/people/1";
    string private constant NOTARY_PUB_KEY =
        "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEBv36FI4ZFszJa0DQFJ3wWCXvVLFr\ncRzMG5kaTeHGoSzDu6cFqx3uEWYpFGo6C0EOUgf+mEgbktLrXocv5yHzKg==\n-----END PUBLIC KEY-----\n";

    constructor() {}

    function web_proof(WebProof calldata webProof) public view returns (bool) {
        Web memory web = WebProofLib.recover(webProof, DATA_URL);

        require(NOTARY_PUB_KEY.equal(web.notaryPubKey), "Incorrect notary public key");

        return true;
    }
}
