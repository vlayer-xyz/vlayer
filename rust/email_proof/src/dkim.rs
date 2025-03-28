use cfdkim::{validate_header, DKIMError, DkimPublicKey};
use mailparse::{MailHeaderMap, ParsedMail};
use slog::{o, Discard, Logger};
use verifiable_dns::DNSRecord;

pub use crate::errors::Error;
use crate::{dns::parse_dns_record, from_header};

const DKIM_SIGNATURE_HEADER: &str = "DKIM-Signature";

pub fn verify_email(email: &ParsedMail, record: &DNSRecord) -> Result<(), Error> {
    verify_dkim_headers(email)?;
    let dkim_public_key = parse_dns_record(&record.data)?;
    verify_email_with_key(email, dkim_public_key)?;
    verify_dkim_header_dns_consistency(email, record)?;

    Ok(())
}

fn verify_dkim_headers(email: &ParsedMail) -> Result<(), DKIMError> {
    verify_email_contains_dkim_headers(email)?;
    verify_dkim_body_length_tag(email)?;

    Ok(())
}

fn verify_email_with_key(email: &ParsedMail, key: DkimPublicKey) -> Result<(), Error> {
    let from_domain = from_header::extract_from_domain(email)?;

    let result =
        cfdkim::verify_email_with_key(&Logger::root(Discard, o!()), &from_domain, email, key)?;

    if result.with_detail().starts_with("pass") {
        Ok(())
    } else if let Some(err) = result.error() {
        Err(Error::DkimVerification(err))
    } else {
        Err(Error::DkimVerification(DKIMError::SignatureDidNotVerify))
    }
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

fn verify_dkim_header_dns_consistency(email: &ParsedMail, record: &DNSRecord) -> Result<(), Error> {
    for header in email.headers.get_all_headers(DKIM_SIGNATURE_HEADER) {
        let dkim_header = validate_header(&String::from_utf8_lossy(header.get_value_raw()))?;

        let selector = dkim_header.get_tag("s").ok_or_else(|| {
            DKIMError::SignatureSyntaxError("Missing selector tag (s=) in DKIM-Signature".into())
        })?;
        let domain = dkim_header.get_tag("d").ok_or_else(|| {
            DKIMError::SignatureSyntaxError("Missing domain tag (d=) in DKIM-Signature".into())
        })?;

        let expected = normalize_dns_name(&format!("{selector}._domainkey.{domain}"));
        let actual = normalize_dns_name(&record.name);

        if expected != actual {
            return Err(Error::DomainMismatch(expected, actual));
        }
    }

    Ok(())
}

fn normalize_dns_name(name: &str) -> String {
    name.trim().trim_end_matches('.').to_lowercase()
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

    mod verify_dkim_header_dns_consistency {
        use mailparse::parse_mail;

        use super::*;

        fn record_with_name(name: &str) -> DNSRecord {
            DNSRecord {
                name: name.to_string(),
                ..Default::default()
            }
        }

        #[test]
        fn success() {
            let dkim_header =
                ("DKIM-Signature", "v=1; a=; c=; d=example.com; s=selector1; h=From; bh=; b=");
            let mime_email = email_with_headers(&[dkim_header]).into_bytes();
            let email = parse_mail(&mime_email).unwrap();
            let record = record_with_name("selector1._domainkey.example.com");

            assert!(verify_dkim_header_dns_consistency(&email, &record).is_ok());
        }

        #[test]
        fn fails_for_different_selector() {
            let dkim_header =
                ("DKIM-Signature", "v=1; a=; c=; d=example.com; s=selector1; h=From; bh=; b=");
            let mime_email = email_with_headers(&[dkim_header]).into_bytes();
            let email = parse_mail(&mime_email).unwrap();
            let record = record_with_name("selector2._domainkey.example.com");

            assert_eq!(
                verify_dkim_header_dns_consistency(&email, &record)
                    .unwrap_err()
                    .to_string(),
                Error::DomainMismatch(
                    "selector1._domainkey.example.com".into(),
                    "selector2._domainkey.example.com".into()
                )
                .to_string()
            );
        }

        #[test]
        fn fails_for_different_domain() {
            let dkim_header =
                ("DKIM-Signature", "v=1; a=; c=; d=example.com; s=selector1; h=From; bh=; b=");
            let mime_email = email_with_headers(&[dkim_header]).into_bytes();
            let email = parse_mail(&mime_email).unwrap();
            let record = record_with_name("selector1._domainkey.otherdomain.com");

            assert_eq!(
                verify_dkim_header_dns_consistency(&email, &record)
                    .unwrap_err()
                    .to_string(),
                Error::DomainMismatch(
                    "selector1._domainkey.example.com".into(),
                    "selector1._domainkey.otherdomain.com".into()
                )
                .to_string()
            );
        }
    }
}
