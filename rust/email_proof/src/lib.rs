mod dkim;
mod email;

use crate::email::Email;
use mail_auth::common::parse::TxtRecordParser;
use mail_auth::common::resolver::{IntoFqdn, UnwrapTxtRecord};
// use mail_auth::dkim::verify::DkimVerifier;
// use mail_auth::{
//     common::resolve::Resolve, AuthenticatedMessage, DkimResult, Error as AuthError, Txt,
// };
use mailparse::MailParseError;
// use std::sync::Arc;

pub fn parse_mime(email: &[u8]) -> Result<Email, MailParseError> {
    mailparse::parse_mail(email)?.try_into()
}

struct StaticResolver {}

// impl Resolve for StaticResolver {
//     async fn txt_lookup<'x, T: TxtRecordParser + Into<Txt> + UnwrapTxtRecord>(
//         &self,
//         _key: impl IntoFqdn<'x>,
//     ) -> mail_auth::Result<Arc<T>> {
//         const DNS_RECORDS: [&str; 2] = [
//             "brisbane._domainkey.football.example.com v=DKIM1; k=ed25519; p=11qYAYKxCrfVS/7TyWQHOg7hcvPapiMlrwIaaPcHURo=",
//             "test._domainkey.football.example.com v=DKIM1; k=rsa; p=MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDkHlOQoBTzWRiGs5V6NpP3idY6Wk08a5qhdR6wy5bdOKb2jLQiY/J16JYi0Qvx/byYzCNb3W91y3FutACDfzwQ/BC/e/8uBsCR+yz1Lxj+PL6lHvqMKrM3rG4hstT5QjvHO9PzoxZyVYLzBfO2EeC3Ip3G+2kryOTIKT+l/K4w3QIDAQAB",
//         ];
//         Ok(Arc::new(T::parse(DNS_RECORDS.join("\n").as_bytes())?))
//     }
// }
//
// pub fn mail_auth_parse(email: &[u8]) -> Result<AuthenticatedMessage, AuthError> {
//     let authenticated_message = AuthenticatedMessage::parse(email).ok_or(AuthError::ParseError)?;
//     let resolver = StaticResolver {};
//     let future = DkimVerifier::verify_dkim(&resolver, &authenticated_message);
//     let dkim_outputs = futures::executor::block_on(future);
//     if dkim_outputs.is_empty() {
//         return Err(AuthError::NoHeadersFound);
//     }
//     for output in dkim_outputs.iter() {
//         let result = output.result().clone();
//         if result == DkimResult::None {
//             return Err(AuthError::NoHeadersFound);
//         }
//         if let DkimResult::Fail(err) = result {
//             return Err(err);
//         }
//         if let DkimResult::Neutral(err) = result {
//             return Err(err);
//         }
//         if let DkimResult::PermError(err) = result {
//             return Err(err);
//         }
//         if let DkimResult::TempError(err) = result {
//             return Err(err);
//         }
//     }
//     Ok(authenticated_message)
// }

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
        assert_eq!(
            "This is a multi-part message in MIME format.\n",
            parsed.body
        );
    }
}
