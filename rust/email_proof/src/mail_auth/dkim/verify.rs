/*
 * Copyright (c) 2020-2023, Stalwart Labs Ltd.
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use super::output::Output as DkimOutput;
use crate::mail_auth::{
    common::{crypto::HashAlgorithm, verify::VerifySignature},
    resolver::{domain_key::DomainKey, Resolver},
    AuthenticatedMessage, Error,
};
use std::time::SystemTime;

use super::{Flag, Signature};

impl Resolver {
    /// Verifies DKIM headers of an RFC5322 message.
    #[inline(always)]
    pub fn verify_dkim<'x>(&self, message: &'x AuthenticatedMessage<'x>) -> Vec<DkimOutput<'x>> {
        self.verify_dkim_(
            message,
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        )
    }

    pub(crate) fn verify_dkim_<'x>(
        &self,
        message: &'x AuthenticatedMessage<'x>,
        now: u64,
    ) -> Vec<DkimOutput<'x>> {
        let mut output: Vec<DkimOutput<'x>> = Vec::with_capacity(message.dkim_headers.len());

        // Validate DKIM headers
        for header in &message.dkim_headers {
            // Validate body hash
            let signature = match &header.header {
                Ok(signature) => {
                    if signature.x == 0 || (signature.x > signature.t && signature.x > now) {
                        signature
                    } else {
                        output.push(
                            DkimOutput::neutral(Error::SignatureExpired).with_signature(signature),
                        );
                        continue;
                    }
                }
                Err(err) => {
                    output.push(DkimOutput::neutral(err.clone()));
                    continue;
                }
            };

            // Validate body hash
            let ha = HashAlgorithm::from(signature.a);
            let bh = &message
                .body_hashes
                .iter()
                .find(|(c, h, l, _)| c == &signature.cb && h == &ha && l == &signature.l)
                .unwrap()
                .3;

            if bh != &signature.bh {
                output.push(
                    DkimOutput::neutral(Error::FailedBodyHashMatch).with_signature(signature),
                );
                continue;
            }

            // Obtain ._domainkey TXT record
            let record = match self
                .dkim_key(signature.domain(), signature.selector())
                .ok_or(Error::Dns("Missing record".to_string()))
            {
                Ok(record) => record,
                Err(err) => {
                    output.push(DkimOutput::dns_error(err).with_signature(signature));
                    continue;
                }
            };

            // Enforce t=s flag
            if !signature.validate_auid(record) {
                output.push(DkimOutput::fail(Error::FailedAuidMatch).with_signature(signature));
                continue;
            }

            // Hash headers
            let dkim_hdr_value = header.value.strip_signature();
            let headers = message.signed_headers(&signature.h, header.name, &dkim_hdr_value);

            // Verify signature
            if let Err(err) = record.verify(&headers, signature, signature.ch) {
                output.push(DkimOutput::fail(err).with_signature(signature));
                continue;
            }
        }

        output
    }
}

impl<'x> AuthenticatedMessage<'x> {
    pub fn signed_headers<'z: 'x>(
        &'z self,
        headers: &'x [String],
        dkim_hdr_name: &'x [u8],
        dkim_hdr_value: &'x [u8],
    ) -> impl Iterator<Item = (&'x [u8], &'x [u8])> {
        let mut last_header_pos: Vec<(&[u8], usize)> = Vec::new();
        headers
            .iter()
            .filter_map(move |h| {
                let header_pos = if let Some((_, header_pos)) = last_header_pos
                    .iter_mut()
                    .find(|(lh, _)| lh.eq_ignore_ascii_case(h.as_bytes()))
                {
                    header_pos
                } else {
                    last_header_pos.push((h.as_bytes(), 0));
                    &mut last_header_pos.last_mut().unwrap().1
                };
                if let Some((last_pos, result)) = self
                    .headers
                    .iter()
                    .rev()
                    .enumerate()
                    .skip(*header_pos)
                    .find(|(_, (mh, _))| h.as_bytes().eq_ignore_ascii_case(mh))
                {
                    *header_pos = last_pos + 1;
                    Some(*result)
                } else {
                    *header_pos = self.headers.len();
                    None
                }
            })
            .chain([(dkim_hdr_name, dkim_hdr_value)])
    }
}

impl Signature {
    #[allow(clippy::while_let_on_iterator)]
    pub(crate) fn validate_auid(&self, record: &DomainKey) -> bool {
        // Enforce t=s flag
        if !self.i.is_empty() && record.has_flag(Flag::MatchDomain) {
            let mut auid = self.i.chars();
            let mut domain = self.d.chars();
            while let Some(ch) = auid.next() {
                if ch == '@' {
                    break;
                }
            }
            while let Some(ch) = auid.next() {
                if let Some(dch) = domain.next() {
                    if ch != dch {
                        return false;
                    }
                } else {
                    break;
                }
            }
            if domain.next().is_some() {
                return false;
            }
        }

        true
    }
}

pub(crate) trait Verifier: Sized {
    fn strip_signature(&self) -> Vec<u8>;
}

impl Verifier for &[u8] {
    fn strip_signature(&self) -> Vec<u8> {
        let mut unsigned_dkim = Vec::with_capacity(self.len());
        let mut iter = self.iter().enumerate();
        let mut last_ch = b';';
        while let Some((pos, &ch)) = iter.next() {
            match ch {
                b'=' if last_ch == b'b' => {
                    unsigned_dkim.push(ch);
                    #[allow(clippy::while_let_on_iterator)]
                    while let Some((_, &ch)) = iter.next() {
                        if ch == b';' {
                            unsigned_dkim.push(b';');
                            break;
                        }
                    }
                    last_ch = 0;
                }
                b'b' | b'B' if last_ch == b';' => {
                    last_ch = b'b';
                    unsigned_dkim.push(ch);
                }
                b';' => {
                    last_ch = b';';
                    unsigned_dkim.push(ch);
                }
                b'\r' if pos == self.len() - 2 => (),
                b'\n' if pos == self.len() - 1 => (),
                _ => {
                    unsigned_dkim.push(ch);
                    if !ch.is_ascii_whitespace() {
                        last_ch = 0;
                    }
                }
            }
        }
        unsigned_dkim
    }
}

// #[cfg(test)]
// #[allow(unused)]
// mod test {
//     use std::{
//         fs,
//         path::PathBuf,
//         time::{Duration, Instant},
//     };

//     use crate::mail_auth::{
//         common::{parse::TxtRecordParser, verify::DomainKey},
//         dkim::verify::Verifier,
//         AuthenticatedMessage, DkimResult, Resolver,
//     };

//     #[tokio::test]
//     async fn dkim_verify() {
//         let mut test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
//         test_dir.push("resources");
//         test_dir.push("dkim");

//         for file_name in fs::read_dir(&test_dir).unwrap() {
//             let file_name = file_name.unwrap().path();
//             /*if !file_name.to_str().unwrap().contains("002") {
//                 continue;
//             }*/
//             println!("DKIM verifying {}", file_name.to_str().unwrap());

