mod dkim;
mod dns;
mod email;
mod errors;
mod from_header;
#[cfg(test)]
mod test_utils;

use dkim::{get_dkim_header, verify_signature::verify_signature};
use dns::extract_public_key;
pub use email::sol::{SolDnsRecord, SolVerificationData, UnverifiedEmail};
use mailparse::{ParsedMail, parse_mail};
use verifiable_dns::DNSRecord;

pub use crate::{email::Email, errors::Error};

const REQUIRED_SIGNED_HEADERS: [&str; 3] = ["from", "to", "subject"];

pub fn parse_and_verify(calldata: &[u8]) -> Result<Email, Error> {
    let (raw_email, dns_record, verification_data) = UnverifiedEmail::parse_calldata(calldata)?;

    let email = parse_mail(&raw_email)?;
    let dkim_public_key = extract_public_key(&dns_record.data)?;

    validate_headers(&email, &dns_record)?;
    dns_record.verify(&verification_data)?;
    verify_signature(&email, dkim_public_key)?;

    Ok(email.try_into()?)
}

fn validate_headers(email: &ParsedMail, dns_record: &DNSRecord) -> Result<(), Error> {
    let raw_headers = parse_headers_bytes(email.raw_bytes)?;
    let dkim_header = get_dkim_header(email)?;

    verify_no_fake_separator(raw_headers)?;
    dkim_header.verify_dns_consistency(dns_record)?;
    dkim_header.verify_required_headers_signed(&REQUIRED_SIGNED_HEADERS)?;
    dkim_header.verify_body_length_tag()?;

    Ok(())
}

#[allow(clippy::expect_used)]
fn parse_headers_bytes(raw_email: &[u8]) -> Result<&[u8], Error> {
    let email_str = std::str::from_utf8(raw_email).expect("Email already verified");

    let header_end = email_str
        .find("\r\n\r\n")
        .ok_or(Error::MissingBodySeparator)?;

    let headers_part = &email_str[..header_end];
    Ok(headers_part.as_bytes())
}

