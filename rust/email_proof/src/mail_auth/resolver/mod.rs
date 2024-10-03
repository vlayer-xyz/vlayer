use std::{collections::HashMap, time::SystemTime};

use domain_key::DomainKey;

use super::{authenticated_message::AuthenticatedMessage, error::Error};
use crate::mail_auth::{
    common::{crypto::HashAlgorithm, verify::VerifySignature},
    dkim::{output::Output as DkimOutput, verify::Verifier},
};

pub(crate) mod domain_key;

type Domain = (String, String);

pub struct Resolver {
    domains: HashMap<Domain, DomainKey>,
}

impl Resolver {
    pub fn dkim_key(&self, domain: &str, selector: &str) -> Option<&DomainKey> {
        self.domains.get(&(domain.to_owned(), selector.to_owned()))
    }

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

impl From<Vec<(Domain, DomainKey)>> for Resolver {
    fn from(value: Vec<(Domain, DomainKey)>) -> Self {
        let domains = value.into_iter().collect();
        Self { domains }
    }
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;

    use super::*;

    lazy_static! {
        static ref NEWENGLAND_SELECTOR_KEY: DomainKey = "v=DKIM1; \
    p=MIGJAoGBALVI635dLK4cJJAH3Lx6upo3X/Lm1tQz3mezcWTA3BUBnyIsdnRf57aD5BtNmhPrYYDlWlzw3UgnKisIxktkk5+iMQMlFtAS10JB8L3YadXNJY+JBcbeSi5TgJe4WFzNgW95FWDAuSTRXSWZfA/8xjflbTLDx0euFZOM7C4T0GwLAgMBAAE=".to_string().try_into().unwrap();
    }

    fn resolver_fixture() -> Resolver {
        vec![(
            ("example.com".to_string(), "newengland".to_string()),
            (*NEWENGLAND_SELECTOR_KEY).clone(),
        )]
        .into()
    }

    mod dkim_key {
        use std::ops::Deref;

        use super::*;

        #[test]
        fn returns_key_for_a_valid_domain_selector_pair() {
            let resolver = resolver_fixture();

            let domain = "example.com";
            let selector = "newengland";

            assert_eq!(
                resolver.dkim_key(domain, selector).unwrap(),
                NEWENGLAND_SELECTOR_KEY.deref()
            )
        }

        #[test]
        fn returns_none_for_invalid_domain_selector_pairs() {
            let resolver = resolver_fixture();

            let domain = "example.com";
            let selector = "football";

            assert_eq!(resolver.dkim_key(domain, selector), None)
        }
    }
}
