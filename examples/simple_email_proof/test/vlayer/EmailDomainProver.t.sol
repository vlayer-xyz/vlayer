// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {VTest} from "vlayer-0.1.0/testing/VTest.sol";
import {Proof} from "vlayer-0.1.0/Proof.sol";

import {UnverifiedEmail, EmailProofLib, VerifiedEmail} from "vlayer-0.1.0/EmailProof.sol";

import {EmailDomainProver} from "../../src/vlayer/EmailDomainProver.sol";

contract EmailProofLibWrapper {
    using EmailProofLib for UnverifiedEmail;

    function verify(UnverifiedEmail calldata email) public view returns (VerifiedEmail memory v) {
        return email.verify();
    }
}

contract EmailDomainProverTest is VTest {
    string constant HARDCODED_DNS_RECORD =
        "v=DKIM1; k=rsa; p=MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAoDLLSKLb3eyflXzeHwBz8qqg9mfpmMY+f1tp+HpwlEOeN5iHO0s4sCd2QbG2i/DJRzryritRnjnc4i2NJ/IJfU8XZdjthotcFUY6rrlFld20a13q8RYBBsETSJhYnBu+DMdIF9q3YxDtXRFNpFCpI1uIeA/x+4qQJm3KTZQWdqi/BVnbsBA6ZryQCOOJC3Ae0oodvz80yfEJUAi9hAGZWqRn+Mprlyu749uQ91pTOYCDCbAn+cqhw8/mY5WMXFqrw9AdfWrk+MwXHPVDWBs8/Hm8xkWxHOqYs9W51oZ/Je3WWeeggyYCZI9V+Czv7eF8BD/yF9UxU/3ZWZPM8EWKKQIDAQAB";

    function getTestEmail(string memory path) public view returns (UnverifiedEmail memory) {
        string memory mime = vm.readFile(path);
        string[] memory dnsRecords = new string[](1);
        dnsRecords[0] = HARDCODED_DNS_RECORD;
        return UnverifiedEmail(mime, dnsRecords);
    }

    function test_verifiesEmailDomain() public {
        EmailProofLibWrapper wrapper = new EmailProofLibWrapper();
        address johnDoe = vm.addr(1);
        EmailDomainProver prover = new EmailDomainProver();
        UnverifiedEmail memory email = getTestEmail("./vlayer/testdata/verify_vlayer.eml");
        callProver();
        VerifiedEmail memory verifiedEmail = wrapper.verify(email);
        callProver();
        (, bytes32 emailHash, address registeredWallet, string memory emailDomain) = prover.main(email, johnDoe);

        assertEq(emailHash, sha256(abi.encodePacked(verifiedEmail.from)));
        assertEq(registeredWallet, johnDoe);
        assertEq(emailDomain, "vlayer.xyz");
    }
}
