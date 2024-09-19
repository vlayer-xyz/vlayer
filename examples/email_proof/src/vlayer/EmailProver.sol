// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Strings} from "@openzeppelin-contracts-5.0.1/utils/Strings.sol";

import {Prover} from "vlayer-0.1.0/src/Prover.sol";
import {VerifiedEmail, MimeEmail, EmailProofLib} from "vlayer-0.1.0/src/EmailProof.sol";

interface IExample {
    function exampleFunction() external returns (uint256);
}

contract EmailProver is Prover {
    using Strings for string;
    using EmailProofLib for MimeEmail;

    function main(MimeEmail calldata mimeEmail) public view returns (bool) {
        VerifiedEmail memory email = mimeEmail.verify();

        require(email.subject.equal("Is dinner ready?"), "subject must be 'Is dinner ready?'");

        return true;
    }
}
