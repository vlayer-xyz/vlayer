// /*
//  * Copyright (c) 2020-2023, Stalwart Labs Ltd.
//  *
//  * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//  * https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//  * <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
//  * option. This file may not be copied, modified, or distributed
//  * except according to those terms.
//  */
// use std::slice::Iter;

// use mail_parser::decoders::base64::base64_decode_stream;

use crate::mail_auth::common::verify::DomainKey;
// use crate::mail_auth::{
//     common::{crypto::VerifyingKeyType, parse::*, verify::DomainKey},
//     dkim::{RR_EXPIRATION, RR_SIGNATURE, RR_UNKNOWN_TAG, RR_VERIFICATION},
//     Error,
// };

// use super::{
//     Algorithm, Atps, Canonicalization, DomainKeyReport, Flag, HashAlgorithm, Service, Signature,
//     Version, RR_DNS, RR_OTHER, RR_POLICY,
// };

// const ATPSH: u64 = (b'a' as u64)
//     | (b't' as u64) << 8
//     | (b'p' as u64) << 16
//     | (b's' as u64) << 24
//     | (b'h' as u64) << 32;
// const ATPS: u64 = (b'a' as u64) | (b't' as u64) << 8 | (b'p' as u64) << 16 | (b's' as u64) << 24;
// const NONE: u64 = (b'n' as u64) | (b'o' as u64) << 8 | (b'n' as u64) << 16 | (b'e' as u64) << 24;
// const SHA256: u64 = (b's' as u64)
//     | (b'h' as u64) << 8
//     | (b'a' as u64) << 16
//     | (b'2' as u64) << 24
//     | (b'5' as u64) << 32
//     | (b'6' as u64) << 40;
// const SHA1: u64 = (b's' as u64) | (b'h' as u64) << 8 | (b'a' as u64) << 16 | (b'1' as u64) << 24;
// const RA: u64 = (b'r' as u64) | (b'a' as u64) << 8;
// const RP: u64 = (b'r' as u64) | (b'p' as u64) << 8;
// const RR: u64 = (b'r' as u64) | (b'r' as u64) << 8;
// const RS: u64 = (b'r' as u64) | (b's' as u64) << 8;
// const ALL: u64 = (b'a' as u64) | (b'l' as u64) << 8 | (b'l' as u64) << 16;

// impl Signature {
//     #[allow(clippy::while_let_on_iterator)]
//     pub fn parse(header: &'_ [u8]) -> crate::mail_auth::Result<Self> {
//         let mut signature = Signature {
//             v: 0,
//             a: Algorithm::RsaSha256,
//             d: "".into(),
//             s: "".into(),
//             i: "".into(),
//             b: Vec::with_capacity(0),
//             bh: Vec::with_capacity(0),
//             h: Vec::with_capacity(0),
//             z: Vec::with_capacity(0),
//             l: 0,
//             x: 0,
//             t: 0,
//             ch: Canonicalization::Simple,
//             cb: Canonicalization::Simple,
//             r: false,
//             atps: None,
//             atpsh: None,
//         };
//         let header_len = header.len();
//         let mut header = header.iter();

