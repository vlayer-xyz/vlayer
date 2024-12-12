// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Strings} from "@openzeppelin-contracts-5.0.1/utils/Strings.sol";
import {Proof} from "vlayer-0.1.0/Proof.sol";
import {Prover} from "vlayer-0.1.0/Prover.sol";
import {RegexLib} from "vlayer-0.1.0/Regex.sol";
import {VerifiedEmail, UnverifiedEmail, EmailProofLib} from "vlayer-0.1.0/EmailProof.sol";

contract EmailDomainProver is Prover {
    using RegexLib for string;
    using Strings for string;
    using EmailProofLib for UnverifiedEmail;

    function main(UnverifiedEmail calldata unverifiedEmail, address targetWallet)
        public
        view
        returns (Proof memory, bytes32, address, string memory)
    {
        VerifiedEmail memory email = unverifiedEmail.verify();
        require(email.subject.equal("Verify me for Email NFT"), "incorrect subject");
        string[] memory captures = email.from.capture("^[^@]+@([^@]+)$");
        require(captures.length == 2, "invalid email domain");
        require(bytes(captures[1]).length > 0, "invalid email domain");

        return (proof(), sha256(abi.encodePacked(email.from)), targetWallet, captures[1]);
    }
}
