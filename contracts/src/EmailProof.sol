// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Address} from "@openzeppelin-contracts-5.0.1/utils/Address.sol";

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
        (bool success, bytes memory returnData) =
            Precompiles.VERIFY_EMAIL_PRECOMPILE.staticcall(abi.encode(unverifiedEmail));
        Address.verifyCallResult(success, returnData);

        VerifiedEmail memory email = abi.decode(returnData, (VerifiedEmail));
        return email;
    }
}
