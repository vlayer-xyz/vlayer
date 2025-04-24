use cfdkim::{DKIMError, header, validate_header};
use itertools::Itertools;
use mailparse::{MailHeader, MailHeaderMap, ParsedMail};
use verifiable_dns::DNSRecord;

use crate::email::extract_address::extract_address;
pub use crate::errors::Error;

pub(crate) mod verify_signature;

const DKIM_SIGNATURE_HEADER: &str = "DKIM-Signature";

#[derive(Debug, Clone)]
pub struct DKIMHeader(pub header::DKIMHeader);

impl TryFrom<&MailHeader<'_>> for DKIMHeader {
    type Error = DKIMError;

    fn try_from(dkim_header: &MailHeader) -> Result<Self, Self::Error> {
        let value = String::from_utf8_lossy(dkim_header.get_value_raw());
        let dkim_header = validate_header(&value)?;
        Ok(DKIMHeader(dkim_header))
    }
}

impl DKIMHeader {
    pub fn verify_body_length_tag(&self) -> Result<(), DKIMError> {
        if self.0.get_tag("l").is_some() {
            return Err(DKIMError::SignatureSyntaxError(
                "DKIM-Signature header contains body length tag (l=)".into(),
            ));
        }
        Ok(())
    }

    pub fn verify_dns_consistency(&self, record: &DNSRecord) -> Result<(), Error> {
        let Some(selector) = self.0.get_tag("s") else {
            return Err(Error::DkimVerification(DKIMError::SignatureSyntaxError(
                "Missing selector tag (s=) in DKIM-Signature".into(),
            )));
        };
        let Some(domain) = self.0.get_tag("d") else {
            return Err(Error::DkimVerification(DKIMError::SignatureSyntaxError(
                "Missing domain tag (d=) in DKIM-Signature".into(),
            )));
        };

        let expected = normalize_dns_name(&format!("{selector}._domainkey.{domain}"));
        let actual = normalize_dns_name(&record.name);

        if expected != actual {
            return Err(Error::DomainMismatch(expected, actual));
        }

        Ok(())
    }

    pub fn verify_required_headers_signed(
        &self,
        required_signed_headers: &[&str],
    ) -> Result<(), Error> {
        let signed_headers = Self::signed_headers(self);

        if let Some(missing) = required_signed_headers
            .iter()
            .find(|h| !signed_headers.contains(&h.to_lowercase()))
        {
            return Err(Error::MissingRequiredHeaderTag((*missing).to_string()));
        }

        Ok(())
    }

    fn signing_domain(&self) -> Option<String> {
        self.0.get_tag("d")
    }

    fn signed_headers(&self) -> Vec<String> {
        self.0
            .get_required_tag("h")
            .split(':')
            .map(|s| s.trim().to_lowercase())
            .collect()
    }
}

fn normalize_dns_name(name: &str) -> String {
    name.trim().trim_end_matches('.').to_lowercase()
}

pub fn get_dkim_header(email: &ParsedMail) -> Result<DKIMHeader, Error> {
    let dkim_headers: Vec<_> = email
        .headers
        .get_all_headers(DKIM_SIGNATURE_HEADER)
        .iter()
        .map(|header| (*header).try_into())
        .collect::<Result<_, _>>()?;
    let from_headers = email.headers.get_all_headers("From");

    let Some(from_header) = from_headers.last() else {
        return Err(Error::NoFromHeader);
    };
    let address = extract_address(from_header)?;
    let from_domain = address.split('@').nth(1).unwrap_or_else(|| {
        unreachable!("`extract_address` function ensures that `address` has exactly one '@'")
    });

    let headers_signing_from_domain: Vec<_> =
        filter_dkim_headers_by_domain(dkim_headers, from_domain);

    let only = headers_signing_from_domain
        .into_iter()
        .exactly_one()
        // It's possible to have multiple DKIM-Signature headers with the
        // same signing domain but we have decided not to support it.
        .map_err(|v| Error::InvalidDkimHeaderCount(v.len()))?;

    Ok(only)
}

