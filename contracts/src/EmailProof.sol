// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

struct EmailProof {
    string mimeEmail;
}

struct Email {
    string from;
    string to;
    string subject;
    string body;
    uint64 date;
}

library EmailProofLib {
    address private constant VERIFY_EMAIL_PRECOMPILE = address(0x102);

    function verify(EmailProof memory emailProof) internal view returns (Email memory) {
        (bool success, bytes memory email) = VERIFY_EMAIL_PRECOMPILE.staticcall(bytes(emailProof.mimeEmail));
        require(success, "verify_email precompile call failed");
        Email memory decodedEmail = abi.decode(email, (Email));
        return decodedEmail;
    }
}
