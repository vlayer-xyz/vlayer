use cfdkim::{header::DKIMHeader, validate_header, DKIMError};
use mailparse::MailHeader;
use verifiable_dns::DNSRecord;

pub use crate::errors::Error;

pub(crate) mod verify_signature;

pub const fn verify_single_dkim_header<'a>(
    dkim_headers: &'a [&MailHeader<'a>],
) -> Result<&'a MailHeader<'a>, Error> {
    if dkim_headers.len() != 1 {
        return Err(Error::InvalidDkimHeaderCount(dkim_headers.len()));
    }
    Ok(dkim_headers[0])
}

pub fn verify_dkim_body_length_tag(dkim_headers: &[&MailHeader<'_>]) -> Result<(), DKIMError> {
    for header in dkim_headers {
        let value = String::from_utf8_lossy(header.get_value_raw());
        let dkim_header = validate_header(&value)?;

        if dkim_header.get_tag("l").is_some() {
            return Err(DKIMError::SignatureSyntaxError(
                "DKIM-Signature header contains body length tag (l=)".into(),
            ));
        }
    }

    Ok(())
}

pub fn verify_dns_consistency(
    dkim_headers: &[&MailHeader],
    record: &DNSRecord,
) -> Result<(), Error> {
    for header in dkim_headers {
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

pub fn verify_required_headers_signed(
    dkim_headers: &[&MailHeader],
    required_signed_headers: &[&str],
) -> Result<(), Error> {
    for header in dkim_headers {
        let value = String::from_utf8_lossy(header.get_value_raw());
        let dkim_header = validate_header(&value)?;
        let signed_headers = signed_headers(&dkim_header);

        for &required_field in required_signed_headers {
            if !signed_headers.contains(&required_field.to_lowercase()) {
                return Err(Error::MissingRequiredHeaderTag(required_field.to_string()));
            }
        }
    }
    Ok(())
}

fn signed_headers(dkim_header: &DKIMHeader) -> Vec<String> {
    dkim_header
        .get_required_tag("h")
        .split(':')
        .map(|s| s.trim().to_lowercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use mailparse::parse_header;

    use super::*;

    mod verify_single_dkim_header {
        use mailparse::parse_header;

        use super::*;

        #[test]
        fn passes_for_single_dkim_header() {
            let header = b"DKIM-Signature: v=1; a=; c=; d=; s=; t=; h=From; bh=; b=";
            let headers = [&parse_header(header).unwrap().0];

            let result = verify_single_dkim_header(&headers);
            assert!(result.is_ok());
        }

        #[test]
        fn fails_for_no_dkim_headers() {
            let headers: &[&MailHeader] = &[];

            let err = verify_single_dkim_header(headers).unwrap_err();
            assert_eq!(err, Error::InvalidDkimHeaderCount(headers.len()));
        }

        #[test]
        fn fails_for_multiple_dkim_headers() {
            let header1 = b"DKIM-Signature: v=1; a=; c=; d=; s=; t=; h=From; bh=; b=";
            let header2 = b"DKIM-Signature: v=1; a=; c=; d=; s=; t=; h=From; bh=; b=";

            let headers = [&parse_header(header1).unwrap().0, &parse_header(header2).unwrap().0];

            let err = verify_single_dkim_header(&headers).unwrap_err();
            assert_eq!(err, Error::InvalidDkimHeaderCount(headers.len()));
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

    mod verify_dns_consistency {
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

            assert!(verify_dns_consistency(&headers, &record).is_ok());
        }

        #[test]
        fn fails_for_different_selector() {
            let header =
                b"DKIM-Signature: v=1; a=; c=; d=example.com; s=selector1; h=From; bh=; b=";
            let headers = [&parse_header(header).unwrap().0];
            let record = record_with_name("selector2._domainkey.example.com");

            assert_eq!(
                verify_dns_consistency(&headers, &record).unwrap_err(),
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
                verify_dns_consistency(&headers, &record).unwrap_err(),
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

    mod verify_required_headers_signed {
        use super::*;

        #[test]
        fn passes_when_required_headers_are_signed() {
            let header = b"DKIM-Signature: v=1; a=; c=; d=; s=; t=; h=From:To:Subject; bh=; b=";
            let headers = [&parse_header(header).unwrap().0];
            let required = ["From", "To", "Subject"];

            let result = verify_required_headers_signed(&headers, &required);
            assert!(result.is_ok());
        }

        #[test]
        fn fails_when_required_header_is_missing() {
            let header = b"DKIM-Signature: v=1; a=; c=; d=; s=; t=; h=From:Subject; bh=; b=";
            let headers = [&parse_header(header).unwrap().0];
            let required = ["From", "To", "Subject"];

            assert_eq!(
                verify_required_headers_signed(&headers, &required).unwrap_err(),
                Error::MissingRequiredHeaderTag("To".to_string())
            );
        }
    }
}
