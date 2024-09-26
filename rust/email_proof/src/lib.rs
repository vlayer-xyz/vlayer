extern crate mail_auth as extern_mail_auth;

mod dkim;
mod email;
mod errors;
mod mail_auth;

pub use crate::email::Email;
pub use crate::errors::Error;

use dkim::verify;
use email::sol::UnverifiedEmail;

use mailparse::MailParseError;

pub fn parse_and_verify(calldata: &[u8]) -> Result<Email, Error> {
    let (raw_email, dns_records) =
        UnverifiedEmail::parse_calldata(calldata).map_err(Error::Calldata)?;
    verify::verify_dkim(&raw_email, &dns_records).map_err(Error::DkimVerification)?;
    parse_mime(&raw_email).map_err(Error::EmailParse)
}

fn parse_mime(email: &[u8]) -> Result<Email, MailParseError> {
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
        assert_eq!(expected_from, parsed.from);

        let expected_to: String = "piro.outsider.reflex+1@gmail.com, \
                 piro.outsider.reflex+2@gmail.com, \
                 mailmaster@example.com, \
                 mailmaster@example.org, \
                 webmaster@example.com, \
                 webmaster@example.org, \
                 webmaster@example.jp, \
                 mailmaster@example.jp"
            .into();
        assert_eq!(expected_to, parsed.to);

        assert_eq!(Some("test confirmation".into()), parsed.subject);
        assert_eq!("This is a multi-part message in MIME format.\n", parsed.body);
    }
}
