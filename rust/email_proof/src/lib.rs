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

    fn email_fixture() -> Vec<u8> {
        read_file("testdata/email.txt")
    }

    #[test]
    fn test_parse_mime() {
        let email = email_fixture();
        let parsed = parse_mime(&email).unwrap();

        let expected_from: String =
            "\"piro-test@clear-code.com\" <piro-test@clear-code.com>".into();
        assert_eq!(expected_from, parsed.from.unwrap());

        let expected_to: String = "piro.outsider.reflex+1@gmail.com, \
                 piro.outsider.reflex+2@gmail.com, \
                 mailmaster@example.com, \
                 mailmaster@example.org, \
                 webmaster@example.com, \
                 webmaster@example.org, \
                 webmaster@example.jp, \
                 mailmaster@example.jp"
            .into();
        assert_eq!(expected_to, parsed.to.unwrap());

        assert_eq!(Some("test confirmation".into()), parsed.subject);
        assert_eq!(
            "2019-08-15 14:54:37 +09:00",
            parsed.date.unwrap().to_string()
        );
        assert_eq!(
            "This is a multi-part message in MIME format.\n",
            parsed.body
        );
    }
}
