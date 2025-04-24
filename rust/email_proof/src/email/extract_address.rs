use mailparse::{MailAddr, MailAddrList, MailHeader, MailParseError, addrparse_header};

const MAX_LOCAL_LENGTH: usize = 64;
const MAX_DOMAIN_LENGTH: usize = 255;
const VALID_LOCAL_CHARS: &[char] = &[
    '!', '#', '$', '%', '&', '\'', '*', '+', '-', '/', '=', '?', '^', '_', '`', '{', '|', '}', '~',
];

pub(crate) fn extract_address(from_header: &MailHeader<'_>) -> Result<String, MailParseError> {
    let addresses = addrparse_header(from_header)?;
    let raw_addr = extract_single_address(&addresses)?;

    let addr = raw_addr.trim();

    let (local, domain) = split_local_domain(addr)?;

    validate_local_part(local)?;
    validate_domain(domain)?;

    Ok(addr.to_string())
}

fn extract_single_address(addresses: &MailAddrList) -> Result<String, MailParseError> {
    if addresses.len() != 1 {
        return Err(MailParseError::Generic("Expected exactly one address in the \"From\" header"));
    }
    match &addresses[0] {
        MailAddr::Single(info) => Ok(info.addr.clone()),
        _ => Err(MailParseError::Generic(
            "Group addresses are not supported in the \"From\" header",
        )),
    }
}

fn split_local_domain(addr: &str) -> Result<(&str, &str), MailParseError> {
    let parts: Vec<_> = addr.split('@').collect();
    if parts.len() != 2 {
        return Err(MailParseError::Generic("Email address must contain exactly one ‘@’"));
    }
    Ok((parts[0], parts[1]))
}

/// Validates the local-part of an email address according to [RFC 5322 §3.2.3](https://datatracker.ietf.org/doc/html/rfc5322#section-3.2.3)
/// and [RFC 5321 §4.5.3.1](https://datatracker.ietf.org/doc/html/rfc5321#section-4.5.3.1).
///
/// The local-part must be a `dot-atom-text`, which consists of one or more `atext` characters
/// separated by single dots. This implies:
/// - No leading, trailing, or consecutive dots.
/// - Each segment between dots must be non-empty.
/// - Each character in a segment must be valid `atext`.
///
/// Valid `atext` characters include:
/// - Uppercase and lowercase ASCII letters (A–Z, a–z)
/// - Digits (0–9)
/// - Printable special characters: ! # $ % & ' * + - / = ? ^ _ ` { | } ~
fn validate_local_part(local: &str) -> Result<(), MailParseError> {
    if local.is_empty() {
        return Err(MailParseError::Generic("Local-part is empty"));
    }
    if local.len() > MAX_LOCAL_LENGTH {
        return Err(MailParseError::Generic(
            "Local-part too long. Maximal length is 64 characters",
        ));
    }

    fn is_valid_atext(c: char) -> bool {
        c.is_ascii_alphanumeric() || VALID_LOCAL_CHARS.contains(&c)
    }

    local.split('.').try_for_each(|segment| {
        if segment.is_empty() {
            Err(MailParseError::Generic("Empty segment in local-part"))
        } else if !segment.chars().all(is_valid_atext) {
            Err(MailParseError::Generic("Invalid character in local-part"))
        } else {
            Ok(())
        }
    })?;

    Ok(())
}

/// Validates the domain part of an email address according to [RFC 5321 §2.3.1](https://datatracker.ietf.org/doc/html/rfc5321#section-2.3.1).
///
/// The domain must:
/// - Be non-empty and not exceed 255 characters [RFC 5321 §4.5.3.1](https://datatracker.ietf.org/doc/html/rfc5321#section-4.5.3.1).
/// - Be composed of one or more labels separated by dots.
/// - Each label must be non-empty, contain only alphanumeric characters or hyphens,
///   and must not begin or end with a hyphen.
fn validate_domain(domain: &str) -> Result<(), MailParseError> {
    if domain.is_empty() {
        return Err(MailParseError::Generic("Domain is empty"));
    }
    if domain.len() > MAX_DOMAIN_LENGTH {
        return Err(MailParseError::Generic("Domain too long. Maximal length is 255 characters"));
    }

    domain
        .split('.')
        .find_map(|label| validate_domain_label(label).err())
        .map_or(Ok(()), Err)?;

    Ok(())
}

