use std::marker::PhantomData;

use super::{record::Record, signer::Signer, time::Now, VerificationData};
use crate::dns_over_https::{types::Record as DNSRecord, Provider as DoHProvider, Query, Response};

#[derive(Clone)]
pub struct Resolver<C: Now> {
    signer: Signer,
    clock: PhantomData<C>,
}

impl<C: Now> Default for Resolver<C> {
    fn default() -> Self {
        Self {
            signer: Signer::new(),
            clock: PhantomData,
        }
    }
}

impl<C: Now> Resolver<C> {
    fn sign_record(&self, record: &DNSRecord) -> VerificationData {
        let now = C::now();
        let valid_until = now + record.ttl;
        let signature = self.signer.sign(&Record::new(record, valid_until));

        VerificationData {
            signature,
            valid_until,
            pub_key: self.signer.public_key(),
        }
    }
}

impl<C: Now> DoHProvider for Resolver<C> {
    fn resolve(&self, query: &Query) -> Option<Response> {
        let mut response: Response = Response::with_flags(false, true, true, false, false);
        response.question = query.clone();
        response.status = 0;

        response.verification_data = response
            .answer
            .first()
            .map(|record| self.sign_record(record));

        Some(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod resolver {
        use super::*;
        use crate::verifiable_dns::time::tests_utils::MockClock;

        type R = Resolver<MockClock<64>>;
        mod resolve {
            use super::*;
            #[test]
            fn resolves_vlayer_default_dkim_selector() {
                let resolver = R::default();
                let result = resolver.resolve(&"google._domainkey.vlayer.xyz".into());
                assert!(result.is_some());
            }

            #[test]
            fn question_field_equals_to_question() {
                let resolver = R::default();
                let query = "google._domainkey.vlayer.xyz".into();
                let result = resolver.resolve(&query).unwrap();

                assert_eq!(result.question, query);
            }

            #[test]
            fn status_code_is_successful() {
                let resolver = R::default();
                let query = "google._domainkey.vlayer.xyz".into();
                let result = resolver.resolve(&query).unwrap();

                assert_eq!(result.status, 0);
            }

            #[test]
            fn flags_are_set() {
                let resolver = R::default();
                let query = "google._domainkey.vlayer.xyz".into();
                let result = resolver.resolve(&query).unwrap();

                // Default values by CF
                assert!(!result.truncated);
                assert!(result.recursive_desired);
                assert!(result.recursion_available);
                assert!(!result.verified_with_dnssec);
            }
        }

        mod sign_record {

            use super::*;
            use crate::dns_over_https::types::RecordType;

            #[test]
            fn valid_until_is_ttl_seconds_from_now() {
                let resolver = R::default();
                let record = DNSRecord {
                    name: "google._domainkey.vlayer.xyz".into(),
                    record_type: RecordType::TXT,
                    ttl: 300,
                    data: "Hello".into(),
                };

                let result = resolver.sign_record(&record);
                assert_eq!(result.valid_until, 364);

                let result = Resolver::<MockClock<11>>::default().sign_record(&record);
                assert_eq!(result.valid_until, 311);
            }
        }
    }
}
