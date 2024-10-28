// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Prover} from "vlayer/Prover.sol";
import {Web, WebProof, WebProofLib, WebLib} from "vlayer/WebProof.sol";

import "@openzeppelin-contracts-5.0.1/utils/Strings.sol";

contract UnconditionalProver is Prover {
    using Strings for string;
    using WebProofLib for WebProof;
    using WebLib for Web;
    constructor() {}
    function web_proof(WebProof calldata webProof) public view returns (bool) {
        return true;
    }
}
