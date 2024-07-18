// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Prover} from "vlayer/Prover.sol";
import {Strings} from "openzeppelin/contracts/utils/Strings.sol";

struct Web {
    string url;
    string content;
}

contract WebProver is Prover {
    constructor() {}

    function main(Web calldata web) public pure returns (string memory) {
        require(Strings.equal(web.url, "https://api.x.com"), "Invalid URL");

        return (web.content);
    }
}
