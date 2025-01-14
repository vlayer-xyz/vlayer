use std::marker::PhantomData;

use super::{record::Record, signer::Signer, time::Now, VerificationData};
use crate::dns_over_https::{types::Record as DNSRecord, Provider as DoHProvider, Query, Response};

#[derive(Clone)]
pub struct Resolver<C: Now, P: DoHProvider> {
    _provider: P,
    signer: Signer,
    clock: PhantomData<C>,
}

impl<C: Now, P: DoHProvider> Default for Resolver<C, P> {
    fn default(provider: P) -> Self {
        Self {
            _provider: provider,
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

impl<C: Now, P: DoHProvider> DoHProvider for Resolver<C, P> {
    async fn resolve(&self, query: &Query) -> Option<Response> {
        let mut response: Response = Response::with_flags(false, true, true, false, false);
        response.question = vec![query.clone()];
        response.status = 0;

        response.verification_data = if let Some(ref answer) = response.answer {
            answer.first().map(|record| self.sign_record(record))
        } else {
            None
        };

        Some(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod resolver {
        use super::*;
        use crate::{
            dns_over_https::provider::ExternalProvider,
            verifiable_dns::time::tests_utils::MockClock,
        };

        type R = Resolver<MockClock<64>, ExternalProvider>;
        fn resolver() -> R {
            R::new(ExternalProvider::new())
        }
        mod resolve {
            use super::*;
            #[tokio::test]
            async fn resolves_vlayer_default_dkim_selector() {
                let resolver = R::default();
                let result = resolver
                    .resolve(&"google._domainkey.vlayer.xyz".into())
                    .await;
                assert!(result.is_some());
            }

            #[tokio::test]
            async fn question_field_equals_to_question() {
                let resolver = R::default();
                let query = "google._domainkey.vlayer.xyz".into();
                let result = resolver.resolve(&query).await.unwrap();

                assert_eq!(result.question.len(), 1);
                assert_eq!(result.question[0], query);
            }

            #[tokio::test]
            async fn status_code_is_successful() {
                let resolver = R::default();
                let query = "google._domainkey.vlayer.xyz".into();
                let result = resolver.resolve(&query).await.unwrap();

                assert_eq!(result.status, 0);
            }

            #[tokio::test]
            async fn flags_are_set() {
                let resolver = R::default();
                let query = "google._domainkey.vlayer.xyz".into();
                let result = resolver.resolve(&query).await.unwrap();

                // Default values by CF
                assert!(!result.truncated);
                assert!(result.recursive_desired);
                assert!(result.recursion_available);
                assert!(!result.verified_with_dnssec);
            }
        }

        mod sign_record {

            use super::*;
            use crate::dns_over_https::{provider::ExternalProvider, types::RecordType};

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

                let result =
                    Resolver::<MockClock<11>, _>::new(ExternalProvider::default()).sign_record(&record);
                assert_eq!(result.valid_until, 311);
            }
        }
    }
}
