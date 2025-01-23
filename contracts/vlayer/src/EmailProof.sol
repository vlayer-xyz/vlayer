// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Address} from "@openzeppelin-contracts-5.0.1/utils/Address.sol";

import {Precompiles} from "./PrecompilesAddresses.sol";

struct DnsRecord {
    string name;
    uint8 recordType;
    string data;
    uint64 ttl;
}

struct VerificationData {
    uint64 validUntil;
    bytes signature;
    bytes pubKey;
}

struct UnverifiedEmail {
    string email;
    DnsRecord dnsRecord;
    VerificationData verificationData;
}

struct VerifiedEmail {
    string from;
    string to;
    string subject;
    string body;
}

library EmailProofLib {
    function verify(UnverifiedEmail memory unverifiedEmail) internal view returns (VerifiedEmail memory) {
        require(unverifiedEmail.verificationData.validUntil > block.timestamp, "EmailProof: expired DNS verification");

        (bool success, bytes memory returnData) = Precompiles.VERIFY_EMAIL.staticcall(abi.encode(unverifiedEmail));
        Address.verifyCallResult(success, returnData);

        VerifiedEmail memory email = abi.decode(returnData, (VerifiedEmail));
        return email;
    }
}
