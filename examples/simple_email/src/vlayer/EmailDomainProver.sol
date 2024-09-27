// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Strings} from "@openzeppelin-contracts-5.0.1/utils/Strings.sol";
import {Prover} from "vlayer-0.1.0/src/Prover.sol";
import {VerifiedEmail, UnverifiedEmail, EmailProofLib} from "vlayer-0.1.0/src/EmailProof.sol";

contract EmailDomainProver is Prover {
    using Strings for string;
    using EmailProofLib for UnverifiedEmail;

    string targetDomain;

    constructor(string memory _targetDomain) {
        targetDomain = _targetDomain;
    }

    function main(UnverifiedEmail calldata unverifiedEmail, address targetWallet)
        public
        view
        returns (bytes32, address)
    {
        VerifiedEmail memory email = unverifiedEmail.verify();

        require(contains(email.from, targetDomain), "incorrect sender domain");
        require(email.subject.equal("Verify me for company NFT"), "incorrect subject");

        return (sha256(abi.encodePacked(email.from)), targetWallet);
    }

    function contains(string memory _string, string memory _substring) public pure returns (bool) {
        bytes memory stringBytes = bytes(_string);
        bytes memory substringBytes = bytes(_substring);

        for (uint256 i = 0; i <= stringBytes.length - substringBytes.length; i++) {
            bool found = true;
            for (uint256 j = 0; j < substringBytes.length; j++) {
                if (stringBytes[i + j] != substringBytes[j]) {
                    found = false;
                    break;
                }
            }
            if (found) return true;
        }
        return false;
    }
}
