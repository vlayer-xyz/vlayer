// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {VTest} from "vlayer/testing/VTest.sol";
import "./EmailProver.sol";
import {EmailProof, EmailProofLib} from "vlayer/EmailProof.sol";

contract EmailProofLibWrapper {
    using EmailProofLib for EmailProof;

    function verify(EmailProof calldata emailProof) public view returns (string memory) {
        return emailProof.verify();
    }
}

contract EmailProverTest is VTest {
    using EmailProofLib for EmailProof;

    function test_decodesEmail() public {
        EmailProofLibWrapper wrapper = new EmailProofLibWrapper();
        EmailProof memory emailProof = EmailProof("From: vitalik@gmail.com");
        callProver();
        string memory email = wrapper.verify(emailProof);
        assertEq(email, "From: vitalik@gmail.com");
    }
}