//         while let Some(key) = header.key() {
//             match key {
//                 V => {
//                     signature.v = header.number().unwrap_or(0) as u32;
//                     if signature.v != 1 {
//                         return Err(Error::UnsupportedVersion);
//                     }
//                 }
//                 A => {
//                     signature.a = header.algorithm()?;
//                 }
//                 B => {
//                     signature.b =
//                         base64_decode_stream(&mut header, header_len, b';').ok_or(Error::Base64)?
//                 }
//                 BH => {
//                     signature.bh =
//                         base64_decode_stream(&mut header, header_len, b';').ok_or(Error::Base64)?
//                 }
//                 C => {
//                     let (ch, cb) = header.canonicalization(Canonicalization::Simple)?;
//                     signature.ch = ch;
//                     signature.cb = cb;
//                 }
//                 D => signature.d = header.text(true),
//                 H => signature.h = header.items(),
//                 I => signature.i = header.text_qp(Vec::with_capacity(20), true, false),
//                 L => signature.l = header.number().unwrap_or(0),
//                 S => signature.s = header.text(true),
//                 T => signature.t = header.number().unwrap_or(0),
//                 X => signature.x = header.number().unwrap_or(0),
//                 Z => signature.z = header.headers_qp(),
//                 R => signature.r = header.value() == Y,
//                 ATPS => {
//                     if signature.atps.is_none() {
//                         signature.atps = Some(header.text(true));
//                     }
//                 }
//                 ATPSH => {
//                     signature.atpsh = match header.value() {
//                         SHA256 => HashAlgorithm::Sha256.into(),
//                         SHA1 => HashAlgorithm::Sha1.into(),
//                         NONE => None,
//                         _ => {
//                             signature.atps = Some("".into());
//                             None
//                         }
//                     };
//                 }
//                 _ => header.ignore(),
//             }
//         }

//         if !signature.d.is_empty()
//             && !signature.s.is_empty()
//             && !signature.b.is_empty()
//             && !signature.bh.is_empty()
//             && !signature.h.is_empty()
//         {
//             Ok(signature)
//         } else {
//             Err(Error::MissingParameters)
//         }
//     }
// }

// pub(crate) trait SignatureParser: Sized {
//     fn canonicalization(
//         &mut self,
//         default: Canonicalization,
//     ) -> crate::mail_auth::Result<(Canonicalization, Canonicalization)>;
//     fn algorithm(&mut self) -> crate::mail_auth::Result<Algorithm>;
// }

// impl SignatureParser for Iter<'_, u8> {
//     fn canonicalization(
//         &mut self,
//         default: Canonicalization,
//     ) -> crate::mail_auth::Result<(Canonicalization, Canonicalization)> {
//         let mut cb = default;
//         let mut ch = default;

//         let mut has_header = false;
//         let mut c = None;

//         while let Some(char) = self.next() {
//             match (char, c) {
//                 (b's' | b'S', None) => {
//                     if self.match_bytes(b"imple") {
//                         c = Canonicalization::Simple.into();
//                     } else {
//                         return Err(Error::UnsupportedCanonicalization);
//                     }
//                 }
//                 (b'r' | b'R', None) => {
//                     if self.match_bytes(b"elaxed") {
//                         c = Canonicalization::Relaxed.into();
//                     } else {
//                         return Err(Error::UnsupportedCanonicalization);
//                     }
//                 }
//                 (b'/', Some(c_)) => {
//                     ch = c_;
//                     c = None;
//                     has_header = true;
//                 }
//                 (b';', _) => {
//                     break;
//                 }
//                 (_, _) => {
//                     if !char.is_ascii_whitespace() {
//                         return Err(Error::UnsupportedCanonicalization);
//                     }
//                 }
//             }
//         }

//         if let Some(c) = c {
//             if has_header {
//                 cb = c;
//             } else {
//                 ch = c;
//             }
//         }

//         Ok((ch, cb))
//     }

//     fn algorithm(&mut self) -> crate::mail_auth::Result<Algorithm> {
//         match self.next_skip_whitespaces().unwrap_or(0) {
//             b'r' | b'R' => {
//                 if self.match_bytes(b"sa-sha") {
//                     let mut algo = 0;

//                     for ch in self {
//                         match ch {
//                             b'1' if algo == 0 => algo = 1,
//                             b'2' if algo == 0 => algo = 2,
//                             b'5' if algo == 2 => algo = 25,
//                             b'6' if algo == 25 => algo = 256,
//                             b';' => {
//                                 break;
//                             }
//                             _ => {
//                                 if !ch.is_ascii_whitespace() {
//                                     return Err(Error::UnsupportedAlgorithm);
//                                 }
//                             }
//                         }
//                     }

