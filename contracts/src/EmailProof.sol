// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Precompiles} from "./PrecompilesAddresses.sol";

struct UnverifiedEmail {
    string email;
    string[] dnsRecords;
}

struct VerifiedEmail {
    string from;
    string to;
    string subject;
    string body;
}

library EmailProofLib {
    function verify(UnverifiedEmail memory unverifiedEmail) internal view returns (VerifiedEmail memory) {
        (bool success, bytes memory emailBytes) = Precompiles.VERIFY_EMAIL_PRECOMPILE.staticcall(abi.encode(unverifiedEmail));
        require(success, "verify_email precompile call failed");
        VerifiedEmail memory email = abi.decode(emailBytes, (VerifiedEmail));
        return email;
    }
}
