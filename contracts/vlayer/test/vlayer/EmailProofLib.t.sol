// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {VTest} from "../../src/testing/VTest.sol";
import {EmailProofLib, UnverifiedEmail, VerifiedEmail} from "../../src/EmailProof.sol";
import {Strings} from "@openzeppelin-contracts-5.0.1/utils/Strings.sol";

contract EmailProofLibWrapper {
    using EmailProofLib for UnverifiedEmail;

    function verify(UnverifiedEmail calldata email) public view returns (VerifiedEmail memory v) {
        return email.verify();
    }
}

contract EmailProofTest is VTest {
    function getTestEmail(string memory path) public view returns (UnverifiedEmail memory) {
        string memory mime = vm.readFile(path);
        return preverifyEmail(mime);
    }

    function test_revertsIf_DnsVerificationIsExpired() public {
        EmailProofLibWrapper wrapper = new EmailProofLibWrapper();
        UnverifiedEmail memory email = getTestEmail("testdata/verify_vlayer.eml");
        email.verificationData.validUntil = uint64(block.timestamp - 1);
        vm.expectRevert("EmailProof: expired DNS verification");
        wrapper.verify(email);
    }
}