//                     match algo {
//                         256 => Ok(Algorithm::RsaSha256),
//                         1 => Ok(Algorithm::RsaSha1),
//                         _ => Err(Error::UnsupportedAlgorithm),
//                     }
//                 } else {
//                     Err(Error::UnsupportedAlgorithm)
//                 }
//             }
//             b'e' | b'E' => {
//                 if self.match_bytes(b"d25519-sha256") && self.seek_tag_end() {
//                     Ok(Algorithm::Ed25519Sha256)
//                 } else {
//                     Err(Error::UnsupportedAlgorithm)
//                 }
//             }
//             _ => Err(Error::UnsupportedAlgorithm),
//         }
//     }
// }

// impl TxtRecordParser for DomainKey {
//     #[allow(clippy::while_let_on_iterator)]
//     fn parse(header: &[u8]) -> crate::mail_auth::Result<Self> {
//         let header_len = header.len();
//         let mut header = header.iter();
//         let mut flags = 0;
//         let mut key_type = VerifyingKeyType::Rsa;
//         let mut public_key = None;

//         while let Some(key) = header.key() {
//             match key {
//                 V => {
//                     if !header.match_bytes(b"DKIM1") || !header.seek_tag_end() {
//                         return Err(Error::InvalidRecordType);
//                     }
//                 }
//                 H => flags |= header.flags::<HashAlgorithm>(),
//                 P => {
//                     if let Some(bytes) = base64_decode_stream(&mut header, header_len, b';') {
//                         public_key = Some(bytes);
//                     }
//                 }
//                 S => flags |= header.flags::<Service>(),
//                 T => flags |= header.flags::<Flag>(),
//                 K => {
//                     if let Some(ch) = header.next_skip_whitespaces() {
//                         match ch {
//                             b'r' | b'R' => {
//                                 if header.match_bytes(b"sa") && header.seek_tag_end() {
//                                     key_type = VerifyingKeyType::Rsa;
//                                 } else {
//                                     return Err(Error::UnsupportedKeyType);
//                                 }
//                             }
//                             b'e' | b'E' => {
//                                 if header.match_bytes(b"d25519") && header.seek_tag_end() {
//                                     key_type = VerifyingKeyType::Ed25519;
//                                 } else {
//                                     return Err(Error::UnsupportedKeyType);
//                                 }
//                             }
//                             b';' => (),
//                             _ => {
//                                 return Err(Error::UnsupportedKeyType);
//                             }
//                         }
//                     }
//                 }
//                 _ => {
//                     header.ignore();
//                 }
//             }
//         }

//         match public_key {
//             Some(public_key) => Ok(DomainKey {
//                 p: key_type.verifying_key(&public_key)?,
//                 f: flags,
//             }),
//             _ => Err(Error::InvalidRecordType),
//         }
//     }
// }

// impl TxtRecordParser for DomainKeyReport {
//     #[allow(clippy::while_let_on_iterator)]
//     fn parse(header: &[u8]) -> crate::mail_auth::Result<Self> {
//         let mut header = header.iter();
//         let mut record = DomainKeyReport {
//             ra: String::new(),
//             rp: 100,
//             rr: u8::MAX,
//             rs: None,
//         };

//         while let Some(key) = header.key() {
//             match key {
//                 RA => {
//                     record.ra = header.text_qp(Vec::with_capacity(20), true, false);
//                 }
//                 RP => {
//                     record.rp = std::cmp::min(header.number().unwrap_or(0), 100) as u8;
//                 }
//                 RS => {
//                     record.rs = header.text_qp(Vec::with_capacity(20), false, false).into();
//                 }
//                 RR => {
//                     record.rr = 0;
//                     loop {
//                         let (val, stop_char) = header.flag_value();
//                         match val {
//                             ALL => {
//                                 record.rr = u8::MAX;
//                             }
//                             D => {
//                                 record.rr |= RR_DNS;
//                             }
//                             O => {
//                                 record.rr |= RR_OTHER;
//                             }
//                             P => {
//                                 record.rr |= RR_POLICY;
//                             }
//                             S => {
//                                 record.rr |= RR_SIGNATURE;
//                             }
//                             U => {
//                                 record.rr |= RR_UNKNOWN_TAG;
//                             }
//                             V => {
//                                 record.rr |= RR_VERIFICATION;
//                             }
//                             X => {
//                                 record.rr |= RR_EXPIRATION;
//                             }
//                             _ => (),
//                         }

