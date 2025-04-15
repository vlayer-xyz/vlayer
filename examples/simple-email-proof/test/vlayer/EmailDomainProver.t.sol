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
    function getTestEmail(string memory path) public view returns (UnverifiedEmail memory) {
        string memory mime = vm.readFile(path);
        return preverifyEmail(mime);
    }

    function test_verifiesEmailDomain() public {
        EmailProofLibWrapper wrapper = new EmailProofLibWrapper();
        address johnDoe = vm.addr(1);
        EmailDomainProver prover = new EmailDomainProver();
        UnverifiedEmail memory email = getTestEmail("testdata/verify_vlayer.eml");
        VerifiedEmail memory verifiedEmail = wrapper.verify(email);
        callProver();
        (, bytes32 emailHash, address registeredWallet, string memory emailDomain) = prover.main(email);

        assertEq(emailHash, sha256(abi.encodePacked(verifiedEmail.from)));
        assertEq(registeredWallet, johnDoe);
        assertEq(emailDomain, "vlayer.xyz");
    }
}