fn validate_domain_label(label: &str) -> Result<(), MailParseError> {
    if label.is_empty() {
        return Err(MailParseError::Generic("Empty label in domain"));
    }
    if label.starts_with('-') || label.ends_with('-') {
        return Err(MailParseError::Generic("Domain label must not start or end with a hyphen"));
    }
    if !label.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
        return Err(MailParseError::Generic("Invalid character in domain"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use mailparse::parse_header;

    use super::*;

    mod extract_address_from_header {
        use super::*;

        #[test]
        fn extracts_email_from_header() {
            let (header, _) = parse_header(b"From:   Name (comment) <hello@aa.aa >  ").unwrap();
            let extracted_email = extract_address(&header).unwrap();
            assert_eq!(extracted_email, "hello@aa.aa");
        }

        #[test]
        fn error_for_wrong_data() {
            let (header, _) = parse_header(b"From:").unwrap();
            let result = extract_address(&header);
            assert!(result.is_err());
        }
    }

    mod addrparse_header {
        use super::*;

        #[test]
        fn works_for_not_named_field() {
            let (header, _) = parse_header(b"From: hello@aa.aa ").unwrap();
            let extracted_email = extract_address(&header).unwrap();
            assert_eq!(extracted_email, "hello@aa.aa");
        }

        #[test]
        fn error_for_missing_brackets() {
            let (header, _) = parse_header(b"Name hello@aa.aa").unwrap();
            let email = extract_address(&header);
            assert_eq!(
                email.unwrap_err().to_string(),
                "Expected exactly one address in the \"From\" header"
            );
        }

        #[test]
        fn error_if_incorrect_email_inside_brackets() {
            let (header, _) = parse_header(b"Name <aaa>").unwrap();
            let email = extract_address(&header);
            assert_eq!(
                email.unwrap_err().to_string(),
                "Expected exactly one address in the \"From\" header"
            );
        }

        #[test]
        fn error_if_brackets_misaligned() {
            let extract = |from: &str| {
                let formatted_from = format!("From: {from}");
                let (header, _) = parse_header(formatted_from.as_bytes()).unwrap();
                extract_address(&header).unwrap_err().to_string()
            };

            assert_eq!(
                extract("Name <hello@aa.aa>>"),
                "Unexpected char found after bracketed address"
            );
            assert_eq!(extract("Name <hello@aa.aa"), "Address string unexpectedly terminated");
            assert_eq!(
                extract("Name <<hello@aa.aa>>"),
                "Unexpected char found after bracketed address"
            );
        }
    }

    mod extract_single_address {
        use mailparse::{GroupInfo, SingleInfo};

        use super::*;

        #[test]
        fn success() {
            let address = MailAddr::Single(SingleInfo {
                display_name: Some("a".to_string()),
                addr: "a@a.a".to_string(),
            });
            let addresses = vec![address].into();
            assert!(extract_single_address(&addresses).is_ok());
        }

        #[test]
        fn fails_when_empty() {
            let addresses = vec![].into();
            let err = extract_single_address(&addresses).unwrap_err();
            assert_eq!(err.to_string(), "Expected exactly one address in the \"From\" header");
        }

        #[test]
        fn fails_when_more_than_one_address() {
            let addresses = vec![
                MailAddr::Single(SingleInfo {
                    display_name: None,
                    addr: "a@a.a".to_string(),
                }),
                MailAddr::Single(SingleInfo {
                    display_name: None,
                    addr: "b@b.b".to_string(),
                }),
            ]
            .into();
            let err = extract_single_address(&addresses).unwrap_err();
            assert_eq!(err.to_string(), "Expected exactly one address in the \"From\" header");
        }

        #[test]
        fn fails_when_group_address() {
            let group = MailAddr::Group(GroupInfo {
                group_name: "group".to_string(),
                addrs: vec![SingleInfo {
                    display_name: None,
                    addr: "a@a.a".to_string(),
                }],
            });
            let addresses = vec![group].into();
            let err = extract_single_address(&addresses).unwrap_err();
            assert_eq!(err.to_string(), "Group addresses are not supported in the \"From\" header");
        }
    }

    mod split_local_domain {
        use super::*;

        #[test]
        fn success() {
            assert!(split_local_domain("a@gmail.com").is_ok());
        }

        #[test]
        fn fails_when_no_at_symbol() {
            let err = split_local_domain("gmail.com").unwrap_err();
            assert_eq!(err.to_string(), "Email address must contain exactly one ‘@’");
        }

        #[test]
        fn fails_when_multiple_at_symbols() {
            let err = split_local_domain("a@b@c").unwrap_err();
            assert_eq!(err.to_string(), "Email address must contain exactly one ‘@’");
        }
    }

    mod validate_local_part {
        use super::*;

        #[test]
        fn success() {
            assert!(validate_local_part("user").is_ok());
            assert!(validate_local_part("user.name").is_ok());
            assert!(validate_local_part("user-name").is_ok());
        }

        #[test]
        fn empty_segment() {
            let err = validate_local_part("user..name").unwrap_err();
            assert_eq!(err.to_string(), "Empty segment in local-part");
        }

        #[test]
        fn invalid_character() {
            let err = validate_local_part("user@name").unwrap_err();
            assert_eq!(err.to_string(), "Invalid character in local-part");
        }
    }

    mod validate_domain {
        use super::*;

        #[test]
        fn success() {
            assert!(validate_domain("example.com").is_ok());
            assert!(validate_domain("sub.example.com").is_ok());
            assert!(validate_domain("sub-domain.example.com").is_ok());
        }

        #[test]
        fn empty_domain() {
            let err = validate_domain("").unwrap_err();
            assert_eq!(err.to_string(), "Domain is empty");
        }

        #[test]
        fn domain_too_long() {
            let long_domain = "a".repeat(256);
            let err = validate_domain(&long_domain).unwrap_err();
            assert_eq!(err.to_string(), "Domain too long. Maximal length is 255 characters");
        }

        #[test]
        fn invalid_label() {
            let result = validate_domain("example@com");
            assert!(result.is_err());
        }
    }

    mod validate_domain_label {
        use super::*;

        #[test]
        fn success() {
            assert!(validate_domain_label("example").is_ok());
            assert!(validate_domain_label("sub-domain").is_ok());
        }

        #[test]
        fn empty_label() {
            let err = validate_domain_label("").unwrap_err();
            assert_eq!(err.to_string(), "Empty label in domain");
        }

        #[test]
        fn wrong_hyphen_placement() {
            let err = validate_domain_label("-example").unwrap_err();
            let err2 = validate_domain_label("example-").unwrap_err();
            assert_eq!(err.to_string(), "Domain label must not start or end with a hyphen");
            assert_eq!(err2.to_string(), "Domain label must not start or end with a hyphen");
        }

        #[test]
        fn invalid_character() {
            let err = validate_domain_label("example@com").unwrap_err();
            assert_eq!(err.to_string(), "Invalid character in domain");
        }
    }
}
