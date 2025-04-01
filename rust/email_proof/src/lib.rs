mod dkim;
mod dns;
mod email;
mod email_address;
mod errors;
mod from_header;
#[cfg(test)]
mod test_utils;

use dkim::{
    verify_dkim_body_length_tag, verify_dkim_header_dns_consistency,
    verify_email_contains_dkim_headers, verify_email_with_key,
};
use dns::parse_dns_record;
pub use email::sol::{SolDnsRecord, SolVerificationData, UnverifiedEmail};
use mailparse::{parse_mail, MailHeaderMap};

pub use crate::{email::Email, errors::Error};

const DKIM_SIGNATURE_HEADER: &str = "DKIM-Signature";

pub fn parse_and_verify(calldata: &[u8]) -> Result<Email, Error> {
    let (raw_email, dns_record, verification_data) = UnverifiedEmail::parse_calldata(calldata)?;
    let email = parse_mail(&raw_email)?;
    let dkim_public_key = parse_dns_record(&dns_record.data)?;
    let dkim_headers = email.headers.get_all_headers(DKIM_SIGNATURE_HEADER);
    let raw_headers = parse_headers_bytes(email.raw_bytes)?;

    verify_headers_crlfs(raw_headers)?;
    dns_record.verify(&verification_data)?;
    verify_email_contains_dkim_headers(&dkim_headers)?;
    verify_dkim_body_length_tag(&dkim_headers)?;
    verify_email_with_key(&email, dkim_public_key)?;
    verify_dkim_header_dns_consistency(&dkim_headers, &dns_record)?;

    Ok(email.try_into()?)
}

fn parse_headers_bytes(raw_email: &[u8]) -> Result<&[u8], Error> {
    let email_str = std::str::from_utf8(raw_email).expect("Email already verified");

    let header_end = email_str
        .find("\r\n\r\n")
        .ok_or(Error::MissingBodySeparator)?;

    let headers_part = &email_str[..header_end];
    Ok(headers_part.as_bytes())
}

fn verify_headers_crlfs(raw_headers: &[u8]) -> Result<(), Error> {
    for window in raw_headers.windows(3) {
        match window {
            [b'\r', b'\n', next]
                if !is_valid_header_start(*next) && !is_valid_folded_line_start(*next) =>
            {
                return Err(Error::InvalidNewLineSeparator(*next));
            }
            _ => {}
        }
    }
    Ok(())
}

// https://datatracker.ietf.org/doc/html/rfc5322#section-2.2
// A header field name must be composed of printable US-ASCII characters (i.e.,
// characters that have values between 33 and 126, inclusive), except colon.
fn is_valid_header_start(c: u8) -> bool {
    c != b':' && (33..=126).contains(&c)
}

// https://datatracker.ietf.org/doc/html/rfc5322#section-2.2.3
// https://datatracker.ietf.org/doc/html/rfc5322#section-3.2.2
// Header fields are single lines of text, but the field body can be split
// into multiple lines using "folding". Folding allows inserting a CRLF
// before any whitespace to handle line length limits.
const fn is_valid_folded_line_start(curr: u8) -> bool {
    curr == b' ' || curr == b'\t'
}

#[cfg(test)]
mod test {
    use alloy_sol_types::{private::bytes, SolValue};
    use lazy_static::lazy_static;
    use test_utils::read_email_from_file;
    use verifiable_dns::{
        verifiable_dns::{sign_record::sign_record, signer::Signer},
        DNSRecord, RecordType,
    };

