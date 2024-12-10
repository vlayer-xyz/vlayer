mod dkim;
mod dns;
mod email;
mod errors;
mod from_header;
#[cfg(test)]
mod test_utils;

pub use email::sol::UnverifiedEmail;

pub use crate::{email::Email, errors::Error};

pub fn parse_and_verify(calldata: &[u8]) -> Result<Email, Error> {
    let (raw_email, dns_records) =
        UnverifiedEmail::parse_calldata(calldata).map_err(Error::Calldata)?;

    let email = mailparse::parse_mail(&raw_email).map_err(Error::EmailParse)?;
    let dns_record = dns_records
        .first()
        .ok_or(Error::InvalidDkimRecord("No DNS records provided".into()))?;

    let from_domain = from_header::extract_from_domain(&email)?;

    dkim::verify_email(email, &from_domain, dns::parse_dns_record(dns_record)?)
        .map_err(Error::DkimVerification)?
        .try_into()
        .map_err(Error::EmailParse)
}

#[cfg(test)]
mod test {
    use alloy_sol_types::SolValue;
    use lazy_static::lazy_static;

    use super::*;
    use crate::test_utils::{signed_email_fixture, unsigned_email_fixture};

    lazy_static! {
        static ref DNS_FIXTURE: Vec<String> = vec![
            "v=DKIM1; k=rsa; p=MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA3gWcOhCm99qzN+h7/2+LeP3CLsJkQQ4EP/2mrceXle5pKq8uZmBl1U4d2Vxn4w+pWFANDLmcHolLboESLFqEL5N6ae7u9b236dW4zn9AFkXAGenTzQEeif9VUFtLAZ0Qh2eV7OQgz/vPj5IaNqJ7h9hpM9gO031fe4v+J0DLCE8Rgo7hXbNgJavctc0983DaCDQaznHZ44LZ6TtZv9TBs+QFvsy4+UCTfsuOtHzoEqOOuXsVXZKLP6B882XbEnBpXEF8QzV4J26HiAJFUbO3mAqZL2UeKC0hhzoIZqZXNG0BfuzOF0VLpDa18GYMUiu+LhEJPJO9D8zhzvQIHNrpGwIDAQAB".into()
        ];

    }
    #[test]
    fn passes_for_valid_email() -> anyhow::Result<()> {
        let email = String::from_utf8(signed_email_fixture())?;
        let calldata = UnverifiedEmail {
            email,
            dnsRecords: DNS_FIXTURE.to_vec(),
        }
        .abi_encode();
        assert_eq!(
            parse_and_verify(&calldata)?,
            Email {
                from: "ivan@vlayer.xyz".into(),
                to: "Ivan Rukhavets <ivanruch@gmail.com>".into(),
                subject: Some("Is dinner ready?".into(),),
                body: "Foo bar\n\n".into(),
            }
        );
        Ok(())
    }

    #[test]
    fn fails_for_missing_signature() -> anyhow::Result<()> {
        let email = String::from_utf8(unsigned_email_fixture())?;
        let calldata = UnverifiedEmail {
            email,
            dnsRecords: DNS_FIXTURE.to_vec(),
        }
        .abi_encode();
        assert_eq!(
            parse_and_verify(&calldata).unwrap_err().to_string(),
            "Error verifying DKIM: signature syntax error: No DKIM-Signature header".to_string()
        );
        Ok(())
    }

    #[test]
    fn fails_for_missing_public_key() -> anyhow::Result<()> {
        let email = String::from_utf8(signed_email_fixture())?;
        let calldata = UnverifiedEmail {
            email,
            dnsRecords: vec![],
        }
        .abi_encode();
        assert_eq!(
            parse_and_verify(&calldata).unwrap_err().to_string(),
            "Invalid DKIM public key record: No DNS records provided".to_string()
        );
        Ok(())
    }
}
