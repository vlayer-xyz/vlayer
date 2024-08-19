// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "openzeppelin/contracts/utils/Strings.sol";

import {Prover} from "vlayer/Prover.sol";
import {WebProof, WebProofLib} from "vlayer/WebProof.sol";

interface IExample {
    function exampleFunction() external returns (uint256);
}

contract WebProofProver is Prover {
    using Strings for string;
    using WebProofLib for WebProof;

    constructor() {}

    function main(WebProof calldata webProof) public pure returns (bool) {
        bytes calldata web_proof_json = bytes(webProof.web_proof_json);
        require(web_proof_json[0] == "{", "Incorrect web proof");
        require(
            web_proof_json[web_proof_json.length - 1] == "}",
            "Incorrect web proof"
        );

        require(webProof.url().equal("api.x.com"), "Incorrect URL");

        return true;
    }
}
