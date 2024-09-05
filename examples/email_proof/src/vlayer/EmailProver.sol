// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Strings} from "openzeppelin/contracts/utils/Strings.sol";

import {Prover} from "vlayer/Prover.sol";
import {EmailProof, EmailProofLib} from "vlayer/EmailProof.sol";

interface IExample {
    function exampleFunction() external returns (uint256);
}

contract EmailProver is Prover {
    using Strings for string;
    using EmailProofLib for EmailProof;

    function main(EmailProof calldata emailProof) public view returns (bool) {
        string memory body = emailProof.verify();

        return true;
    }
}
