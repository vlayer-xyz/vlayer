use cfdkim::{DKIMError, DKIMResult, DkimPublicKey};
use mailparse::ParsedMail;
use slog::{Discard, Logger, o};

pub use crate::errors::Error;
use crate::from_header;

pub fn verify_signature(email: &ParsedMail, key: DkimPublicKey) -> Result<(), Error> {
    let result = dkim_key_verification(email, key)?;
    interpret_dkim_verification_result(&result)
}

fn dkim_key_verification(email: &ParsedMail, key: DkimPublicKey) -> Result<DKIMResult, Error> {
    let from_domain = from_header::extract_from_domain(email)?;
    let result =
        cfdkim::verify_email_with_key(&Logger::root(Discard, o!()), &from_domain, email, key)?;
    Ok(result)
}

fn interpret_dkim_verification_result(result: &DKIMResult) -> Result<(), Error> {
    if result.with_detail().starts_with("pass") {
        Ok(())
    } else if let Some(err) = result.error() {
        Err(Error::DkimVerification(err))
    } else {
        Err(Error::DkimVerification(DKIMError::SignatureDidNotVerify))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod interpret_dkim_verification_result {
        use cfdkim::{DKIMError, DKIMResult, canonicalization::Type};
        use lazy_static::lazy_static;

        use super::*;

        lazy_static! {
            static ref DOMAIN_NAME: String = "example.com".to_string();
        }

        #[test]
        fn returns_ok_for_pass_result() {
            let result = DKIMResult::pass(DOMAIN_NAME.clone(), Type::Simple, Type::Simple);
            let res = interpret_dkim_verification_result(&result);
            assert!(res.is_ok());
        }

        #[test]
        fn returns_error_if_dkim_error_present() {
            let result = DKIMResult::fail(DKIMError::SignatureExpired, DOMAIN_NAME.clone());

            assert_eq!(
                interpret_dkim_verification_result(&result).unwrap_err(),
                Error::DkimVerification(DKIMError::SignatureExpired)
            );
        }

        #[test]
        fn returns_signature_did_not_verify_if_no_error_and_summary_not_pass() {
            let result = DKIMResult::neutral(DOMAIN_NAME.clone());

            assert_eq!(
                interpret_dkim_verification_result(&result).unwrap_err(),
                Error::DkimVerification(DKIMError::SignatureDidNotVerify)
            )
        }
    }

    mod verify_signature {
        use lazy_static::lazy_static;
        use mailparse::parse_mail;

        use super::*;
        use crate::{dns::extract_public_key, test_utils::read_email_from_file};

        lazy_static! {
            static ref DNS_RECORD_DATA: String = "v=DKIM1; k=rsa; p=MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAoDLLSKLb3eyflXzeHwBz8qqg9mfpmMY+f1tp+HpwlEOeN5iHO0s4sCd2QbG2i/DJRzryritRnjnc4i2NJ/IJfU8XZdjthotcFUY6rrlFld20a13q8RYBBsETSJhYnBu+DMdIF9q3YxDtXRFNpFCpI1uIeA/x+4qQJm3KTZQWdqi/BVnbsBA6ZryQCOOJC3Ae0oodvz80yfEJUAi9hAGZWqRn+Mprlyu749uQ91pTOYCDCbAn+cqhw8/mY5WMXFqrw9AdfWrk+MwXHPVDWBs8/Hm8xkWxHOqYs9W51oZ/Je3WWeeggyYCZI9V+Czv7eF8BD/yF9UxU/3ZWZPM8EWKKQIDAQAB".to_string();
            static ref PUBLIC_KEY: DkimPublicKey = {
                extract_public_key(&DNS_RECORD_DATA).unwrap()
            };
        }

        fn assert_signature_did_not_verify(file_path: &str) {
            let email = read_email_from_file(file_path);
            let parsed_email = parse_mail(email.as_bytes()).unwrap();
            let result =
                verify_signature(&parsed_email, extract_public_key(&DNS_RECORD_DATA).unwrap());

            assert_eq!(
                result.unwrap_err().to_string(),
                "Error verifying DKIM: signature did not verify".to_string()
            );
        }

        #[test]
        fn fails_when_from_address_is_from_subdomain() {
            assert_signature_did_not_verify("./testdata/signed_email_from_subdomain.txt");
        }

        #[test]
        fn fails_for_mismatching_signer_and_sender_domain() {
            assert_signature_did_not_verify("./testdata/signed_email_different_domains.txt");
        }

        #[test]
        fn fails_for_missing_signature() {
            assert_signature_did_not_verify("./testdata/email.txt");
        }

        #[test]
        fn fails_for_mismatching_body() {
            let email = read_email_from_file("./testdata/signed_email_modified_body.txt");
            let parsed_email = parse_mail(email.as_bytes()).unwrap();
            let result =
                verify_signature(&parsed_email, extract_public_key(&DNS_RECORD_DATA).unwrap());

            assert_eq!(
                result.unwrap_err().to_string(),
                "Error verifying DKIM: body hash did not verify".to_string()
            );
        }
    }
}
