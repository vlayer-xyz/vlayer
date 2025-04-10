use std::collections::HashMap;

use base64::{Engine, engine::general_purpose};
use cfdkim::DkimPublicKey;

use crate::Error;

pub fn extract_public_key(record: &str) -> Result<DkimPublicKey, Error> {
    let tags: HashMap<&str, &str> = record
        .split(';')
        .map(str::trim)
        .map(|p| p.split_once('=').unwrap_or_default())
        .collect();

    match tags.get("k") {
        Some(&"rsa") | None => Ok(()),
        Some(v) => Err(Error::InvalidDkimRecord(format!("Invalid k tag value: {v}"))),
    }?;

    let public_key = tags
        .get("p")
        .ok_or(Error::InvalidDkimRecord("p tag missing".into()))?;

    let public_key_bytes = general_purpose::STANDARD
        .decode(public_key)
        .map_err(|e| Error::InvalidDkimRecord(format!("Public key decoding error: {e}")))?;

    Ok(DkimPublicKey::Rsa(
        rsa::pkcs8::DecodePublicKey::from_public_key_der(&public_key_bytes)
            .or_else(|_| rsa::pkcs1::DecodeRsaPublicKey::from_pkcs1_der(&public_key_bytes))
            .map_err(|_| Error::InvalidDkimRecord("Failed to parse public key".into()))?,
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_dns_record() -> anyhow::Result<()> {
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
        let public_key = extract_public_key(record)?;
        assert_eq!(public_key.key_type(), "rsa");

        Ok(())
    }

    #[test]
    fn parses_dns_record_with_missing_v_and_k_tags() -> anyhow::Result<()> {
        let record = concat!(
            " p=MIIBIjANBgkqhkiG9w0BAQEFAAOC",
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
        let public_key = extract_public_key(record)?;
        assert_eq!(public_key.key_type(), "rsa");

        Ok(())
    }

    #[test]
    fn only_rsa_is_supported() {
        let record = concat!(
            "v=DKIM1; k=ecdsa ; p=MIIBIjANBgkqhkiG9w0BAQEFAAOC",
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
        let result = extract_public_key(record);
        assert_eq!(
            result.err().unwrap().to_string(),
            "Invalid DKIM public key record: Invalid k tag value: ecdsa".to_string()
        );
    }
}