//                         if stop_char != b':' {
//                             break;
//                         }
//                     }
//                 }

//                 _ => {
//                     header.ignore();
//                 }
//             }
//         }

//         if !record.ra.is_empty() {
//             Ok(record)
//         } else {
//             Err(Error::InvalidRecordType)
//         }
//     }
// }

// impl TxtRecordParser for Atps {
//     #[allow(clippy::while_let_on_iterator)]
//     fn parse(header: &[u8]) -> crate::mail_auth::Result<Self> {
//         let mut header = header.iter();
//         let mut record = Atps {
//             v: Version::V1,
//             d: None,
//         };
//         let mut has_version = false;

//         while let Some(key) = header.key() {
//             match key {
//                 V => {
//                     if !header.match_bytes(b"ATPS1") || !header.seek_tag_end() {
//                         return Err(Error::InvalidRecordType);
//                     }
//                     has_version = true;
//                 }
//                 D => {
//                     record.d = header.text(true).into();
//                 }
//                 _ => {
//                     header.ignore();
//                 }
//             }
//         }

//         if !has_version {
//             return Err(Error::InvalidRecordType);
//         }

//         Ok(record)
//     }
// }

impl DomainKey {
    pub fn has_flag(&self, flag: impl Into<u64>) -> bool {
        (self.f & flag.into()) != 0
    }
}

// impl ItemParser for HashAlgorithm {
//     fn parse(bytes: &[u8]) -> Option<Self> {
//         if bytes.eq_ignore_ascii_case(b"sha256") {
//             HashAlgorithm::Sha256.into()
//         } else if bytes.eq_ignore_ascii_case(b"sha1") {
//             HashAlgorithm::Sha1.into()
//         } else {
//             None
//         }
//     }
// }

// impl ItemParser for Flag {
//     fn parse(bytes: &[u8]) -> Option<Self> {
//         if bytes.eq_ignore_ascii_case(b"y") {
//             Flag::Testing.into()
//         } else if bytes.eq_ignore_ascii_case(b"s") {
//             Flag::MatchDomain.into()
//         } else {
//             None
//         }
//     }
// }

// impl ItemParser for Service {
//     fn parse(bytes: &[u8]) -> Option<Self> {
//         if bytes.eq(b"*") {
//             Service::All.into()
//         } else if bytes.eq_ignore_ascii_case(b"email") {
//             Service::Email.into()
//         } else {
//             None
//         }
//     }
// }

// #[cfg(test)]
// mod test {
//     use mail_parser::decoders::base64::base64_decode;

//     use crate::mail_auth::{
//         common::{
//             crypto::{Algorithm, R_HASH_SHA1, R_HASH_SHA256},
//             parse::TxtRecordParser,
//             verify::DomainKey,
//         },
//         dkim::{
//             Canonicalization, DomainKeyReport, Signature, RR_DNS, RR_EXPIRATION, RR_OTHER,
//             RR_POLICY, RR_SIGNATURE, RR_UNKNOWN_TAG, RR_VERIFICATION, R_FLAG_MATCH_DOMAIN,
//             R_FLAG_TESTING, R_SVC_ALL, R_SVC_EMAIL,
//         },
//     };

