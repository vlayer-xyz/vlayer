use alloy_sol_types::SolValue;
use mailparse::headers::Headers;
use mailparse::{MailHeaderMap, MailParseError, ParsedMail};
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

        let body = mail.get_body()?;

        Ok(Email {
            from: from_email,
            to,
            subject,
            body,
        })
    }
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

    fn build_mime_email(headers: Vec<(&str, &str)>, body: &str) -> String {
        let mut email = String::new();
        for (key, value) in headers {
            email.push_str(&format!("{}: {}\n", key, value));
        }
        if !body.is_empty() {
            email.push('\n');
            email.push_str(body);
        }
        email
    }

    fn parsed_email(headers: Vec<(&str, &str)>, body: &str) -> Result<Email, MailParseError> {
        let mime = build_mime_email(headers, body);
        Email::try_from(mailparse::parse_mail(mime.as_bytes())?)
    }

    mod try_from {
        use super::*;

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
