use alloy_sol_types::SolValue;
use mailparse::{headers::Headers, MailHeaderMap, MailParseError, ParsedMail};
use regex::Regex;

pub(crate) mod sol;

#[derive(Debug)]
pub struct Email {
    pub from: String,
    pub to: String,
    pub subject: Option<String>,
    pub body: String,
}

impl Email {
    pub fn abi_encode(self) -> Vec<u8> {
        sol::SolEmail::from(self).abi_encode()
    }
}

impl TryFrom<ParsedMail<'_>> for Email {
    type Error = MailParseError;

    fn try_from(mail: ParsedMail) -> Result<Self, Self::Error> {
        let headers = mail.get_headers();

        let get_header = header_getter(headers);

        let from_raw =
            get_header("From").ok_or(MailParseError::Generic("\"From\" header is missing"))?;
        let from_email = Self::extract_address_from_header(&from_raw)?;
        let to = get_header("To").ok_or(MailParseError::Generic("\"To\" header is missing"))?;
        let subject = get_header("Subject");

        Ok(Email {
            from: from_email,
            body: get_body(&mail)?,
            to,
            subject,
        })
    }
}

fn get_body(mail: &ParsedMail) -> Result<String, MailParseError> {
    mail.parts()
        .filter(|part| is_plain_text_mimetype(part))
        .map(ParsedMail::get_body)
        .collect::<Result<_, _>>()
        .and_then(validate_body_parts)
        .map(|parts| parts.join(""))
}

fn validate_body_parts(parts: Vec<String>) -> Result<Vec<String>, MailParseError> {
    if parts.is_empty() {
        Err(MailParseError::Generic("Plain text body not found in the email"))
    } else {
        Ok(parts)
    }
}

fn is_plain_text_mimetype(part: &ParsedMail) -> bool {
    part.ctype.mimetype == "text/plain"
}

impl Email {
    fn extract_address_from_header(header: &str) -> Result<String, MailParseError> {
        let Some(start) = header.find('<') else {
            return Self::validate_email(header);
        };
        let maybe_end = header.rfind('>');

        match maybe_end {
            None => Err(Self::invalid_from_header()),
            Some(end) if end <= start => Err(Self::invalid_from_header()),
            Some(end) => Self::validate_email(&header[start + 1..end]),
        }
    }

    const fn invalid_from_header() -> MailParseError {
        MailParseError::Generic("Unexpected \"From\" format")
    }

    fn validate_email(email: &str) -> Result<String, MailParseError> {
        let email = email.trim();

        if !Self::is_email_valid(email) {
            return Err(Self::invalid_from_header());
        }

        Ok(email.to_string())
    }

    fn is_email_valid(email: &str) -> bool {
        let email_regex = Regex::new(r"^[\w\-.]+@([\w-]+\.)+[\w-]{2,4}$").unwrap();
        email_regex.is_match(email)
    }
}

fn header_getter(headers: Headers) -> impl Fn(&str) -> Option<String> + '_ {
    move |key: &str| headers.get_first_value(key).map(String::from)
}

#[cfg(test)]
mod test {
    use super::*;

    mod get_body {
        use super::*;
        use crate::test_utils::{build_mime_email, read_file};

        #[test]
        fn returns_whole_body_for_plain_text_body_content_type() -> anyhow::Result<()> {
            let email =
                build_mime_email(vec![("Content-Type", "text/plain")], "This is a plain body");
            let body = get_body(&mailparse::parse_mail(email.as_bytes())?)?;

            assert_eq!(body, "This is a plain body");
            Ok(())
        }

        #[test]
        fn returns_whole_body_when_no_content_type() -> anyhow::Result<()> {
            let email = build_mime_email(vec![], "This is a plain body");
            let body = get_body(&mailparse::parse_mail(email.as_bytes())?)?;

            assert_eq!(body, "This is a plain body");
            Ok(())
        }

        #[test]
        fn throws_when_no_plain_text_in_email() -> anyhow::Result<()> {
            let email =
                build_mime_email(vec![("Content-Type", "text/html")], "<div>This is html</div>");
            let body = get_body(&mailparse::parse_mail(email.as_bytes())?);

            assert_eq!(body.err().unwrap().to_string(), "Plain text body not found in the email");
            Ok(())
        }

