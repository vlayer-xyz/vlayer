use std::collections::HashMap;

use base64::{engine::general_purpose, Engine};
use cfdkim::{verify_email_with_key, DkimPublicKey};
use mailparse::{MailHeaderMap, ParsedMail};
use slog::{o, Discard, Logger};

pub fn verify_email<'a>(
    raw_email: &'a [u8],
    keys: &Vec<String>,
) -> Result<ParsedMail<'a>, &'static str> {
    let email = mailparse::parse_mail(raw_email).map_err(|_| "Failed to parse email")?;
    let key = keys.first().ok_or("No DNS records provided")?;
    let dkim_public_key = parse_dns_record(key)?;
    let dkim_headers = email.headers.get_all_headers("DKIM-Signature");
    if dkim_headers.is_empty() {
        return Err("No DKIM signatures found");
    }
    let from = email
        .get_headers()
        .get_first_value("From")
        .ok_or("No From header")?;
    let logger = Logger::root(Discard, o!());
    let result = verify_email_with_key(&logger, &from, &email, dkim_public_key)
        .map_err(|_| "Failed to verify email")?;

    match result {
        result if result.with_detail().starts_with("pass") => Ok(email),
        _ => Err("DKIM verification failed"),
    }
}

fn parse_dns_record(record: &str) -> Result<DkimPublicKey, &'static str> {
    let tags: HashMap<&str, &str> = record
        .split(';')
        .map(str::trim)
        .map(|p| p.split_once('=').unwrap_or_default())
        .collect();

    let tag = tags.get("p").ok_or("DUPA")?;
    let bytes = general_purpose::STANDARD
        .decode(&tag)
        .map_err(|_| "failed to decode public key")?;

    Ok(DkimPublicKey::Rsa(
        rsa::pkcs8::DecodePublicKey::from_public_key_der(&bytes)
            .or_else(|_| rsa::pkcs1::DecodeRsaPublicKey::from_pkcs1_der(&bytes))
            .map_err(|_| "failed to parse public key")?,
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_dns_record() {
        let record = concat!(
            "v=DKIM1; k=rsa ; p=MIIBIjANBgkqhkiG9w0BAQEFAAOC",
            "AQ8AMIIBCgKCAQEAvzwKQIIWzQXv0nihasFTT3+JO23hXCg",
            "e+ESWNxCJdVLxKL5edxrumEU3DnrPeGD6q6E/vjoXwBabpm",
            "8F5o96MEPm7v12O5IIK7wx7gIJiQWvexwh+GJvW4aFFa0g1",
            "3Ai75UdZjGFNKHAEGeLmkQYybK/EHW5ymRlSg3g8zydJGEc",
            "I/melLCiBoShHjfZFJEThxLmPHNSi+KOUMypxqYHd7hzg6W",
            "7qnq6t9puZYXMWj6tEaf6ORWgb7DOXZSTJJjAJPBWa2+Urx",
            "XX6Ro7L7Xy1zzeYFCk8W5vmn0wMgGpjkWw0ljJWNwIpxZAj9",
            "p5wMedWasaPS74TZ1b7tI39ncp6QIDAQAB ; t= y : s :yy:x;",
            "s=*:email;; h= sha1:sha 256:other;; n=ignore these notes "
        );
        let result = parse_dns_record(record);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().key_type(), "rsa");
    }
}
