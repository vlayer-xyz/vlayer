use alloy_sol_types::SolValue;
use extract_address::extract_address;
use mailparse::{
    DispositionType, MailHeader, MailHeaderMap, MailParseError, ParsedMail, headers::Headers,
};
use sol::SolEmail;

pub(crate) mod extract_address;
pub(crate) mod sol;

#[derive(Debug, PartialEq)]
pub struct Email {
    pub from: String,
    pub to: String,
    pub subject: Option<String>,
    pub body: String,
}

impl Email {
    pub fn abi_encode(self) -> Vec<u8> {
        SolEmail::from(self).abi_encode()
    }
}

impl TryFrom<ParsedMail<'_>> for Email {
    type Error = MailParseError;

    fn try_from(mail: ParsedMail) -> Result<Self, Self::Error> {
        let headers = mail.get_headers();

        let get_header = last_header_getter(&headers);

        let from_header =
            get_header("From").ok_or(MailParseError::Generic("\"From\" header is missing"))?;
        let from_email = extract_address(from_header)?;
        let to = get_header("To")
            .ok_or(MailParseError::Generic("\"To\" header is missing"))?
            .get_value();
        let subject = get_header("Subject").map(MailHeader::get_value);

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
        .filter(|part| is_inlined_body_content(part))
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

fn is_inlined_body_content(part: &ParsedMail) -> bool {
    part.get_content_disposition().disposition == DispositionType::Inline
}

// Last headers are signed first: https://datatracker.ietf.org/doc/html/rfc6376#section-5.4.2
fn last_header_getter<'a>(
    headers: &'a Headers,
) -> impl Fn(&str) -> Option<&'a mailparse::MailHeader<'a>> {
    move |key: &str| headers.get_all_headers(key).pop()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::{build_mime_email, parsed_email, read_file};

    mod get_body {
        use super::*;

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
        #[test]
        fn ignores_attachments() -> anyhow::Result<()> {
            let email = build_mime_email(
                vec![(
                    "Content-Type",
                    "multipart/alternative; boundary=\"00000000000039e0a3062e05ba79\"",
                )],
                r#"
--00000000000039e0a3062e05ba79
Content-Type: text/plain; charset="UTF-8"

Email content
--00000000000039e0a3062e05ba79
Content-Type: text/plain; charset="US-ASCII"; name="attachment.txt"
Content-Disposition: attachment; filename="attachment.txt"
Content-Transfer-Encoding: base64
Content-ID: <f_m73cs6o50>
X-Attachment-Id: f_m73cs6o50

ZmlsZSBjb250ZW50Cg==
--00000000000039e0a3062e05ba79--"#,
            );

            let expected_body = "Email content\n".to_string();

            let body = get_body(&mailparse::parse_mail(email.as_bytes())?).unwrap();
            assert_eq!(body, expected_body);
            Ok(())
        }
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
        fn takes_last_header_if_multiple() {
            let email = parsed_email(
                vec![("From", "me@aa.aa"), ("From", "you@aa.aa"), ("To", "you"), ("To", "me")],
                "body",
            )
            .unwrap();
            assert_eq!(email.from, "you@aa.aa");
            assert_eq!(email.to, "me");
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
            let decoded = SolEmail::abi_decode(&encoded, true).unwrap();
            assert_eq!(decoded.from, "me@aa.aa".to_string());
            assert_eq!(decoded.to, "you".to_string());
            assert_eq!(decoded.subject, "hello".to_string());
            assert_eq!(decoded.body, "body".to_string());
        }

        #[test]
        fn replaces_empty_subject_with_empty_string() {
            let email = parsed_email(vec![("From", "me@aa.aa"), ("To", "you")], "body");
            let encoded = email.unwrap().abi_encode();
            let decoded = SolEmail::abi_decode(&encoded, true).unwrap();
            assert_eq!(decoded.from, "me@aa.aa".to_string());
            assert_eq!(decoded.to, "you".to_string());
            assert_eq!(decoded.subject, "".to_string());
            assert_eq!(decoded.body, "body".to_string());
        }
    }
}
