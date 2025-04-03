use cfdkim::{validate_header, DKIMError};
use mailparse::MailHeader;
use verifiable_dns::DNSRecord;

pub use crate::errors::Error;

pub(crate) mod verify_signature;

pub fn verify_email_contains_dkim_headers(headers: &[&MailHeader<'_>]) -> Result<(), DKIMError> {
    if headers.is_empty() {
        return Err(DKIMError::SignatureSyntaxError("No DKIM-Signature header".into()));
    }
    Ok(())
}

pub fn verify_dkim_body_length_tag(headers: &[&MailHeader<'_>]) -> Result<(), DKIMError> {
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

pub fn verify_dkim_header_dns_consistency(
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
                verify_dkim_header_dns_consistency(&headers, &record).unwrap_err(),
                Error::DomainMismatch(
                    "selector1._domainkey.example.com".into(),
                    "selector2._domainkey.example.com".into()
                )
            );
        }

        #[test]
        fn fails_for_different_domain() {
            let header =
                b"DKIM-Signature: v=1; a=; c=; d=example.com; s=selector1; h=From; bh=; b=";
            let headers = [&parse_header(header).unwrap().0];
            let record = record_with_name("selector1._domainkey.otherdomain.com");

            assert_eq!(
                verify_dkim_header_dns_consistency(&headers, &record).unwrap_err(),
                Error::DomainMismatch(
                    "selector1._domainkey.example.com".into(),
                    "selector1._domainkey.otherdomain.com".into()
                )
            );
        }
    }

    mod normalize_dns_name {
        use super::*;

        #[test]
        fn normalizes() {
            let name = "Example.com. ";
            let normalized = normalize_dns_name(name);
            assert_eq!(normalized, "example.com");
        }
    }
}
