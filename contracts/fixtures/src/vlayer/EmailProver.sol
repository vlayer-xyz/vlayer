// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Strings} from "@openzeppelin-contracts-5.0.1/utils/Strings.sol";

import {Proof} from "vlayer/Proof.sol";
import {Prover} from "vlayer/Prover.sol";
import {RegexLib} from "vlayer/Regex.sol";
import {EmailProofLib, VerifiedEmail, UnverifiedEmail} from "vlayer/EmailProof.sol";
import {IVDnsKeyRepository} from "vlayer/Repository.sol";

import {AddressParser} from "./utils/AddressParser.sol";

interface IExample {
    function exampleFunction() external returns (uint256);
}

contract EmailProver is Prover {
    using Strings for string;
    using RegexLib for string;
    using AddressParser for string;
    using EmailProofLib for UnverifiedEmail;

    IVDnsKeyRepository public vDnsKeyVerifier;

    function main(UnverifiedEmail calldata unverifiedEmail) public view returns (Proof memory) {
        VerifiedEmail memory email = unverifiedEmail.verify();

        require(email.subject.equal("Verify me for Email NFT"), "incorrect subject");
        require(email.from.matches("^.*@vlayer.xyz$"), "from must be a vlayer address");

        return (proof());
    }
}