//             let test = String::from_utf8(fs::read(&file_name).unwrap()).unwrap();
//             let (dns_records, raw_message) = test.split_once("\n\n").unwrap();
//             let resolver = new_resolver(dns_records);
//             let raw_message = raw_message.replace('\n', "\r\n");
//             let message = AuthenticatedMessage::parse(raw_message.as_bytes()).unwrap();

//             let dkim = resolver.verify_dkim_(&message, 1667843664).await;

//             assert_eq!(dkim.last().unwrap().result(), &DkimResult::Pass);
//         }
//     }

//     #[test]
//     fn dkim_strip_signature() {
//         for (value, stripped_value) in [
//             ("b=abc;h=From\r\n", "b=;h=From"),
//             ("bh=B64b=;h=From;b=abc\r\n", "bh=B64b=;h=From;b="),
//             ("h=From; b = abc\r\ndef\r\n; v=1\r\n", "h=From; b =; v=1"),
//             ("B\r\n=abc;v=1\r\n", "B\r\n=;v=1"),
//         ] {
//             assert_eq!(
//                 String::from_utf8(value.as_bytes().strip_signature()).unwrap(),
//                 stripped_value
//             );
//         }
//     }

//     fn new_resolver(dns_records: &str) -> Resolver {
//         let resolver = Resolver::new_system_conf().unwrap();
//         for (key, value) in dns_records
//             .split('\n')
//             .filter_map(|r| r.split_once(' ').map(|(a, b)| (a, b.as_bytes())))
//         {
//             #[cfg(any(test, feature = "test"))]
//             resolver.txt_add(
//                 format!("{key}."),
//                 DomainKey::parse(value).unwrap(),
//                 Instant::now() + Duration::new(3200, 0),
//             );
//         }

//         resolver
//     }
// }