        #[test]
        fn for_multipart_email_concats_all_plain_texts() -> anyhow::Result<()> {
            let email = read_file("testdata/email.txt");
            let body = get_body(&mailparse::parse_mail(&email)?)?;

            assert_eq!(
                body,
                r"testtest
testtest
testtest
testtest
testtest
testtest



74c8f09dfa30eaccfb392b132314fc6b978f325a *flex-confirm-mail.1.10.0.xpi
ccee4b4aa47f513abce342ce1e2ec2fd9600e31b *flex-confirm-mail.1.11.0.xpi
071e9e378ad0179bfadb1bdc650a49454d204a83 *flex-confirm-mail.1.12.0.xpi
9d7aa153418ea8fbc8be7ba6f54ce8a1cb7ee468 *flex-confirm-mail.1.9.9.xpi
8168563cb276ea4f9a2b630b9a207d90fb11856e *flex-confirm-mail.xpi
"
            );

            Ok(())
        }

        #[test]
        fn throws_if_no_plain_text_among_multiparts() -> anyhow::Result<()> {
            let email = build_mime_email(
                vec![(
                    "Content-Type",
                    "multipart/alternative; boundary=\"0000000000002fe9ab0626ed1e27\"",
                )],
                r#"--0000000000002fe9ab0626ed1e27
Content-Type: text/html; charset="UTF-8"

Welcome to vlayer, 0x0E8e5015042BeF1ccF2D449652C7A457a163ECB9

--0000000000002fe9ab0626ed1e27
Content-Type: text/html; charset="UTF-8"

<div dir="ltr">Welcome to vlayer, 0x0E8e5015042BeF1ccF2D449652C7A457a163ECB9</div>

--0000000000002fe9ab0626ed1e27--"#,
            );

            let body = get_body(&mailparse::parse_mail(email.as_bytes())?);

            assert_eq!(body.err().unwrap().to_string(), "Plain text body not found in the email");
            Ok(())
        }
    }

    mod try_from {
        use super::*;
        use crate::test_utils::{parsed_email, read_file};

        #[test]
        fn parses_email() {
            let email = parsed_email(
                vec![("From", "me@aa.aa"), ("To", "you"), ("Subject", "hello")],
                "body",
            )
            .unwrap();
            assert_eq!(email.from, "me@aa.aa");
            assert_eq!(email.to, "you");
            assert_eq!(email.subject.unwrap(), "hello");
            assert_eq!(email.body, "body");
        }

        #[test]
        fn parses_body_of_multipart_email() -> anyhow::Result<()> {
            let multipart_email = read_file("testdata/multipart_email.eml");
            let email: Email = mailparse::parse_mail(&multipart_email)?.try_into()?;
            let expected_body = "Welcome to vlayer, 0x0E8e5015042BeF1ccF2D449652C7A457a163ECB9\n\n";
            assert_eq!(email.body, expected_body);
            assert_eq!(email.from, "grzegorz@vlayer.xyz");
            assert_eq!(email.to, "Grzegorz Pociejewski <grzegorz@vlayer.xyz>");
            assert_eq!(
                email.subject.unwrap(),
                "Welcome to vlayer, 0x0E8e5015042BeF1ccF2D449652C7A457a163ECB9"
            );
            Ok(())
        }
        #[test]
        fn error_when_from_header_is_missing() {
            let email = parsed_email(vec![("To", "me")], "body");
            assert_eq!(
                email.unwrap_err().to_string(),
                MailParseError::Generic("\"From\" header is missing").to_string()
            );
        }

        #[test]
        fn error_when_to_header_is_missing() {
            let email = parsed_email(vec![("From", "me@aa.aa")], "body");
            assert_eq!(
                email.unwrap_err().to_string(),
                MailParseError::Generic("\"To\" header is missing").to_string()
            );
        }

        #[test]
        fn takes_first_header_if_multiple() {
            let email = parsed_email(
                vec![("From", "me@aa.aa"), ("From", "you@aa.aa"), ("To", "you"), ("To", "me")],
                "body",
            )
            .unwrap();
            assert_eq!(email.from, "me@aa.aa");
            assert_eq!(email.to, "you");
        }

        #[test]
        fn works_when_body_is_missing() {
            let email = parsed_email(vec![("From", "me@aa.aa"), ("To", "you")], "");
            assert_eq!(email.unwrap().body, "");
        }
    }

    mod abi_encode {
        use super::*;
        use crate::test_utils::parsed_email;

