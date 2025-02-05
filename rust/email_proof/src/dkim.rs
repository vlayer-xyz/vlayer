use cfdkim::{verify_email_with_key, DKIMError, DkimPublicKey};
use mailparse::{MailHeaderMap, ParsedMail};
use slog::{o, Discard, Logger};

const DKIM_SIGNATURE_HEADER: &str = "DKIM-Signature";
pub fn verify_email<'a>(
    email: ParsedMail<'a>,
    from_domain: &str,
    dkim_public_key: DkimPublicKey,
) -> Result<ParsedMail<'a>, DKIMError> {
    verify_dkim_headers(&email)?;

    let logger = Logger::root(Discard, o!());
    let result = verify_email_with_key(&logger, from_domain, &email, dkim_public_key)?;

    match result {
        result if result.with_detail().starts_with("pass") => Ok(email),
        result if result.error().is_some() => Err(result.error().unwrap()),
        _ => Err(DKIMError::SignatureDidNotVerify),
    }
}
fn verify_dkim_headers(email: &ParsedMail) -> Result<(), DKIMError> {
    verify_email_contains_dkim_headers(email)?;

    Ok(())
}

fn verify_email_contains_dkim_headers(email: &ParsedMail) -> Result<(), DKIMError> {
    let dkim_headers = email.headers.get_all_headers(DKIM_SIGNATURE_HEADER);
    if dkim_headers.is_empty() {
        return Err(DKIMError::SignatureSyntaxError("No DKIM-Signature header".into()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use lazy_static::lazy_static;

    use super::*;
    use crate::test_utils::*;

    lazy_static! {
        static ref DEFAULT_HEADERS: Vec<(&'static str, &'static str)> = vec![
            ("From", "alice@vlayer.xyz"),
            ("To", "bob@vlayer.xyz"),
            ("Date", "Thu, 15 Aug 2019 14:54:37 +0900"),
            ("Subject", "Test email")
        ];
    }
    const DEFAULT_BODY: &str = "Hello World";

    fn email_with_headers(additional_headers: &[(&str, &str)]) -> String {
        let mut headers = DEFAULT_HEADERS.clone();
        headers.extend(additional_headers);
        build_mime_email(headers, DEFAULT_BODY)
    }

    mod verify_dkim_headers {

        use super::*;

        #[test]
        fn passes_for_headers_without_l_tag() {
            let dkim_header = ("DKIM-Signature", "v=1; a=; c=; d=; s=; t=; h=From; bh=; b=");
            let mime_email = email_with_headers(&[dkim_header]).into_bytes();
            let email = mailparse::parse_mail(&mime_email).unwrap();

            assert!(verify_dkim_headers(&email).is_ok());
        }

        #[test]
        fn fails_for_email_without_dkim_headers() {
            let mime_email = email_with_headers(&[]).into_bytes();
            let email = mailparse::parse_mail(&mime_email).unwrap();

            assert_eq!(
                verify_dkim_headers(&email).unwrap_err(),
                DKIMError::SignatureSyntaxError("No DKIM-Signature header".into())
            );
        }
    }
}
