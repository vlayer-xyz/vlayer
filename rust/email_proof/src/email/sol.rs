use alloy_sol_types::{Error, SolValue};

use crate::email::Email;

mod private {
    use alloy_sol_types::sol;

    sol!("../../contracts/src/EmailProof.sol");
}

pub(crate) use private::{UnverifiedEmail, VerifiedEmail as SolEmail};

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
    pub(crate) fn parse_calldata(calldata: &[u8]) -> Result<(Vec<u8>, Vec<String>), Error> {
        let unverified_email = UnverifiedEmail::abi_decode(calldata, true)?;
        let raw_email = unverified_email.email.into_bytes();
        let dns_records = unverified_email.dnsRecords;
        Ok((raw_email, dns_records))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod unverified_email {
        use super::*;

        #[test]
        fn test_parse_calldata() {
            let input_email = UnverifiedEmail {
                email: "email".into(),
                dnsRecords: vec!["dns".into()],
            };
            let bytecode = UnverifiedEmail::abi_encode(&input_email);

            let (raw_email, dns_records) = UnverifiedEmail::parse_calldata(&bytecode).unwrap();
            assert_eq!(raw_email, "email".as_bytes());
            assert_eq!(dns_records, input_email.dnsRecords);
        }

        #[test]
        fn test_error_if_parse_calldata_fails() {
            let result = UnverifiedEmail::parse_calldata(&[0x00]);
            assert_eq!(result, Err(Error::Overrun));
        }
    }
}
