use mailparse::MailParseError;

use crate::Email;

pub fn build_mime_email(headers: Vec<(&str, &str)>, body: &str) -> String {
    let mut email = String::new();
    for (key, value) in headers {
        email.push_str(&format!("{key}: {value}\n"));
    }
    if !body.is_empty() {
        email.push('\n');
        email.push_str(body);
    }
    email
}

pub fn read_file(file: &str) -> Vec<u8> {
    std::fs::read(file).unwrap()
}

pub fn signed_email_fixture() -> Vec<u8> {
    read_file("./testdata/signed_email.eml")
}

pub fn unsigned_email_fixture() -> Vec<u8> {
    read_file("./testdata/email.txt")
}

pub fn parsed_email(headers: Vec<(&str, &str)>, body: &str) -> Result<Email, MailParseError> {
    let mime = build_mime_email(headers, body);
    mailparse::parse_mail(mime.as_bytes()).and_then(Email::try_from)
}
