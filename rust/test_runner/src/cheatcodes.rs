use alloy_sol_types::{private::Address, sol};
use forge::revm::primitives::address;

pub const CHEATCODE_CALL_ADDR: Address = address!("e5F6E4A8da66436561059673919648CdEa4e486B");

sol!(
    #[derive(Default)]
    struct Seal {
        bytes18 lhv;
        bytes19 rhv;
    }
    #[derive(Default)]
    struct CallAssumptions {
        address proverContractAddress;
        bytes4 functionSelector;
        uint256 settleBlockNumber; // Block number for which the assumptions was made.
        bytes32 settleBlockHash; // Hash of the block at the specified block number.
    }
    #[derive(Default)]
    struct Proof {
        uint256 length;
        Seal seal;
        CallAssumptions callAssumptions;
    }

    #[derive(Default)]
    struct DnsRecord {
        string name;
        uint8 recordType;
        string data;
        uint64 ttl;
    }

    #[derive(Default)]
    struct VerificationData {
        uint64 validUntil;
        bytes signature;
        bytes pubKey;
    }

    #[derive(Default)]
    struct UnverifiedEmail {
        string email;
        DnsRecord dnsRecord;
        VerificationData verificationData;
    }

    function callProver() external returns (bool);
    function getProof() external returns (Proof memory);
    function preverifyEmail(string memory email) external returns (UnverifiedEmail memory);
);

impl From<verifiable_dns::VerificationData> for VerificationData {
    fn from(value: verifiable_dns::VerificationData) -> Self {
        Self {
            validUntil: value.valid_until,
            signature: value.signature.0.into(),
            pubKey: value.pub_key.0.into(),
        }
    }
}
