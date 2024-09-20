// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {VTest} from "vlayer-0.1.0/src/testing/VTest.sol";
import "./EmailProver.sol";
import {MimeEmail, EmailProofLib, VerifiedEmail} from "vlayer-0.1.0/src/EmailProof.sol";

contract EmailProofLibWrapper {
    using EmailProofLib for MimeEmail;

    function verify(MimeEmail calldata email) public view returns (VerifiedEmail memory) {
        return email.verify();
    }
}

contract EmailProverTest is VTest {
    using EmailProofLib for MimeEmail;

    function test_decodesEmail() public {
        EmailProofLibWrapper wrapper = new EmailProofLibWrapper();
        string memory mime = vm.readFile("./testdata/test_email.txt");
        MimeEmail memory email = MimeEmail(mime);
        callProver();
        VerifiedEmail memory verifiedEmail = wrapper.verify(email);
        assertEq(verifiedEmail.from, "Joe SixPack <joe@football.example.com>");
        assertEq(verifiedEmail.body, "Hi.\r\n\r\nWe lost the game. Are you hungry yet?\r\n\r\nJoe.\r\n");
    }
}
