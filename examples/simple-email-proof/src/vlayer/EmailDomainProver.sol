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

    function stringToAddress(string memory str) public pure returns (address) {
        bytes memory strBytes = bytes(str);
        require(strBytes.length == 42, "Invalid address length");
        bytes memory addrBytes = new bytes(20);

        for (uint256 i = 0; i < 20; i++) {
            addrBytes[i] = bytes1(hexCharToByte(strBytes[2 + i * 2]) * 16 + hexCharToByte(strBytes[3 + i * 2]));
        }

        return address(uint160(bytes20(addrBytes)));
    }

    function hexCharToByte(bytes1 char) internal pure returns (uint8) {
        uint8 byteValue = uint8(char);
        if (byteValue >= uint8(bytes1("0")) && byteValue <= uint8(bytes1("9"))) {
            return byteValue - uint8(bytes1("0"));
        } else if (byteValue >= uint8(bytes1("a")) && byteValue <= uint8(bytes1("f"))) {
            return 10 + byteValue - uint8(bytes1("a"));
        } else if (byteValue >= uint8(bytes1("A")) && byteValue <= uint8(bytes1("F"))) {
            return 10 + byteValue - uint8(bytes1("A"));
        }
        revert("Invalid hex character");
    }

    function main(UnverifiedEmail calldata unverifiedEmail)
        public
        view
        returns (Proof memory, bytes32, address, string memory)
    {
        VerifiedEmail memory email = unverifiedEmail.verify();
        string[] memory subjectCapture = email.subject.capture("^Mint my domain NFT at address: (0x[a-fA-F0-9]{40})$");
        require(subjectCapture.length > 0, "no wallet address in subject");

        address targetWallet = stringToAddress(subjectCapture[1]);

        string[] memory captures = email.from.capture("^[\\w.-]+@([a-zA-Z\\d.-]+\\.[a-zA-Z]{2,})$");
        require(captures.length == 2, "invalid email domain");
        require(bytes(captures[1]).length > 0, "invalid email domain");

        return (proof(), sha256(abi.encodePacked(email.from)), targetWallet, captures[1]);
    }
}
