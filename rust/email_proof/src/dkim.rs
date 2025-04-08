use cfdkim::{header::DKIMHeader, validate_header, DKIMError};
use mailparse::{MailHeader, MailHeaderMap, ParsedMail};
use verifiable_dns::DNSRecord;

pub use crate::errors::Error;

pub(crate) mod verify_signature;

const DKIM_SIGNATURE_HEADER: &str = "DKIM-Signature";

pub fn get_dkim_header<'a>(email: &'a ParsedMail) -> Result<&'a MailHeader<'a>, Error> {
    let dkim_headers = email.headers.get_all_headers(DKIM_SIGNATURE_HEADER);

    if dkim_headers.len() != 1 {
        return Err(Error::InvalidDkimHeaderCount(dkim_headers.len()));
    }
    Ok(dkim_headers[0])
}

pub fn verify_dkim_body_length_tag(dkim_header: &MailHeader) -> Result<(), DKIMError> {
    let value = String::from_utf8_lossy(dkim_header.get_value_raw());
    let dkim_header = validate_header(&value)?;

    if dkim_header.get_tag("l").is_some() {
        return Err(DKIMError::SignatureSyntaxError(
            "DKIM-Signature header contains body length tag (l=)".into(),
        ));
    }

    Ok(())
}

pub fn verify_dns_consistency(dkim_header: &MailHeader, record: &DNSRecord) -> Result<(), Error> {
    let dkim_header = validate_header(&String::from_utf8_lossy(dkim_header.get_value_raw()))?;

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

    Ok(())
}

fn normalize_dns_name(name: &str) -> String {
    name.trim().trim_end_matches('.').to_lowercase()
}

pub fn verify_required_headers_signed(
    dkim_header: &MailHeader,
    required_signed_headers: &[&str],
) -> Result<(), Error> {
    let value = String::from_utf8_lossy(dkim_header.get_value_raw());
    let dkim_header = validate_header(&value)?;
    let signed_headers = signed_headers(&dkim_header);

    for &required_field in required_signed_headers {
        if !signed_headers.contains(&required_field.to_lowercase()) {
            return Err(Error::MissingRequiredHeaderTag(required_field.to_string()));
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

    mod verify_exactly_one_dkim_header {
        use mailparse::parse_mail;

        use super::*;

        #[test]
        fn passes_for_single_dkim_header() {
            let email =
                parse_mail(b"DKIM-Signature: v=1; a=; c=; d=; s=; t=; h=From; bh=; b=").unwrap();

            let result = get_dkim_header(&email);
            assert!(result.is_ok());
        }

        #[test]
        fn fails_for_not_exactly_one_dkim_headers() {
            let email = parse_mail(b"").unwrap();

            assert_eq!(
                get_dkim_header(&email).unwrap_err(),
                Error::InvalidDkimHeaderCount(0)
            );
        }
    }

    mod verify_dkim_body_length_tag {
        use super::*;

        #[test]
        fn passes_for_header_without_l_tag() {
            let header = b"DKIM-Signature: v=1; a=; c=; d=; s=; t=; h=From; bh=; b=";
            let header = parse_header(header).unwrap().0;

            assert!(verify_dkim_body_length_tag(&header).is_ok());
        }

        #[test]
        fn fails_for_header_with_l_tag() {
            let header = b"DKIM-Signature: v=1; a=; c=; d=; s=; t=; h=From; bh=; b=; l=100";
            let header = parse_header(header).unwrap().0;

            assert_eq!(
                verify_dkim_body_length_tag(&header).unwrap_err(),
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
            let header = parse_header(header).unwrap().0;
            let record = record_with_name("selector1._domainkey.example.com");

            assert!(verify_dns_consistency(&header, &record).is_ok());
        }

        #[test]
        fn fails_for_different_selector() {
            let header =
                b"DKIM-Signature: v=1; a=; c=; d=example.com; s=selector1; h=From; bh=; b=";
            let header = parse_header(header).unwrap().0;
            let record = record_with_name("selector2._domainkey.example.com");

            assert_eq!(
                verify_dns_consistency(&header, &record).unwrap_err(),
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
            let header = parse_header(header).unwrap().0;
            let record = record_with_name("selector1._domainkey.otherdomain.com");

            assert_eq!(
                verify_dns_consistency(&header, &record).unwrap_err(),
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
            let header = parse_header(header).unwrap().0;
            let required = ["From", "To", "Subject"];

            let result = verify_required_headers_signed(&header, &required);
            assert!(result.is_ok());
        }

        #[test]
        fn fails_when_required_header_is_missing() {
            let header = b"DKIM-Signature: v=1; a=; c=; d=; s=; t=; h=From:Subject; bh=; b=";
            let header = parse_header(header).unwrap().0;
            let required = ["From", "To", "Subject"];

            assert_eq!(
                verify_required_headers_signed(&header, &required).unwrap_err(),
                Error::MissingRequiredHeaderTag("To".to_string())
            );
        }
    }
}
