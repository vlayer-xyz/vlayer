use alloy_sol_types::{Error, SolValue};
use verifiable_dns::{DNSRecord, PublicKey, RecordType, Signature, VerificationData};

use crate::email::Email;

mod private {
    use alloy_sol_types::sol;

    sol!("../../contracts/vlayer/src/EmailProof.sol");
}

pub use private::{
    DnsRecord as SolDnsRecord, UnverifiedEmail, VerificationData as SolVerificationData,
    VerifiedEmail as SolEmail,
};

impl From<Email> for SolEmail {
    fn from(email: Email) -> SolEmail {
        SolEmail {
            from: email.from,
            to: email.to,
            subject: email.subject.unwrap_or_default(),
            body: email.body,
        }
    }
}

impl UnverifiedEmail {
    pub(crate) fn parse_calldata(
        calldata: &[u8],
    ) -> Result<(Vec<u8>, DNSRecord, VerificationData), Error> {
        let unverified_email = UnverifiedEmail::abi_decode(calldata, true)?;
        let raw_email = unverified_email.email.into_bytes();
        let dns_record = DNSRecord {
            name: unverified_email.dnsRecord.name,
            record_type: parse_record_type(&unverified_email.dnsRecord.recordType)?,
            data: unverified_email.dnsRecord.data,
            valid_until: unverified_email.dnsRecord.validUntil,
        };
        let verification_data = VerificationData {
            valid_until: unverified_email.verificationData.validUntil,
            signature: Signature(unverified_email.verificationData.signature.into()),
            pub_key: PublicKey(unverified_email.verificationData.pubKey.into()),
        };
        Ok((raw_email, dns_record, verification_data))
    }
}

fn parse_record_type(record_type: &str) -> Result<RecordType, Error> {
    match record_type {
        "TXT" => Ok(RecordType::TXT),
        _ => Err(Error::custom(format!(
            "Unexpected DNS record type: {record_type}. Supported types: TXT"
        ))),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    mod unverified_email {
        use alloy_sol_types::private::bytes;

        use super::*;

        #[test]
        fn test_parse_calldata() {
            let input_email = UnverifiedEmail {
                email: "email".into(),
                dnsRecord: SolDnsRecord {
                    name: "name".into(),
                    recordType: "TXT".into(),
                    data: "data".into(),
                    validUntil: 123,
                },
                verificationData: SolVerificationData {
                    validUntil: 456,
                    signature: bytes!("1234"),
                    pubKey: bytes!("5678"),
                },
            };
            let bytecode = UnverifiedEmail::abi_encode(&input_email);

            let (raw_email, dns_records, _) = UnverifiedEmail::parse_calldata(&bytecode).unwrap();
            assert_eq!(raw_email, "email".as_bytes());
            assert_eq!(
                dns_records,
                DNSRecord {
                    name: "name".into(),
                    record_type: RecordType::TXT,
                    data: "data".into(),
                    valid_until: 123,
                }
            );
        }

        #[test]
        fn test_error_if_parse_calldata_fails() {
            let result = UnverifiedEmail::parse_calldata(&[0x00]);
            assert_eq!(result, Err(Error::Overrun));
        }
    }
}
