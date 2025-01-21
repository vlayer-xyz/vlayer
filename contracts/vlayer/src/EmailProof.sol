// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Address} from "@openzeppelin-contracts-5.0.1/utils/Address.sol";

import {ChainIdLibrary} from "./proof_verifier/ChainId.sol";
import {IVDnsKeyVerifier} from "./interface/IVDnsKeyVerifier.sol";
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

IVDnsKeyVerifier constant KEY_VAULT = IVDnsKeyVerifier(address(uint160(uint256(keccak256("vlayer.keyvault")))));
bytes constant TEST_DNS_PUBLIC_KEY = "TEST_DNS_PUBLIC_KEY";

library EmailProofLib {
    function verify(UnverifiedEmail memory unverifiedEmail) internal view returns (VerifiedEmail memory) {
        if (ChainIdLibrary.is_mainnet() || ChainIdLibrary.is_testnet()) {
            require(KEY_VAULT.isKeyValid(unverifiedEmail.verificationData.pubKey), "Not a valid VDNS public key");
        } else if (ChainIdLibrary.is_devnet()) {
            require(keccak256(unverifiedEmail.verificationData.pubKey) == keccak256(TEST_DNS_PUBLIC_KEY), "Not a valid VDNS hardcoded key");
        }

        (bool success, bytes memory returnData) = Precompiles.VERIFY_EMAIL.staticcall(abi.encode(unverifiedEmail));
        Address.verifyCallResult(success, returnData);

        VerifiedEmail memory email = abi.decode(returnData, (VerifiedEmail));
        return email;
    }
}
