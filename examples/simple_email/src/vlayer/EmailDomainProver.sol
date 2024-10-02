// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Strings} from "@openzeppelin-contracts-5.0.1/utils/Strings.sol";
import {Proof} from "vlayer-0.1.0/src/Proof.sol";
import {Prover} from "vlayer-0.1.0/src/Prover.sol";
import {VerifiedEmail, UnverifiedEmail, EmailProofLib} from "vlayer-0.1.0/src/EmailProof.sol";
import {EmailStrings} from "./EmailStrings.sol";

contract EmailDomainProver is Prover {
    using Strings for string;
    using EmailStrings for string;
    using EmailProofLib for UnverifiedEmail;

    string targetDomain;

    constructor(string memory _targetDomain) {
        targetDomain = _targetDomain;
    }

    function main(UnverifiedEmail calldata unverifiedEmail, address targetWallet)
        public
        view
        returns (Proof memory, bytes32, address)
    {
        VerifiedEmail memory email = unverifiedEmail.verify();

        require(email.from.contains(targetDomain), "incorrect sender domain");
        require(email.subject.equal("Verify me for company NFT"), "incorrect subject");

        return (proof(), sha256(abi.encodePacked(email.from)), targetWallet);
    }
}
