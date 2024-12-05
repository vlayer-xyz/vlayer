#[cfg(not(feature = "cloudflare"))]
extern crate mail_auth as extern_mail_auth;

#[cfg(feature = "cloudflare")]
mod cloudflare_dkim;
#[cfg(not(feature = "cloudflare"))]
mod dkim;
mod email;
mod errors;
#[cfg(not(feature = "cloudflare"))]
mod mail_auth;

#[cfg(not(feature = "cloudflare"))]
use dkim::verify;
pub use email::sol::UnverifiedEmail;
#[cfg(not(feature = "cloudflare"))]
use mailparse::MailParseError;

pub use crate::{email::Email, errors::Error};

pub fn parse_and_verify(calldata: &[u8]) -> Result<Email, Error> {
    let (raw_email, dns_records) =
        UnverifiedEmail::parse_calldata(calldata).map_err(Error::Calldata)?;
    #[cfg(not(feature = "cloudflare"))]
    {
        verify::verify_dkim(&raw_email, &dns_records).map_err(Error::DkimVerification)?;
        parse_mime(&raw_email).map_err(Error::EmailParse)
    }
    #[cfg(feature = "cloudflare")]
    {
        let x = cloudflare_dkim::verify_email(&raw_email, &dns_records)
            .map_err(|_| Error::Generic("Dupa".into()))?;

        x.try_into().map_err(Error::EmailParse)
    }
}

#[cfg(not(feature = "cloudflare"))]
fn parse_mime(email: &[u8]) -> Result<Email, MailParseError> {
    mailparse::parse_mail(email)?.try_into()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_file(file: &str) -> Vec<u8> {
        std::fs::read(file).unwrap()
    }

    fn email_fixture() -> Vec<u8> {
        read_file("testdata/email.txt")
    }

    #[test]
    fn test_parse_mime() {
        let email = email_fixture();
        let parsed = parse_mime(&email).unwrap();

        assert_eq!(parsed.from, "piro-test@clear-code.com".to_string());

        let expected_to: String = "piro.outsider.reflex+1@gmail.com, \
                 piro.outsider.reflex+2@gmail.com, \
                 mailmaster@example.com, \
                 mailmaster@example.org, \
                 webmaster@example.com, \
                 webmaster@example.org, \
                 webmaster@example.jp, \
                 mailmaster@example.jp"
            .into();
        assert_eq!(parsed.to, expected_to);

        assert_eq!(parsed.subject, Some("test confirmation".into()));
        assert!(parsed
            .body
            .contains("testtest\ntesttest\ntesttest\ntesttest\ntesttest\ntesttest"));
    }
}
