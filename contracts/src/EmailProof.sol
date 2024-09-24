// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

struct UnverifiedEmail {
    string email;
}

struct VerifiedEmail {
    string from;
    string to;
    string subject;
    string body;
}

library EmailProofLib {
    address private constant VERIFY_EMAIL_PRECOMPILE = address(0x102);

    function verify(UnverifiedEmail memory unverifiedEmail) internal view returns (VerifiedEmail memory) {
        (bool success, bytes memory emailBytes) = VERIFY_EMAIL_PRECOMPILE.staticcall(bytes(unverifiedEmail.email));
        require(success, "verify_email precompile call failed");
        VerifiedEmail memory email = abi.decode(emailBytes, (VerifiedEmail));
        return email;
    }
}
