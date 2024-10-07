// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {VTest} from "vlayer-0.1.0/testing/VTest.sol";
import "./EmailProver.sol";
import {UnverifiedEmail, EmailProofLib, VerifiedEmail} from "vlayer-0.1.0/EmailProof.sol";

contract EmailProofLibWrapper {
    using EmailProofLib for UnverifiedEmail;

    function verify(UnverifiedEmail calldata email) public view returns (VerifiedEmail memory) {
        return email.verify();
    }
}

contract EmailProverTest is VTest {
    using EmailProofLib for UnverifiedEmail;

    string constant HARDCODED_DNS_RECORD =
        "newengland._domainkey.example.com v=DKIM1; p=MIGJAoGBALVI635dLK4cJJAH3Lx6upo3X/Lm1tQz3mezcWTA3BUBnyIsdnRf57aD5BtNmhPrYYDlWlzw3UgnKisIxktkk5+iMQMlFtAS10JB8L3YadXNJY+JBcbeSi5TgJe4WFzNgW95FWDAuSTRXSWZfA/8xjflbTLDx0euFZOM7C4T0GwLAgMBAAE=";

    function getTestEmail() public view returns (UnverifiedEmail memory) {
        string memory mime = vm.readFile("./vlayer/testdata/test_email.txt");
        string[] memory dnsRecords = new string[](1);
        dnsRecords[0] = HARDCODED_DNS_RECORD;
        return UnverifiedEmail(mime, dnsRecords);
    }

    function test_decodesEmail() public {
        EmailProofLibWrapper wrapper = new EmailProofLibWrapper();
        UnverifiedEmail memory email = getTestEmail();
        callProver();
        VerifiedEmail memory verifiedEmail = wrapper.verify(email);
        assertEq(verifiedEmail.from, "joe@football.example.com");
        assertEq(verifiedEmail.body, "Hi.\r\n\r\nWe lost the game. Are you hungry yet?\r\n\r\nJoe.\r\n");
    }

    function test_provesEmail() public {
        UnverifiedEmail memory email = getTestEmail();
        EmailProver prover = new EmailProver();
        callProver();
        prover.main(email);
    }
}