        #[test]
        fn encodes_email_to_sol_type() {
            let email = parsed_email(
                vec![("From", "me@aa.aa"), ("To", "you"), ("Subject", "hello")],
                "body",
            );
            let encoded = email.unwrap().abi_encode();
            let decoded = sol::SolEmail::abi_decode(&encoded, true).unwrap();
            assert_eq!(decoded.from, "me@aa.aa".to_string());
            assert_eq!(decoded.to, "you".to_string());
            assert_eq!(decoded.subject, "hello".to_string());
            assert_eq!(decoded.body, "body".to_string());
        }

        #[test]
        fn replaces_empty_subject_with_empty_string() {
            let email = parsed_email(vec![("From", "me@aa.aa"), ("To", "you")], "body");
            let encoded = email.unwrap().abi_encode();
            let decoded = sol::SolEmail::abi_decode(&encoded, true).unwrap();
            assert_eq!(decoded.from, "me@aa.aa".to_string());
            assert_eq!(decoded.to, "you".to_string());
            assert_eq!(decoded.subject, "".to_string());
            assert_eq!(decoded.body, "body".to_string());
        }
    }

    mod is_email_valid {
        use super::*;

        #[test]
        fn valid_email() {
            assert!(Email::is_email_valid("hello@aa.aa"));
            assert!(Email::is_email_valid("this.is-valid...e-mail@aa.aa"));
        }

        #[test]
        fn invalid_email() {
            assert!(!Email::is_email_valid("hello@aa"));
            assert!(!Email::is_email_valid("hel@lo@aa.aa"));
            assert!(!Email::is_email_valid("hello@aa..aa"));
            assert!(!Email::is_email_valid("hello@aa.a"));
            assert!(!Email::is_email_valid("hello@.aa"));
            assert!(!Email::is_email_valid("hello.aa"));
            assert!(!Email::is_email_valid("email with.whitespace@aa.aa"));
            assert!(!Email::is_email_valid("email<with>brackets@aa.aa"));
        }
    }

    mod validate_email {
        use super::*;

        #[test]
        fn validates_email_with_spaces() {
            assert_eq!(
                Email::validate_email(" hello@aa.aa   ").unwrap(),
                "hello@aa.aa".to_string()
            );
        }

        #[test]
        fn returns_error_for_invalid_email() {
            let email = Email::validate_email("hello@aa");
            assert_eq!(email.unwrap_err().to_string(), Email::invalid_from_header().to_string());
        }
    }

    mod extract_address_from_header {
        use super::*;

        #[test]
        fn extracts_email_from_header() {
            let extracted_email =
                Email::extract_address_from_header("  Name (comment) <hello@aa.aa >  ").unwrap();
            assert_eq!(extracted_email, "hello@aa.aa");
        }

        #[test]
        fn works_for_not_named_field() {
            let extracted_email = Email::extract_address_from_header(" hello@aa.aa ").unwrap();
            assert_eq!(extracted_email, "hello@aa.aa");
        }

        #[test]
        fn error_for_missing_brackets() {
            let email = Email::extract_address_from_header("Name hello@aa.aa");
            assert_eq!(email.unwrap_err().to_string(), Email::invalid_from_header().to_string());
        }

        #[test]
        fn error_if_email_is_missing() {
            let email = Email::extract_address_from_header("Name (comment)");
            assert_eq!(email.unwrap_err().to_string(), Email::invalid_from_header().to_string());
        }

        #[test]
        fn error_if_incorrect_email_inside_brackets() {
            let email = Email::extract_address_from_header("Name <aaa>");
            assert_eq!(email.unwrap_err().to_string(), Email::invalid_from_header().to_string());
        }

        #[test]
        fn error_if_brackets_misaligned() {
            let extract = |from: &str| {
                Email::extract_address_from_header(from)
                    .unwrap_err()
                    .to_string()
            };
            assert_eq!(extract("Name <hello@aa.aa>>"), Email::invalid_from_header().to_string());
            assert_eq!(extract("Name hello@aa.aa>"), Email::invalid_from_header().to_string());
            assert_eq!(extract("Name <<hello@aa.aa>"), Email::invalid_from_header().to_string());
            assert_eq!(extract("Name <hello@aa.aa"), Email::invalid_from_header().to_string());
            assert_eq!(extract("Name <<hello@aa.aa>>"), Email::invalid_from_header().to_string());
        }

        #[test]
        fn error_if_several_emails() {
            let extract = |from: &str| {
                Email::extract_address_from_header(from)
                    .unwrap_err()
                    .to_string()
            };
            assert_eq!(
                extract("Name <hello@aa.aa>, Name2 <hello2@aa.aa>"),
                Email::invalid_from_header().to_string()
            );
        }
    }
}
