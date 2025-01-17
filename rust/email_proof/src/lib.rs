mod dkim;
mod dns;
mod email;
mod email_address;
mod errors;
mod from_header;
#[cfg(test)]
mod test_utils;

pub use email::sol::{SolDnsRecord, SolVerificationData, UnverifiedEmail};

pub use crate::{email::Email, errors::Error};

pub fn parse_and_verify(calldata: &[u8]) -> Result<Email, Error> {
    let (raw_email, dns_record, _) =
        UnverifiedEmail::parse_calldata(calldata).map_err(Error::Calldata)?;

    let email = mailparse::parse_mail(&raw_email).map_err(Error::EmailParse)?;

    let from_domain = from_header::extract_from_domain(&email)?;

    dkim::verify_email(email, &from_domain, dns::parse_dns_record(&dns_record.data)?)
        .map_err(Error::DkimVerification)?
        .try_into()
        .map_err(Error::EmailParse)
}

#[cfg(test)]
mod test {
    use alloy_sol_types::SolValue;
    use lazy_static::lazy_static;

    use super::*;
    use crate::test_utils::{read_file, signed_email_fixture, unsigned_email_fixture};

    lazy_static! {
        static ref DNS_FIXTURE: SolDnsRecord = SolDnsRecord {
            name: "_domainkey.vlayer.xyz".into(),
            recordType: "TXT".into(),
            data: "v=DKIM1; k=rsa; p=MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAoDLLSKLb3eyflXzeHwBz8qqg9mfpmMY+f1tp+HpwlEOeN5iHO0s4sCd2QbG2i/DJRzryritRnjnc4i2NJ/IJfU8XZdjthotcFUY6rrlFld20a13q8RYBBsETSJhYnBu+DMdIF9q3YxDtXRFNpFCpI1uIeA/x+4qQJm3KTZQWdqi/BVnbsBA6ZryQCOOJC3Ae0oodvz80yfEJUAi9hAGZWqRn+Mprlyu749uQ91pTOYCDCbAn+cqhw8/mY5WMXFqrw9AdfWrk+MwXHPVDWBs8/Hm8xkWxHOqYs9W51oZ/Je3WWeeggyYCZI9V+Czv7eF8BD/yF9UxU/3ZWZPM8EWKKQIDAQAB".into(),
            validUntil: 0,
        };
        static ref VERIFICATION_DATA: SolVerificationData = SolVerificationData {
            signature: Default::default(),
            pubKey: Default::default(),
            validUntil: 0,
        };
    }
    #[test]
    fn passes_for_valid_email() -> anyhow::Result<()> {
        let email = String::from_utf8(signed_email_fixture())?;
        let calldata = UnverifiedEmail {
            email,
            dnsRecord: DNS_FIXTURE.clone(),
            verificationData: VERIFICATION_DATA.clone(),
        }
        .abi_encode();

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
    fn fails_for_missing_signature() -> anyhow::Result<()> {
        let email = String::from_utf8(unsigned_email_fixture())?;
        let calldata = UnverifiedEmail {
            email,
            dnsRecord: DNS_FIXTURE.clone(),
            verificationData: VERIFICATION_DATA.clone(),
        }
        .abi_encode();
        assert_eq!(
            parse_and_verify(&calldata).unwrap_err().to_string(),
            "Error verifying DKIM: signature syntax error: No DKIM-Signature header".to_string()
        );
        Ok(())
    }

    #[test]
    fn fails_for_mismatching_body() -> anyhow::Result<()> {
        let email = String::from_utf8(read_file("./testdata/signed_email_modified_body.txt"))?;
        let calldata = UnverifiedEmail {
            email,
            dnsRecord: DNS_FIXTURE.clone(),
            verificationData: VERIFICATION_DATA.clone(),
        }
        .abi_encode();
        assert_eq!(
            parse_and_verify(&calldata).unwrap_err().to_string(),
            "Error verifying DKIM: body hash did not verify".to_string()
        );
        Ok(())
    }

    #[test]
    fn fails_for_missing_dns_record() -> anyhow::Result<()> {
        let email = String::from_utf8(signed_email_fixture())?;
        let calldata = UnverifiedEmail {
            email,
            dnsRecord: SolDnsRecord {
                data: "".into(),
                ..DNS_FIXTURE.clone()
            },
            verificationData: VERIFICATION_DATA.clone(),
        }
        .abi_encode();
        assert_eq!(
            parse_and_verify(&calldata).unwrap_err().to_string(),
            "Invalid DKIM public key record: p tag missing".to_string()
        );
        Ok(())
    }

    #[test]
    fn fails_for_mismatching_signer_and_sender_domain() -> anyhow::Result<()> {
        let email = String::from_utf8(read_file("./testdata/signed_email_different_domains.txt"))?;
        let calldata = UnverifiedEmail {
            email,
            dnsRecord: SolDnsRecord {
                data: "v=DKIM1; k=rsa; p=MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA3gWcOhCm99qzN+h7/2+LeP3CLsJkQQ4EP/2mrceXle5pKq8uZmBl1U4d2Vxn4w+pWFANDLmcHolLboESLFqEL5N6ae7u9b236dW4zn9AFkXAGenTzQEeif9VUFtLAZ0Qh2eV7OQgz/vPj5IaNqJ7h9hpM9gO031fe4v+J0DLCE8Rgo7hXbNgJavctc0983DaCDQaznHZ44LZ6TtZv9TBs+QFvsy4+UCTfsuOtHzoEqOOuXsVXZKLP6B882XbEnBpXEF8QzV4J26HiAJFUbO3mAqZL2UeKC0hhzoIZqZXNG0BfuzOF0VLpDa18GYMUiu+LhEJPJO9D8zhzvQIHNrpGwIDAQAB".into(),
                ..DNS_FIXTURE.clone()
            },
            verificationData: VERIFICATION_DATA.clone(),
        }
        .abi_encode();

        assert_eq!(
            parse_and_verify(&calldata).unwrap_err().to_string(),
            "Error verifying DKIM: signature did not verify".to_string()
        );
        Ok(())
    }

    #[test]
    fn fails_when_from_address_is_from_subdomain() -> anyhow::Result<()> {
        let email = String::from_utf8(read_file("./testdata/signed_email_from_subdomain.txt"))?;
        let calldata = UnverifiedEmail {
            email,
            dnsRecord: DNS_FIXTURE.clone(),
            verificationData: VERIFICATION_DATA.clone(),
        }
        .abi_encode();

        assert_eq!(
            parse_and_verify(&calldata).unwrap_err().to_string(),
            "Error verifying DKIM: signature did not verify".to_string()
        );

        Ok(())
    }

    #[test]
    fn fails_when_dkim_signer_address_is_from_subdomain() -> anyhow::Result<()> {
        let email = String::from_utf8(read_file("./testdata/signed_email_from_subdomain.txt"))?;
        let calldata = UnverifiedEmail {
            email,
            dnsRecord: DNS_FIXTURE.clone(),
            verificationData: VERIFICATION_DATA.clone(),
        }
        .abi_encode();

        assert_eq!(
            parse_and_verify(&calldata).unwrap_err().to_string(),
            "Error verifying DKIM: signature did not verify".to_string()
        );

        Ok(())
    }
}
