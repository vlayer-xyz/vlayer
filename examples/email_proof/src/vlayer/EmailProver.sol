// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Strings} from "@openzeppelin-contracts-5.0.1/utils/Strings.sol";

import {Proof} from "vlayer-0.1.0/Proof.sol";
import {Prover} from "vlayer-0.1.0/Prover.sol";
import {VerifiedEmail, UnverifiedEmail, EmailProofLib} from "vlayer-0.1.0/EmailProof.sol";

interface IExample {
    function exampleFunction() external returns (uint256);
}

contract EmailProver is Prover {
    using Strings for string;
    using EmailProofLib for UnverifiedEmail;

    function main(UnverifiedEmail calldata unverifiedEmail) public view returns (Proof memory, bool) {
        VerifiedEmail memory email = unverifiedEmail.verify();

        require(email.subject.equal("Is dinner ready?"), "subject must be 'Is dinner ready?'");

        return (proof(), true);
    }
}
