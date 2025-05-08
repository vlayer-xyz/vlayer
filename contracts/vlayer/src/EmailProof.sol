// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Address} from "@openzeppelin-contracts-5.0.1/utils/Address.sol";
import {ChainIdLibrary} from "./proof_verifier/ChainId.sol";
import {Precompiles} from "./PrecompilesAddresses.sol";
import {TestnetStableDeployment} from "./TestnetStableDeployment.sol";
import {MainnetStableDeployment} from "./MainnetStableDeployment.sol";

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

// Generated with: `openssl pkey -pubin -in rust/verifiable_dns/assets/public_key.pem -outform DER | xxd -p`
// TEST_DNS_PUBLIC_KEY = 0x30820122300d06092a864886f70d01010105000382010f003082010a0282010100e41b913c0e5e78a84fec1ec6f289036d3ce7737e523e0ecf6b8bb9b08ff95d776c96838b9e702e89e99ebe75ed6812fed63f14fb2591ebab0e940e8a89537de2304643026022d313b38e658197e6526d0bee27bc60fc5a822baeefe9934406ed6d186620676c64da4426e3233d0a3fc118a4c905adc5e539a6ad995cd07d1ed8c96f3a9dbe236ce05b2e01b916e467a30fcee90c4006dc101de818f1003ae21b1e00602ff5dc0c6f80f5153bdf2df1a23068c598434e86cc31585311cd62aa647e6082feaecea25f804a3fcc487fec2bb7feb610027750dd0b88ac65860600887a156ef705761ff11eea53835530ccd4f9b0f8e6dd308217f39c1edcc70ee65d0203010001;
bytes32 constant TEST_DNS_PUBLIC_KEY_HASH = 0xc16646301c7615357b8f8ee125956b0e5fbf972fa2a0c26feb1f1ae75d04103f; // keccak256(TEST_DNS_PUBLIC_KEY)

library EmailProofLib {
    function verify(UnverifiedEmail memory unverifiedEmail) internal view returns (VerifiedEmail memory) {
        require(unverifiedEmail.verificationData.validUntil > block.timestamp, "EmailProof: expired DNS verification");
        if (ChainIdLibrary.isTestEnv()) {
            require(
                keccak256(unverifiedEmail.verificationData.pubKey) == TEST_DNS_PUBLIC_KEY_HASH,
                "Not a valid VDNS hardcoded key"
            );
        } else if (ChainIdLibrary.isMainnet()) {
            require(
                MainnetStableDeployment.repository().isDnsKeyValid(unverifiedEmail.verificationData.pubKey),
                "Not a valid VDNS public key"
            );
        } else {
            require(
                TestnetStableDeployment.repository().isDnsKeyValid(unverifiedEmail.verificationData.pubKey),
                "Not a valid VDNS public key"
            );
        }

        (bool success, bytes memory returnData) = Precompiles.VERIFY_EMAIL.staticcall(abi.encode(unverifiedEmail));
        Address.verifyCallResult(success, returnData);

        VerifiedEmail memory email = abi.decode(returnData, (VerifiedEmail));
        return email;
    }
}
