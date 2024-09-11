mod email;

use crate::email::Email;
use mailparse::MailParseError;

pub fn parse_mime(email: &[u8]) -> Result<Email, MailParseError> {
    mailparse::parse_mail(email)?.try_into()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_file(file: &str) -> Vec<u8> {
        use std::fs::File;
        use std::io::Read;

        let mut f = File::open(file).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();

        buffer
    }

    fn read_test_email() -> Vec<u8> {
        read_file("testdata/email.txt")
    }

    #[test]
    fn test_parse_mime() {
        let email = read_test_email();
        let parsed = parse_mime(&email).unwrap();

        assert_eq!(
            parsed.from,
            Some("\"piro-test@clear-code.com\" <piro-test@clear-code.com>".into())
        );
        assert_eq!(
            parsed.to,
            Some(
                "piro.outsider.reflex+1@gmail.com, \
                 piro.outsider.reflex+2@gmail.com, \
                 mailmaster@example.com, \
                 mailmaster@example.org, \
                 webmaster@example.com, \
                 webmaster@example.org, \
                 webmaster@example.jp, \
                 mailmaster@example.jp"
                    .into()
            )
        );
        assert_eq!(parsed.subject, Some("test confirmation".into()));
        assert_eq!(
            parsed.date.unwrap().to_string(),
            "2019-08-15 14:54:37 +09:00"
        );
        assert_eq!(
            parsed.body,
            "This is a multi-part message in MIME format.\n"
        );
    }
}
