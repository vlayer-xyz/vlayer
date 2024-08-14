// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "openzeppelin/contracts/utils/Strings.sol";

import {Prover} from "vlayer/Prover.sol";
import {WebProof} from "vlayer/WebProof.sol";

interface IExample {
    function exampleFunction() external returns (uint256);
}

contract WebProofProver is Prover {
    using Strings for string;

    string dataWebProof = "api.x.com";

    constructor() {}

    function main(WebProof calldata webProof) public returns (bool) {
        require(
            webProof.web_proof_json.equal(dataWebProof),
            "Incorrect web proof"
        );

        return true;
    }
}
