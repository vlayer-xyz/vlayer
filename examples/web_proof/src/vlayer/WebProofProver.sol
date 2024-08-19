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

    constructor() {}

    function main(WebProof calldata webProof) public returns (bool) {
        bytes calldata web_proof_json = bytes(webProof.web_proof_json);
        require(
            web_proof_json[0] == "{",
            string(abi.encodePacked("Incorrect web proof"))
        );
        require(
            web_proof_json[web_proof_json.length - 1] == "}",
            string(abi.encodePacked("Incorrect web proof"))
        );

        require(
            keccak256(abi.encodePacked(WebProofLib.url(webProof))) ==
                keccak256("api.x.com"),
            "Incorrect URL"
        );

        return true;
    }
}
