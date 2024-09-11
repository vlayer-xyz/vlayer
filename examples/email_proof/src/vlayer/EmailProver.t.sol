// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {VTest} from "vlayer/testing/VTest.sol";
import "./EmailProver.sol";
import {EmailProof, EmailProofLib, Email} from "vlayer/EmailProof.sol";

contract EmailProofLibWrapper {
    using EmailProofLib for EmailProof;

    function verify(EmailProof calldata emailProof) public view returns (Email memory) {
        return emailProof.verify();
    }
}

contract EmailProverTest is VTest {
    using EmailProofLib for EmailProof;

    function test_decodesEmail() public {
        EmailProofLibWrapper wrapper = new EmailProofLibWrapper();
        string memory mime = "From: vitalik@gmail.com\nDate: Thu, 15 Aug 2019 14:54:37 +0900\n\nTHIS IS BODY";
        EmailProof memory emailProof = EmailProof(mime);
        callProver();
        Email memory email = wrapper.verify(emailProof);
        assertEq(email.from, "vitalik@gmail.com");
        assertEq(email.body, "THIS IS BODY");
        assertEq(email.date, 1565848477);
    }
}
