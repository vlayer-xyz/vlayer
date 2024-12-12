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
        "v=DKIM1; k=rsa; p=MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA3gWcOhCm99qzN+h7/2+LeP3CLsJkQQ4EP/2mrceXle5pKq8uZmBl1U4d2Vxn4w+pWFANDLmcHolLboESLFqEL5N6ae7u9b236dW4zn9AFkXAGenTzQEeif9VUFtLAZ0Qh2eV7OQgz/vPj5IaNqJ7h9hpM9gO031fe4v+J0DLCE8Rgo7hXbNgJavctc0983DaCDQaznHZ44LZ6TtZv9TBs+QFvsy4+UCTfsuOtHzoEqOOuXsVXZKLP6B882XbEnBpXEF8QzV4J26HiAJFUbO3mAqZL2UeKC0hhzoIZqZXNG0BfuzOF0VLpDa18GYMUiu+LhEJPJO9D8zhzvQIHNrpGwIDAQAB";

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
        VerifiedEmail memory verifiedEmail = wrapper.verify(email);
        callProver();
        (, bytes32 emailHash, address registeredWallet, string memory emailDomain) = prover.main(email, johnDoe);

        assertEq(emailHash, sha256(abi.encodePacked(verifiedEmail.from)));
        assertEq(registeredWallet, johnDoe);
        assertEq(emailDomain, "vlayer.xyz");
    }
}