fn filter_dkim_headers_by_domain(dkim_headers: Vec<DKIMHeader>, domain: &str) -> Vec<DKIMHeader> {
    dkim_headers
        .into_iter()
        .filter_map(|dkim_header| match dkim_header.signing_domain() {
            Some(sig_domain) if sig_domain.eq_ignore_ascii_case(domain) => Some(dkim_header),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use mailparse::parse_header;

    use super::*;

    pub fn from_raw_data(raw_data: &[u8]) -> DKIMHeader {
        DKIMHeader::try_from(&parse_header(raw_data).unwrap().0).unwrap()
    }

    mod get_dkim_header {
        use mailparse::parse_mail;

        use super::*;

        #[test]
        fn passes_for_single_dkim_header() {
            let email = parse_mail(
                b"From: Alice <alice@example.com>\r\n\
                DKIM-Signature: v=1; a=; c=; d=example.com; s=; t=; h=From; bh=; b=",
            )
            .unwrap();

            let result = get_dkim_header(&email);
            assert!(result.is_ok());
        }

        #[test]
        fn fails_for_not_exactly_one_dkim_headers() {
            let email = parse_mail(b"From: Alice <alice@example.com>").unwrap();

            assert_eq!(get_dkim_header(&email).unwrap_err(), Error::InvalidDkimHeaderCount(0));
        }

        #[test]
        fn fails_for_no_from_header() {
            let email =
                parse_mail(b"DKIM-Signature: v=1; a=; c=; d=example.com; s=; t=; h=From; bh=; b=")
                    .unwrap();

            assert_eq!(get_dkim_header(&email).unwrap_err(), Error::NoFromHeader);
        }
    }

    #[cfg(test)]
    mod filter_dkim_headers_by_domain {
        use super::*;

        const DOMAIN: &str = "example.com";

        fn header_with_domain(domain: &str) -> DKIMHeader {
            from_raw_data(
                format!("DKIM-Signature: v=1; a=; c=; d={domain}; s=; t=; h=From; bh=; b=")
                    .as_bytes(),
            )
        }

        #[test]
        fn returns_matching_headers() {
            let headers = vec![header_with_domain(DOMAIN), header_with_domain("other.com")];

            let filtered = filter_dkim_headers_by_domain(headers, DOMAIN);

            let domains: Vec<String> = filtered
                .iter()
                .filter_map(DKIMHeader::signing_domain)
                .collect();

            assert!(domains.iter().all(|d| d.eq(DOMAIN)));
        }

        #[test]
        fn returns_empty_for_no_matching_headers() {
            let headers = vec![header_with_domain("other.com")];

            let filtered = filter_dkim_headers_by_domain(headers, DOMAIN);

            assert!(filtered.is_empty());
        }
    }

    mod verify_dkim_body_length_tag {
        use super::*;

        #[test]
        fn passes_for_header_without_l_tag() {
            let header = from_raw_data(b"DKIM-Signature: v=1; a=; c=; d=; s=; t=; h=From; bh=; b=");

            assert!(header.verify_body_length_tag().is_ok());
        }

        #[test]
        fn fails_for_header_with_l_tag() {
            let header =
                from_raw_data(b"DKIM-Signature: v=1; a=; c=; d=; s=; t=; h=From; bh=; b=; l=100");

            assert_eq!(
                header.verify_body_length_tag().unwrap_err(),
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
            let header = from_raw_data(
                b"DKIM-Signature: v=1; a=; c=; d=example.com; s=selector1; h=From; bh=; b=",
            );
            let record = record_with_name("selector1._domainkey.example.com");

            assert!(header.verify_dns_consistency(&record).is_ok());
        }

        #[test]
        fn fails_for_different_selector() {
            let header = from_raw_data(
                b"DKIM-Signature: v=1; a=; c=; d=example.com; s=selector1; h=From; bh=; b=",
            );
            let record = record_with_name("selector2._domainkey.example.com");

            assert_eq!(
                header.verify_dns_consistency(&record).unwrap_err(),
                Error::DomainMismatch(
                    "selector1._domainkey.example.com".into(),
                    "selector2._domainkey.example.com".into()
                )
            );
        }

        #[test]
        fn fails_for_different_domain() {
            let header = from_raw_data(
                b"DKIM-Signature: v=1; a=; c=; d=example.com; s=selector1; h=From; bh=; b=",
            );
            let record = record_with_name("selector1._domainkey.otherdomain.com");

            assert_eq!(
                header.verify_dns_consistency(&record).unwrap_err(),
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
            let header = from_raw_data(
                b"DKIM-Signature: v=1; a=; c=; d=; s=; t=; h=From:To:Subject; bh=; b=",
            );
            let required = ["From", "To", "Subject"];

            let result = header.verify_required_headers_signed(&required);
            assert!(result.is_ok());
        }

        #[test]
        fn fails_when_required_header_is_missing() {
            let header =
                from_raw_data(b"DKIM-Signature: v=1; a=; c=; d=; s=; t=; h=From:Subject; bh=; b=");
            let required = ["From", "To", "Subject"];

            assert_eq!(
                header
                    .verify_required_headers_signed(&required)
                    .unwrap_err(),
                Error::MissingRequiredHeaderTag("To".to_string())
            );
        }
    }
}
