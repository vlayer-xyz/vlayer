use mailparse::{MailHeaderMap, ParsedMail};

use crate::{Email, Error};

pub fn extract_from_domain(p0: &ParsedMail) -> Result<String, Error> {
    let from_header = p0
        .get_headers()
        .get_first_value("From")
        .ok_or(Error::InvalidFromHeader("Missing".into()))?;

    let email = Email::extract_address_from_header(&from_header).map_err(Error::EmailParse)?;

    let (_, domain) = email
        .rsplit_once('@')
        .ok_or(Error::InvalidFromHeader(from_header))?;

    Ok(domain.into())
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use lazy_static::lazy_static;

    use super::*;
    use crate::test_utils::build_mime_email;

    lazy_static! {
        static ref FIXTURES: HashMap<&'static str, Result<&'static str, Error>> = {
            [
                ("hello@world.com", Ok("world.com")),
                ("hello@world.com   ", Ok("world.com")),
                ("<hello@world.com>", Ok("world.com")),
                ("Hello Worldowski <hello@world.com>", Ok("world.com")),
                ("Hello Worldowski <hello@world.com  \t>  \t ", Ok("world.com")),
                ("Hello Worldowski (Hi) <hello@world.com>", Ok("world.com")),
                (
                    r#""John Doe" <user@example.com>, "Jane Smith" <jane@example.com>"#,
                    Err(Error::EmailParse(mailparse::MailParseError::Generic(
                        "Unexpected \"From\" format",
                    ))),
                ),
                (r#"John Doe <"user.name"@example.com>"#, Ok("example.com")),
                (
                    r#"John Doe <"user@name"@example.com>"#,
                    Err(Error::EmailParse(mailparse::MailParseError::Generic(
                        "Unexpected \"From\" format",
                    ))),
                ),
                (
                    "@routing:user@example.com",
                    Err(Error::EmailParse(mailparse::MailParseError::Generic(
                        "Unexpected \"From\" format",
                    ))),
                ),
                (
                    "Recipients: John Doe <john@example.com>, Jane Smith <jane@example.com>;",
                    Err(Error::EmailParse(mailparse::MailParseError::Generic(
                        "Unexpected \"From\" format",
                    ))),
                ),
                ("=?UTF-8?B?SsO2cmc=?= <joerg@example.com>", Ok("example.com")),
                (r#""piro-test@clear-code.com" <piro-test@clear-code.com>"#, Ok("clear-code.com")),
            ]
            .into_iter()
            .collect()
        };
    }

    #[test]
    fn extracts_from_domain_when_only_email_is_present() {
        test_domain_extraction("hello@world.com");
    }

    #[test]
    fn correctly_trims_from_domain_when_only_email_is_present() {
        test_domain_extraction("hello@world.com   ");
    }

    #[test]
    fn extracts_from_domain_when_named_email_is_present() {
        test_domain_extraction("Hello Worldowski <hello@world.com>");
    }

    #[test]
    fn correctly_trims_from_domain_when_named_email_is_present() {
        test_domain_extraction("Hello Worldowski <hello@world.com  \t>  \t ");
    }

    #[test]
    fn smtp_envelop_format() {
        test_domain_extraction("<hello@world.com>");
    }

    #[test]
    fn name_with_comment_format() {
        test_domain_extraction("Hello Worldowski (Hi) <hello@world.com>");
    }

    #[ignore]
    #[test]
    fn quoted_local_part_format() {
        test_domain_extraction(r#"John Doe <"user.name"@example.com>"#);
    }

    #[test]
    fn fails_for_multiple_recipients() {
        test_domain_extraction(r#""John Doe" <user@example.com>, "Jane Smith" <jane@example.com>"#);
    }

    #[test]
    fn fails_for_quoted_local_part_with_at_symbol() {
        test_domain_extraction(r#"John Doe <"user@name"@example.com>"#);
    }

    #[test]
    fn fails_for_deprecated_routing_information() {
        test_domain_extraction("@routing:user@example.com");
    }

    #[test]
    fn fails_for_grouping_format() {
        test_domain_extraction(
            "Recipients: John Doe <john@example.com>, Jane Smith <jane@example.com>;",
        );
    }

    #[test]
    fn utf_encoded_names() {
        test_domain_extraction("=?UTF-8?B?SsO2cmc=?= <joerg@example.com>");
    }

    #[test]
    fn with_at_symbol_in_name() {
        test_domain_extraction(r#""piro-test@clear-code.com" <piro-test@clear-code.com>"#);
    }

    fn test_domain_extraction(key: &'static str) {
        let raw_email = build_mime_email(vec![("From", key)], "Body");
        let email = mailparse::parse_mail(raw_email.as_ref()).unwrap();
        let result = extract_from_domain(&email);
        let expected = FIXTURES.get(key).unwrap();
        match result {
            Ok(res) => {
                assert_eq!(&res.as_str(), expected.as_ref().unwrap())
            }
            Err(err) => assert_eq!(err.to_string(), expected.as_ref().unwrap_err().to_string()),
        }
    }
}
