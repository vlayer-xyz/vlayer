// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "vlayer-0.1.0/testing/VTest.sol";
import "./EmailProver.sol";
import {UnverifiedEmail, EmailProofLib, VerifiedEmail} from "vlayer-0.1.0/EmailProof.sol";

contract EmailProofLibWrapper {
    using EmailProofLib for UnverifiedEmail;

    function verify(UnverifiedEmail calldata email) public view returns (VerifiedEmail memory v) {
        return email.verify();
    }
}

contract EmailProverTest is VTest {
    using EmailProofLib for UnverifiedEmail;

    string constant HARDCODED_DNS_RECORD =
        "v=DKIM1; k=rsa; p=MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA3gWcOhCm99qzN+h7/2+LeP3CLsJkQQ4EP/2mrceXle5pKq8uZmBl1U4d2Vxn4w+pWFANDLmcHolLboESLFqEL5N6ae7u9b236dW4zn9AFkXAGenTzQEeif9VUFtLAZ0Qh2eV7OQgz/vPj5IaNqJ7h9hpM9gO031fe4v+J0DLCE8Rgo7hXbNgJavctc0983DaCDQaznHZ44LZ6TtZv9TBs+QFvsy4+UCTfsuOtHzoEqOOuXsVXZKLP6B882XbEnBpXEF8QzV4J26HiAJFUbO3mAqZL2UeKC0hhzoIZqZXNG0BfuzOF0VLpDa18GYMUiu+LhEJPJO9D8zhzvQIHNrpGwIDAQAB";

    function getTestEmail(string memory path) public view returns (UnverifiedEmail memory) {
        string memory mime = vm.readFile(path);
        string[] memory dnsRecords = new string[](1);
        dnsRecords[0] = HARDCODED_DNS_RECORD;
        return UnverifiedEmail(mime, dnsRecords);
    }

    function test_decodesEmail() public {
        EmailProofLibWrapper wrapper = new EmailProofLibWrapper();
        UnverifiedEmail memory email = getTestEmail("./vlayer/testdata/real_signed_email.eml");
        callProver();
        VerifiedEmail memory verifiedEmail = wrapper.verify(email);
        assertEq(verifiedEmail.from, "ivan@vlayer.xyz");
        assertEq(verifiedEmail.subject, "Is dinner ready?");
    }

    function test_provesEmail() public {
        UnverifiedEmail memory email = getTestEmail("./vlayer/testdata/real_signed_email.eml");
        EmailProver prover = new EmailProver();
        callProver();
        prover.main(email);
    }

    function test_doesNotAcceptTooLargeEmail() public {
        UnverifiedEmail memory email = getTestEmail("./vlayer/testdata/email_over_max_size.eml");
        EmailProver prover = new EmailProver();
        callProver();
        try prover.main(email) {
            revert("Did not revert as expected");
        } catch Error(string memory reason) {
            assertEq(reason, "CalldataTooLargeError(5243524)");
        }
    }
}