//     #[test]
//     fn dkim_signature_parse() {
//         for (signature, expected_result) in [
//             (
//                 concat!(
//                     "v=1; a=rsa-sha256; s=default; d=stalw.art; c=relaxed/relaxed; ",
//                     "bh=QoiUNYyUV+1tZ/xUPRcE+gST2zAStvJx1OK078Ylm5s=; ",
//                     "b=Du0rvdzNodI6b5bhlUaZZ+gpXJi0VwjY/3qL7lS0wzKutNVCbvdJuZObGdAcv\n",
//                     " eVI/RNQh2gxW4H2ynMS3B+Unse1YLJQwdjuGxsCEKBqReKlsEKT8JlO/7b2AvxR\n",
//                     "\t9Q+M2aHD5kn9dbNIKnN/PKouutaXmm18QwL5EPEN9DHXSqQ=;",
//                     "h=Subject:To:From; t=311923920",
//                 ),
//                 Signature {
//                     v: 1,
//                     a: Algorithm::RsaSha256,
//                     d: "stalw.art".into(),
//                     s: "default".into(),
//                     i: "".into(),
//                     bh: base64_decode(b"QoiUNYyUV+1tZ/xUPRcE+gST2zAStvJx1OK078Ylm5s=").unwrap(),
//                     b: base64_decode(
//                         concat!(
//                             "Du0rvdzNodI6b5bhlUaZZ+gpXJi0VwjY/3qL7lS0wzKutNVCbvdJuZObGdAcv",
//                             "eVI/RNQh2gxW4H2ynMS3B+Unse1YLJQwdjuGxsCEKBqReKlsEKT8JlO/7b2AvxR",
//                             "9Q+M2aHD5kn9dbNIKnN/PKouutaXmm18QwL5EPEN9DHXSqQ="
//                         )
//                         .as_bytes(),
//                     )
//                     .unwrap(),
//                     h: vec!["Subject".into(), "To".into(), "From".into()],
//                     z: vec![],
//                     l: 0,
//                     x: 0,
//                     t: 311923920,
//                     ch: Canonicalization::Relaxed,
//                     cb: Canonicalization::Relaxed,
//                     r: false,
//                     atps: None,
//                     atpsh: None,
//                 },
//             ),
//             (
//                 concat!(
//                     "v=1; a=rsa-sha1; d=example.net; s=brisbane;\r\n",
//                     " c=simple; q=dns/txt; i=@eng.example.net;\r\n",
//                     " t=1117574938; x=1118006938;\r\n",
//                     " h=from:to:subject:date;\r\n",
//                     " z=From:foo@eng.example.net|To:joe@example.com|\r\n",
//                     " Subject:demo=20run|Date:July=205,=202005=203:44:08=20PM=20-0700;\r\n",
//                     " bh=MTIzNDU2Nzg5MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTI=;\r\n",
//                     " b=dzdVyOfAKCdLXdJOc9G2q8LoXSlEniSbav+yuU4zGeeruD00lszZVoG4ZHRNiYzR",
//                 ),
//                 Signature {
//                     v: 1,
//                     a: Algorithm::RsaSha1,
//                     d: "example.net".into(),
//                     s: "brisbane".into(),
//                     i: "@eng.example.net".into(),
//                     bh: base64_decode(b"MTIzNDU2Nzg5MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTI=").unwrap(),
//                     b: base64_decode(
//                         concat!(
//                             "dzdVyOfAKCdLXdJOc9G2q8LoXSlEniSbav+yuU4zGe",
//                             "eruD00lszZVoG4ZHRNiYzR"
//                         )
//                         .as_bytes(),
//                     )
//                     .unwrap(),
//                     h: vec!["from".into(), "to".into(), "subject".into(), "date".into()],
//                     z: vec![
//                         "From:foo@eng.example.net".into(),
//                         "To:joe@example.com".into(),
//                         "Subject:demo run".into(),
//                         "Date:July 5, 2005 3:44:08 PM -0700".into(),
//                     ],
//                     l: 0,
//                     x: 1118006938,
//                     t: 1117574938,
//                     ch: Canonicalization::Simple,
//                     cb: Canonicalization::Simple,
//                     r: false,
//                     atps: None,
//                     atpsh: None,
//                 },
//             ),
//             (
//                 concat!(
//                     "v=1; a = rsa - sha256; s = brisbane; d = example.com;  \r\n",
//                     "c = simple / relaxed; q=dns/txt; i = \r\n joe=20@\r\n",
//                     " football.example.com; \r\n",
//                     "h=Received : From : To :\r\n Subject : : Date : Message-ID::;;;; \r\n",
//                     "bh=2jUSOH9NhtVGCQWNr9BrIAPreKQjO6Sn7XIkfJVOzv8=; \r\n",
//                     "b=AuUoFEfDxTDkHlLXSZEpZj79LICEps6eda7W3deTVFOk4yAUoqOB \r\n",
//                     "4nujc7YopdG5dWLSdNg6xNAZpOPr+kHxt1IrE+NahM6L/LbvaHut \r\n",
//                     "KVdkLLkpVaVVQPzeRDI009SO2Il5Lu7rDNH6mZckBdrIx0orEtZV \r\n",
//                     "4bmp/YzhwvcubU4=; l = 123",
//                 ),
//                 Signature {
//                     v: 1,
//                     a: Algorithm::RsaSha256,
//                     d: "example.com".into(),
//                     s: "brisbane".into(),
//                     i: "joe @football.example.com".into(),
//                     bh: base64_decode(b"2jUSOH9NhtVGCQWNr9BrIAPreKQjO6Sn7XIkfJVOzv8=").unwrap(),
//                     b: base64_decode(
//                         concat!(
//                             "AuUoFEfDxTDkHlLXSZEpZj79LICEps6eda7W3deTVFOk4yAUoqOB",
//                             "4nujc7YopdG5dWLSdNg6xNAZpOPr+kHxt1IrE+NahM6L/LbvaHut",
//                             "KVdkLLkpVaVVQPzeRDI009SO2Il5Lu7rDNH6mZckBdrIx0orEtZV",
//                             "4bmp/YzhwvcubU4="
//                         )
//                         .as_bytes(),
//                     )
//                     .unwrap(),
//                     h: vec![
//                         "Received".into(),
//                         "From".into(),
//                         "To".into(),
//                         "Subject".into(),
//                         "Date".into(),
//                         "Message-ID".into(),
//                     ],
//                     z: vec![],
//                     l: 123,
//                     x: 0,
//                     t: 0,
//                     ch: Canonicalization::Simple,
//                     cb: Canonicalization::Relaxed,
//                     r: false,
//                     atps: None,
//                     atpsh: None,
//                 },
//             ),
//         ] {
//             let result = Signature::parse(signature.as_bytes()).unwrap();
//             assert_eq!(result.v, expected_result.v, "{signature:?}");
//             assert_eq!(result.a, expected_result.a, "{signature:?}");
//             assert_eq!(result.d, expected_result.d, "{signature:?}");
//             assert_eq!(result.s, expected_result.s, "{signature:?}");
//             assert_eq!(result.i, expected_result.i, "{signature:?}");
//             assert_eq!(result.b, expected_result.b, "{signature:?}");
//             assert_eq!(result.bh, expected_result.bh, "{signature:?}");
//             assert_eq!(result.h, expected_result.h, "{signature:?}");
//             assert_eq!(result.z, expected_result.z, "{signature:?}");
//             assert_eq!(result.l, expected_result.l, "{signature:?}");
//             assert_eq!(result.x, expected_result.x, "{signature:?}");
//             assert_eq!(result.t, expected_result.t, "{signature:?}");
//             assert_eq!(result.ch, expected_result.ch, "{signature:?}");
//             assert_eq!(result.cb, expected_result.cb, "{signature:?}");
//         }
//     }