    use super::*;
    use crate::test_utils::{signed_email_fixture, unsigned_email_fixture};

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
    fn passes_for_dns_record_with_missing_v_and_k_tags() -> anyhow::Result<()> {
        let dns_record = SolDnsRecord {
            data: "  p=MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAoDLLSKLb3eyflXzeHwBz8qqg9mfpmMY+f1tp+HpwlEOeN5iHO0s4sCd2QbG2i/DJRzryritRnjnc4i2NJ/IJfU8XZdjthotcFUY6rrlFld20a13q8RYBBsETSJhYnBu+DMdIF9q3YxDtXRFNpFCpI1uIeA/x+4qQJm3KTZQWdqi/BVnbsBA6ZryQCOOJC3Ae0oodvz80yfEJUAi9hAGZWqRn+Mprlyu749uQ91pTOYCDCbAn+cqhw8/mY5WMXFqrw9AdfWrk+MwXHPVDWBs8/Hm8xkWxHOqYs9W51oZ/Je3WWeeggyYCZI9V+Czv7eF8BD/yF9UxU/3ZWZPM8EWKKQIDAQAB".into(),
            ..DNS_FIXTURE.clone()
        };
        let verification_data = sign_dns_fixture(&dns_record);
        let calldata = calldata(&signed_email_fixture(), &dns_record, &verification_data);

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
    fn fails_for_missing_signature() {
        let email = unsigned_email_fixture();
        let calldata = calldata(&email, &DNS_FIXTURE, &VERIFICATION_DATA);

        assert_eq!(
            parse_and_verify(&calldata).unwrap_err().to_string(),
            "Error verifying DKIM: signature syntax error: No DKIM-Signature header".to_string()
        );
    }

    #[test]
    fn fails_for_mismatching_body() {
        let email = read_email_from_file("./testdata/signed_email_modified_body.txt");
        let calldata = calldata(&email, &DNS_FIXTURE, &VERIFICATION_DATA);

        assert_eq!(
            parse_and_verify(&calldata).unwrap_err().to_string(),
            "Error verifying DKIM: body hash did not verify".to_string()
        );
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

    #[test]
    fn fails_for_mismatching_signer_and_sender_domain() {
        let email = read_email_from_file("./testdata/signed_email_different_domains.txt");
        let calldata = calldata(&email, &DNS_FIXTURE, &VERIFICATION_DATA);

        assert_eq!(
            parse_and_verify(&calldata).unwrap_err().to_string(),
            "Error verifying DKIM: signature did not verify".to_string()
        );
    }

    #[test]
    fn fails_when_from_address_is_from_subdomain() {
        let email = read_email_from_file("./testdata/signed_email_from_subdomain.txt");
        let calldata = calldata(&email, &DNS_FIXTURE, &VERIFICATION_DATA);

        assert_eq!(
            parse_and_verify(&calldata).unwrap_err().to_string(),
            "Error verifying DKIM: signature did not verify".to_string()
        );
    }

    #[test]
    fn fails_when_dkim_signer_address_is_from_subdomain() {
        let email = read_email_from_file("./testdata/signed_email_from_subdomain.txt");
        let calldata = calldata(&email, &DNS_FIXTURE, &VERIFICATION_DATA);

        assert_eq!(
            parse_and_verify(&calldata).unwrap_err().to_string(),
            "Error verifying DKIM: signature did not verify".to_string()
        );
    }

    #[test]
    fn fails_for_dkim_signature_of_truncated_body() {
        let email = read_email_from_file("./testdata/signed_email_with_dkim_l_tag.eml");
        let calldata = calldata(&email, &DNS_FIXTURE, &VERIFICATION_DATA);

        assert_eq!(
            parse_and_verify(&calldata).unwrap_err().to_string(),
            "Error verifying DKIM: signature syntax error: DKIM-Signature header contains body length tag (l=)".to_string()
        );
    }

    mod verify_headers_crlfs {
        use super::*;

        fn verify(raw: &str) -> Result<(), Error> {
            verify_headers_crlfs(raw.as_bytes())
        }

        #[test]
        fn accepts_valid_simple_headers() {
            let email = concat!(
                "From: test@example.com\r\n",
                "Subject: Hello"
            );
            assert!(verify(email).is_ok());
        }

        #[test]
        fn accepts_folded_headers() {
            let email = concat!(
                "Subject: This is a long subject\r\n",
                " continuing here\r\n",
                "From: test@example.com",
            );
            assert!(verify(email).is_ok());
        }

        #[test]
        fn rejects_line_with_wrong_next_line_character() {
            let email = concat!(
                "From: test@example.com\r\n",
                ": invalid next line character"
            );
            assert_eq!(verify(email).unwrap_err(), Error::InvalidNewLineSeparator(b':'));
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