fn verify_no_fake_separator(raw_headers: &[u8]) -> Result<(), Error> {
    for i in 0..raw_headers.len() {
        if raw_headers[i] == b'\n' && (i == 0 || raw_headers[i - 1] != b'\r') {
            return Err(Error::LoneNewLine(raw_headers[i - 1]));
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use alloy_sol_types::{SolValue, private::bytes};
    use lazy_static::lazy_static;
    use test_utils::read_email_from_file;
    use verifiable_dns::{
        DNSRecord, RecordType,
        verifiable_dns::{sign_record::sign_record, signer::Signer},
    };

    use super::*;
    use crate::test_utils::signed_email_fixture;

    fn sign_dns_fixture(dns_fixture: &SolDnsRecord) -> SolVerificationData {
        let verification_data_signed = sign_record(
            &Signer::default(),
            &DNSRecord {
                data: dns_fixture.data.clone(),
                name: dns_fixture.name.clone(),
                record_type: RecordType::TXT,
                ttl: dns_fixture.ttl,
            },
            0,
        );
        SolVerificationData {
            validUntil: verification_data_signed.valid_until,
            signature: verification_data_signed.signature.0.clone().into(),
            pubKey: verification_data_signed.pub_key.0.into(),
        }
    }

    fn calldata(
        email: &str,
        dns_record: &SolDnsRecord,
        verification_data: &SolVerificationData,
    ) -> Vec<u8> {
        UnverifiedEmail {
            email: email.into(),
            dnsRecord: dns_record.clone(),
            verificationData: verification_data.clone(),
        }
        .abi_encode()
    }

    lazy_static! {
        static ref DNS_FIXTURE: SolDnsRecord = SolDnsRecord {
            name: "google._domainkey.vlayer.xyz".into(),
            recordType: 16,
            data: "v=DKIM1; k=rsa; p=MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAoDLLSKLb3eyflXzeHwBz8qqg9mfpmMY+f1tp+HpwlEOeN5iHO0s4sCd2QbG2i/DJRzryritRnjnc4i2NJ/IJfU8XZdjthotcFUY6rrlFld20a13q8RYBBsETSJhYnBu+DMdIF9q3YxDtXRFNpFCpI1uIeA/x+4qQJm3KTZQWdqi/BVnbsBA6ZryQCOOJC3Ae0oodvz80yfEJUAi9hAGZWqRn+Mprlyu749uQ91pTOYCDCbAn+cqhw8/mY5WMXFqrw9AdfWrk+MwXHPVDWBs8/Hm8xkWxHOqYs9W51oZ/Je3WWeeggyYCZI9V+Czv7eF8BD/yF9UxU/3ZWZPM8EWKKQIDAQAB".into(),
            ttl: 0,
        };

        static ref SPOOFED_DOMAIN_DNS_FIXTURE: SolDnsRecord = SolDnsRecord {
            name: "google._domainkey.spoofed-vlayer.xyz".into(),
            ..DNS_FIXTURE.clone()
        };

        static ref VERIFICATION_DATA: SolVerificationData = sign_dns_fixture(&DNS_FIXTURE);
    }

    #[test]
    fn passes_for_valid_email() -> anyhow::Result<()> {
        let email = signed_email_fixture();
        let calldata = calldata(&email, &DNS_FIXTURE, &VERIFICATION_DATA);

        assert_eq!(
            parse_and_verify(&calldata)?,
            Email {
                from: "ivan@vlayer.xyz".into(),
                to: "Ivan Rukhavets <ivanruch@gmail.com>".into(),
                subject: Some("Is dinner ready?".into(),),
                body: "Foo bar\r\n\r\n".into(),
            }
        );
        Ok(())
    }

    #[test]
    fn passes_for_valid_email_with_attachment() -> anyhow::Result<()> {
        let email = read_email_from_file("./testdata/email_with_attachment.eml");
        let calldata = calldata(&email, &DNS_FIXTURE, &VERIFICATION_DATA);

        assert_eq!(
            parse_and_verify(&calldata)?,
            Email {
                from: "piotr@vlayer.xyz".into(),
                to: "Ivan Rukhavets <ivan@vlayer.xyz>".into(),
                subject: Some("Email with attachment".into(),),
                body: "Hello,\r\ntake a look at the following remappings.\r\n\r\nBest Regards,\r\nPiotr\r\n\r\n".into(),
            }
        );
        Ok(())
    }

    #[test]
    fn fails_for_missing_dns_record() {
        let email = signed_email_fixture();
        let dns_record = SolDnsRecord {
            name: "".into(),
            recordType: 0,
            data: "".into(),
            ttl: 0,
        };
        let calldata = calldata(&email, &dns_record, &VERIFICATION_DATA);

        assert_eq!(
            parse_and_verify(&calldata).unwrap_err().to_string(),
            "Invalid UnverifiedEmail calldata: Unexpected DNS record type: 0. Supported types: TXT(16)".to_string()
        );
    }

    mod verifiable_dns_integration {
        use super::*;

        #[test]
        fn fails_for_invalid_vdns_signature() {
            let email = signed_email_fixture();
            let verification_data = SolVerificationData {
                signature: bytes!("1234"),
                ..VERIFICATION_DATA.clone()
            };
            let calldata = calldata(&email, &DNS_FIXTURE, &verification_data);

            assert_eq!(
                parse_and_verify(&calldata).unwrap_err().to_string(),
                "VDNS signature verification failed: Signature verification error".to_string()
            );
        }

        #[test]
        fn fails_for_missing_vdns_signature() {
            let email = signed_email_fixture();
            let verification_data = SolVerificationData {
                signature: Default::default(),
                pubKey: Default::default(),
                validUntil: 0,
            };
            let calldata = calldata(&email, &DNS_FIXTURE, &verification_data);

            assert_eq!(
                parse_and_verify(&calldata).unwrap_err().to_string(),
                "VDNS signature verification failed: Public key decoding error: ASN.1 error: ASN.1 DER message is incomplete: expected 1, actual 0 at DER byte 0".to_string()
            );
        }
    }

    mod verify_no_lone_separator {
        use super::*;

        #[test]
        fn pass() {
            let raw_headers = concat!("From: test@example.com\r\n", "Subject: Hello\r\n");
            let result = verify_no_fake_separator(raw_headers.as_bytes());
            assert!(result.is_ok());
        }

        #[test]
        fn fail() {
            let raw_headers = concat!("From: test@example.com\n", "Subject: Hello\r\n");
            let result = verify_no_fake_separator(raw_headers.as_bytes());
            assert_eq!(result.unwrap_err(), Error::LoneNewLine(b'm'));
        }
    }

    mod parse_headers_bytes {
        use super::*;

        #[test]
        fn parses_headers_for_valid_email() {
            let email = b"From: test@example.com\r\nSubject: Hello\r\n\r\nBody";
            assert_eq!(
                parse_headers_bytes(email).unwrap(),
                b"From: test@example.com\r\nSubject: Hello"
            )
        }

        #[test]
        fn rejects_missing_header_body_separator() {
            let email = b"From: test@example.com\r\nSubject: Hello\r\nBody";
            assert_eq!(parse_headers_bytes(email).unwrap_err(), Error::MissingBodySeparator);
        }
    }
}
