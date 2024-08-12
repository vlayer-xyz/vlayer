// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "@openzeppelin/contracts/utils/Strings.sol";

import {Prover} from "vlayer/Prover.sol";
import {Web} from "vlayer/Web.sol";

interface IExample {
    function exampleFunction() external returns (uint256);
}

contract WebProofProver is Prover {
    using Strings for string;

    string dataUrl = "api.x.com";

    constructor() {}

    function main(Web calldata web) public returns (bool) {
        require(web.url.equal(dataUrl), "Incorrect URL");

        return true;
    }
}
