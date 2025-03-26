use cfdkim::{DKIMError, DkimPublicKey, validate_header, verify_email_with_key};
use mailparse::{MailHeaderMap, ParsedMail};
use slog::{Discard, Logger, o};

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
    verify_dkim_body_length_tag(email)?;

    Ok(())
}

fn verify_email_contains_dkim_headers(email: &ParsedMail) -> Result<(), DKIMError> {
    let dkim_headers = email.headers.get_all_headers(DKIM_SIGNATURE_HEADER);
    if dkim_headers.is_empty() {
        return Err(DKIMError::SignatureSyntaxError("No DKIM-Signature header".into()));
    }
    Ok(())
}

fn verify_dkim_body_length_tag(email: &ParsedMail) -> Result<(), DKIMError> {
    let headers = email.headers.get_all_headers(DKIM_SIGNATURE_HEADER);

    for h in headers {
        let value = String::from_utf8_lossy(h.get_value_raw());
        let dkim_header = validate_header(&value)?;

        if dkim_header.get_tag("l").is_some() {
            return Err(DKIMError::SignatureSyntaxError(
                "DKIM-Signature header contains body length tag (l=)".into(),
            ));
        }
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

        #[test]
        fn fails_for_header_with_l_tag() {
            let dkim_header = ("DKIM-Signature", "v=1; a=; c=; d=; s=; t=; h=From; bh=; b=; l=100");
            let mime_email = email_with_headers(&[dkim_header]).into_bytes();
            let email = mailparse::parse_mail(&mime_email).unwrap();

            assert_eq!(
                verify_dkim_headers(&email).unwrap_err(),
                DKIMError::SignatureSyntaxError(
                    "DKIM-Signature header contains body length tag (l=)".into()
                )
            );
        }

        #[test]
        fn fails_for_headers_with_one_of_them_having_l_tag() {
            let dkim_headers = [
                ("DKIM-Signature", "v=1; a=; c=; d=; s=; t=; h=From; bh=; b=;"),
                ("DKIM-Signature", "v=1; a=; c=; d=; s=; t=; h=From; bh=; b=; l=100"),
            ];
            let mime_email = email_with_headers(&dkim_headers).into_bytes();
            let email = mailparse::parse_mail(&mime_email).unwrap();

            assert_eq!(
                verify_dkim_headers(&email).unwrap_err(),
                DKIMError::SignatureSyntaxError(
                    "DKIM-Signature header contains body length tag (l=)".into()
                )
            );
        }
    }
}