//     #[test]
//     fn dkim_record_parse() {
//         for (record, expected_result) in [
//             (
//                 concat!(
//                     "v=DKIM1; p=MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQ",
//                     "KBgQDwIRP/UC3SBsEmGqZ9ZJW3/DkMoGeLnQg1fWn7/zYt",
//                     "IxN2SnFCjxOCKG9v3b4jYfcTNh5ijSsq631uBItLa7od+v",
//                     "/RtdC2UzJ1lWT947qR+Rcac2gbto/NMqJ0fzfVjH4OuKhi",
//                     "tdY9tf6mcwGjaNBcWToIMmPSPDdQPNUYckcQ2QIDAQAB",
//                 ),
//                 0,
//             ),
//             (
//                 concat!(
//                     "v=DKIM1; k=rsa; p=MIIBIjANBgkqhkiG9w0BAQEFAAOC",
//                     "AQ8AMIIBCgKCAQEAvzwKQIIWzQXv0nihasFTT3+JO23hXCg",
//                     "e+ESWNxCJdVLxKL5edxrumEU3DnrPeGD6q6E/vjoXwBabpm",
//                     "8F5o96MEPm7v12O5IIK7wx7gIJiQWvexwh+GJvW4aFFa0g1",
//                     "3Ai75UdZjGFNKHAEGeLmkQYybK/EHW5ymRlSg3g8zydJGEc",
//                     "I/melLCiBoShHjfZFJEThxLmPHNSi+KOUMypxqYHd7hzg6W",
//                     "7qnq6t9puZYXMWj6tEaf6ORWgb7DOXZSTJJjAJPBWa2+Urx",
//                     "XX6Ro7L7Xy1zzeYFCk8W5vmn0wMgGpjkWw0ljJWNwIpxZAj9",
//                     "p5wMedWasaPS74TZ1b7tI39ncp6QIDAQAB ; t= y : s :yy:x;",
//                     "s=*:email;; h= sha1:sha 256:other;; n=ignore these notes "
//                 ),
//                 R_HASH_SHA1
//                     | R_HASH_SHA256
//                     | R_SVC_ALL
//                     | R_SVC_EMAIL
//                     | R_FLAG_MATCH_DOMAIN
//                     | R_FLAG_TESTING,
//             ),
//             (
//                 concat!(
//                     "p=MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCYtb/9Sh8nGKV7exhUFS",
//                     "+cBNXlHgO1CxD9zIfQd5ztlq1LO7g38dfmFpQafh9lKgqPBTolFhZxhF1yUNT",
//                     "hpV673NdAtaCVGNyx/fTYtvyyFe9DH2tmm/ijLlygDRboSkIJ4NHZjK++48hk",
//                     "NP8/htqWHS+CvwWT4Qgs0NtB7Re9bQIDAQAB"
//                 ),
//                 0,
//             ),
//         ] {
//             assert_eq!(
//                 DomainKey::parse(record.as_bytes()).unwrap().f,
//                 expected_result
//             );
//         }
//     }

//     #[test]
//     fn dkim_report_record_parse() {
//         for (record, expected_result) in [
//             (
//                 "ra=dkim-errors; rp=97; rr=v:x",
//                 DomainKeyReport {
//                     ra: "dkim-errors".to_string(),
//                     rp: 97,
//                     rr: RR_VERIFICATION | RR_EXPIRATION,
//                     rs: None,
//                 },
//             ),
//             (
//                 "ra=postmaster; rp=1; rr=d:o:p:s:u:v:x; rs=Error=20Message;",
//                 DomainKeyReport {
//                     ra: "postmaster".to_string(),
//                     rp: 1,
//                     rr: RR_DNS
//                         | RR_OTHER
//                         | RR_POLICY
//                         | RR_SIGNATURE
//                         | RR_UNKNOWN_TAG
//                         | RR_VERIFICATION
//                         | RR_EXPIRATION,
//                     rs: "Error Message".to_string().into(),
//                 },
//             ),
//         ] {
//             assert_eq!(
//                 DomainKeyReport::parse(record.as_bytes()).unwrap(),
//                 expected_result
//             );
//         }
//     }
// }
