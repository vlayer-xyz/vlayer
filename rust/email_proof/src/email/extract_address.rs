use mailparse::{MailAddr, MailAddrList, MailHeader, MailParseError, addrparse_header};

pub(crate) fn extract_address(from_header: &MailHeader<'_>) -> Result<String, MailParseError> {
    let addresses = addrparse_header(from_header)?;
    let raw_addr = extract_single_address(&addresses)?;

    let addr = trim_and_reject_whitespace(&raw_addr)?;

    let (local, domain) = split_local_domain(addr)?;

    validate_non_empty(local, domain)?;
    validate_length(local, domain)?;
    validate_dot_placement(local, domain)?;
    validate_local_chars(local)?;
    validate_domain_chars(domain)?;

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

fn trim_and_reject_whitespace(raw: &str) -> Result<&str, MailParseError> {
    let t = raw.trim();
    if t.chars().any(char::is_whitespace) {
        return Err(MailParseError::Generic(
            "Email address must not contain whitespace characters",
        ));
    }
    Ok(t)
}

fn split_local_domain(addr: &str) -> Result<(&str, &str), MailParseError> {
    let parts: Vec<_> = addr.split('@').collect();
    if parts.len() != 2 {
        return Err(MailParseError::Generic("Email address must contain exactly one ‘@’"));
    }
    Ok((parts[0], parts[1]))
}

const fn validate_non_empty(local: &str, domain: &str) -> Result<(), MailParseError> {
    if local.is_empty() || domain.is_empty() {
        Err(MailParseError::Generic("Local-part or domain is empty"))
    } else {
        Ok(())
    }
}

// The maximum length of the local-part is 64 characters, and the domain part is 255 characters.
// https://datatracker.ietf.org/doc/html/rfc5321#section-4.5.3.1
const fn validate_length(local: &str, domain: &str) -> Result<(), MailParseError> {
    if local.len() > 64 || domain.len() > 255 {
        Err(MailParseError::Generic("Local-part or domain too long"))
    } else {
        Ok(())
    }
}

/// Validates dot placement in the local-part and domain of an email address.
///
/// According to [RFC 5322 §3.2.3](https://datatracker.ietf.org/doc/html/rfc5322#section-3.2.3),
/// `dot-atom-text` — which is a common format for the local-part — must consist of one or more 
/// valid `atext` characters separated by single dots. The format must:
/// - not start or end with a dot,
/// - not contain consecutive dots (`..`).
///
/// This function returns an error if either the local or domain part violates these dot rules.
fn validate_dot_placement(local: &str, domain: &str) -> Result<(), MailParseError> {
    let bad = |s: &str| s.starts_with('.') || s.ends_with('.') || s.contains("..");
    if bad(local) || bad(domain) {
        Err(MailParseError::Generic(
            "Invalid dot placement in local-part or domain",
        ))
    } else {
        Ok(())
    }
}

fn validate_local_chars(local: &str) -> Result<(), MailParseError> {
    fn is_valid_local_char(c: char) -> bool {
        c.is_ascii_alphanumeric()
            || matches!(c, '!' | '#' | '$' | '%' | '&' | '\'' | '*' | '+' | '-' | '/' | '=' |
                           '?' | '^' | '_' | '`' | '{' | '|' | '}' | '~' | '.')
    }

    if local.chars().all(is_valid_local_char) {
        Ok(())
    } else {
        Err(MailParseError::Generic("Invalid character in local-part"))
    }
}

fn validate_domain_chars(domain: &str) -> Result<(), MailParseError> {
    for label in domain.split('.') {
        if !label.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            return Err(MailParseError::Generic("Invalid character in domain"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod extract_address_from_header {
        use mailparse::parse_header;

        use super::*;

        #[test]
        fn extracts_email_from_header() {
            let (header, _) = parse_header(b"From:   Name (comment) <hello@aa.aa >  ").unwrap();
            let extracted_email = extract_address(&header).unwrap();
            assert_eq!(extracted_email, "hello@aa.aa");
        }

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
            assert_eq!(
                extract("Name hello@aa.aa>"),
                "Email address must not contain whitespace characters"
            );
            assert_eq!(extract("Name <hello@aa.aa"), "Address string unexpectedly terminated");
            assert_eq!(
                extract("Name <<hello@aa.aa>>"),
                "Unexpected char found after bracketed address"
            );
        }

        #[test]
        fn error_if_several_emails() {
            let (header, _) =
                parse_header(b"From: Name <hello@aa.aa>, Name2 <hello2@aa.aa>").unwrap();
            let error = extract_address(&header).unwrap_err().to_string();
            assert_eq!(error, "Expected exactly one address in the \"From\" header");
        }

        #[test]
        fn error_multiple_at_symbols() {
            let (header, _) = parse_header(b"From: <foo@@bar.com>").unwrap();
            let error = extract_address(&header).unwrap_err().to_string();
            assert_eq!(error, "Email address must contain exactly one ‘@’");
        }

        #[test]
        fn error_empty_local_part() {
            let (header, _) = parse_header(b"From: <@example.com>").unwrap();
            assert_eq!(
                extract_address(&header).unwrap_err().to_string(),
                "Local-part or domain is empty"
            );
        }

        #[test]
        fn error_empty_domain_part() {
            let (header, _) = parse_header(b"From: <local@>").unwrap();
            assert_eq!(
                extract_address(&header).unwrap_err().to_string(),
                "Local-part or domain is empty"
            );
        }

        #[test]
        fn error_local_part_too_long() {
            let long_local = "a".repeat(65);
            let hdr = format!("From: <{}@example.com>", long_local);
            let (header, _) = parse_header(hdr.as_bytes()).unwrap();
            let error = extract_address(&header).unwrap_err().to_string();
            assert_eq!(error, "Local-part or domain too long");
        }

        #[test]
        fn error_domain_part_too_long() {
            let long_domain = "a".repeat(256);
            let hdr = format!("From: <user@{}>", long_domain);
            let (header, _) = parse_header(hdr.as_bytes()).unwrap();
            assert!(extract_address(&header).is_err());
            let error = extract_address(&header).unwrap_err().to_string();
            assert_eq!(error, "Local-part or domain too long");
        }

        mod invalid_dots {
            use super::*;

            const INVALID_DOT_PLACEMENT: &str = "Invalid dot placement in local-part or domain";

            #[test]
            fn error_leading_dot_in_local() {
                let (header, _) = parse_header(b"From: <.local@domain.com>").unwrap();
                let error = extract_address(&header).unwrap_err().to_string();
                assert_eq!(error, INVALID_DOT_PLACEMENT);
            }

            #[test]
            fn error_trailing_dot_in_local() {
                let (header, _) = parse_header(b"From: <local.@domain.com>").unwrap();
                let error = extract_address(&header).unwrap_err().to_string();
                assert_eq!(error, INVALID_DOT_PLACEMENT);
            }

            #[test]
            fn error_consecutive_dots_in_local() {
                let (header, _) = parse_header(b"From: <lo..cal@domain.com>").unwrap();
                let error = extract_address(&header).unwrap_err().to_string();
                assert_eq!(error, INVALID_DOT_PLACEMENT);
            }

            #[test]
            fn error_leading_dot_in_domain() {
                let (header, _) = parse_header(b"From: <local@.domain.com>").unwrap();
                let error = extract_address(&header).unwrap_err().to_string();
                assert_eq!(error, INVALID_DOT_PLACEMENT);
            }

            #[test]
            fn error_trailing_dot_in_domain() {
                let (header, _) = parse_header(b"From: <local@domain.com.>").unwrap();
                let error = extract_address(&header).unwrap_err().to_string();
                assert_eq!(error, INVALID_DOT_PLACEMENT);
            }

            #[test]
            fn error_consecutive_dots_in_domain() {
                let (header, _) = parse_header(b"From: <local@domain..com>").unwrap();
                let error = extract_address(&header).unwrap_err().to_string();
                assert_eq!(error, INVALID_DOT_PLACEMENT);
            }
        }

        #[test]
        fn error_invalid_character_in_local() {
            let (header, _) = parse_header(b"From: <loc(al)@domain.com>").unwrap();
            let error = extract_address(&header).unwrap_err().to_string();
            assert_eq!(error, "Invalid character in local-part");
        }

        #[test]
        fn error_invalid_character_in_domain() {
            let (header, _) = parse_header(b"From: <user@domain_.com>").unwrap();
            let error = extract_address(&header).unwrap_err().to_string();
            assert_eq!(error, "Invalid character in domain");
        }
    }
}
