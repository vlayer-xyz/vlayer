use cfdkim::{validate_header, DKIMError, DkimPublicKey};
use mailparse::{MailHeader, MailHeaderMap, ParsedMail};
use slog::{o, Discard, Logger};
use verifiable_dns::DNSRecord;

pub use crate::errors::Error;
use crate::{dns::parse_dns_record, from_header};

const DKIM_SIGNATURE_HEADER: &str = "DKIM-Signature";

pub fn verify_email(email: &ParsedMail, record: &DNSRecord) -> Result<(), Error> {
    let dkim_headers = email.headers.get_all_headers(DKIM_SIGNATURE_HEADER);
    let dkim_public_key = parse_dns_record(&record.data)?;

    verify_email_contains_dkim_headers(&dkim_headers)?;
    verify_dkim_body_length_tag(&dkim_headers)?;
    verify_email_with_key(email, dkim_public_key)?;
    verify_dkim_header_dns_consistency(&dkim_headers, record)?;

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

fn verify_email_contains_dkim_headers(headers: &[&MailHeader<'_>]) -> Result<(), DKIMError> {
    if headers.is_empty() {
        return Err(DKIMError::SignatureSyntaxError("No DKIM-Signature header".into()));
    }
    Ok(())
}

fn verify_dkim_body_length_tag(headers: &[&MailHeader<'_>]) -> Result<(), DKIMError> {
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

fn verify_dkim_header_dns_consistency(
    headers: &[&MailHeader<'_>],
    record: &DNSRecord,
) -> Result<(), Error> {
    for header in headers {
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
    use mailparse::parse_header;

    use super::*;

    mod verify_email_contains_dkim_headers {
        use super::*;

        #[test]
        fn passes_for_no_empty_headers() {
            let header = b"DKIM-Signature: v=1; a=; c=; d=; s=; t=; h=From; bh=; b=";
            let headers = [&parse_header(header).unwrap().0];

            assert!(verify_email_contains_dkim_headers(&headers).is_ok());
        }


        #[test]
        fn fails_for_no_headers() {
            assert_eq!(
                verify_email_contains_dkim_headers(&[] as &[&MailHeader]).unwrap_err(),
                DKIMError::SignatureSyntaxError("No DKIM-Signature header".into())
            );
        }
    }

    mod verify_dkim_body_length_tag {
        use super::*;

        #[test]
        fn passes_for_header_without_l_tag() {
            let header = b"DKIM-Signature: v=1; a=; c=; d=; s=; t=; h=From; bh=; b=";
            let headers = [&parse_header(header).unwrap().0];

            assert!(verify_dkim_body_length_tag(&headers).is_ok());
        }

        #[test]
        fn fails_for_header_with_l_tag() {
            let header = b"DKIM-Signature: v=1; a=; c=; d=; s=; t=; h=From; bh=; b=; l=100";
            let headers = [&parse_header(header).unwrap().0];

            assert_eq!(
                verify_dkim_body_length_tag(&headers).unwrap_err(),
                DKIMError::SignatureSyntaxError(
                    "DKIM-Signature header contains body length tag (l=)".into()
                )
            );
        }
    }

    mod verify_dkim_header_dns_consistency {
        use super::*;

        fn record_with_name(name: &str) -> DNSRecord {
            DNSRecord {
                name: name.to_string(),
                ..Default::default()
            }
        }

        #[test]
        fn success() {
            let header =
                b"DKIM-Signature: v=1; a=; c=; d=example.com; s=selector1; h=From; bh=; b=";
            let headers = [&parse_header(header).unwrap().0];
            let record = record_with_name("selector1._domainkey.example.com");

            assert!(verify_dkim_header_dns_consistency(&headers, &record).is_ok());
        }

        #[test]
        fn fails_for_different_selector() {
            let header =
                b"DKIM-Signature: v=1; a=; c=; d=example.com; s=selector1; h=From; bh=; b=";
            let headers = [&parse_header(header).unwrap().0];
            let record = record_with_name("selector2._domainkey.example.com");

            assert_eq!(
                verify_dkim_header_dns_consistency(&headers, &record)
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
            let header =
                b"DKIM-Signature: v=1; a=; c=; d=example.com; s=selector1; h=From; bh=; b=";
            let headers = [&parse_header(header).unwrap().0];
            let record = record_with_name("selector1._domainkey.otherdomain.com");

            assert_eq!(
                verify_dkim_header_dns_consistency(&headers, &record)
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
