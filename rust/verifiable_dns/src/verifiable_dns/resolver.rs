mod responses_validation;

use std::marker::PhantomData;

use futures::future::join_all;
use responses_validation::{responses_match, validate_response};

use super::{record::Record, signer::Signer, time::Now, VerificationData};
use crate::dns_over_https::{
    types::{Record as DNSRecord, RecordType},
    Provider as DoHProvider, Query, Response,
};

#[derive(Clone)]
pub struct Resolver<C: Now, P: DoHProvider, const Q: usize> {
    providers: [P; Q],
    signer: Signer,
    clock: PhantomData<C>,
}

impl<C: Now, P: DoHProvider, const Q: usize> Default for Resolver<C, P, Q> {
    fn default(providers: [P; Q]) -> Self {
        Self {
            providers,
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

fn validate_responses(responses: &[Option<Response>]) -> Option<()> {
    if responses.iter().any(|r| r.is_none()) {
        return None;
    }

    let responses: Vec<_> = responses.iter().map(|r| r.as_ref().unwrap()).collect();

    if responses.iter().any(|r| !validate_response(r)) {
        return None;
    }

    let first_response: &Response = responses.get(0)?;

    if responses
        .iter()
        .skip(1)
        .any(|r| !responses_match(first_response, *r))
    {
        return None;
    }

    Some(())
}

impl<C: Now, P: DoHProvider, const Q: usize> DoHProvider for Resolver<C, P, Q> {
    async fn resolve(&self, query: &Query) -> Option<Response> {
        let jobs: Vec<_> = self.providers.iter().map(|p| p.resolve(query)).collect();
        let responses = join_all(jobs).await;
        validate_responses(&responses)?;

        let provider_response = responses.get(0)?.as_ref()?.clone();

        let mut response = Response {
            status: 0,
            question: vec![query.clone()],
            answer: provider_response.answer,
            comment: provider_response.comment,
            ..Response::with_flags(false, true, true, false, false)
        };

        response.verification_data = if let Some(ref answer) = response.answer {
            answer
                .iter()
                .filter(|r| r.record_type == RecordType::TXT)
                .last()
                .map(|record| self.sign_record(record))
        } else {
            None
        };

        Some(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dns_over_https::types::RecordType;

    fn response() -> Response {
        let record = DNSRecord {
            name: "vlayer.xyz".into(),
            record_type: RecordType::TXT,
            data: "some data".into(),
            ttl: 300,
        };

        Response {
            status: 0,
            question: Default::default(),
            answer: Some(vec![record]),
            comment: Some("Some boring comment".to_string()),
            verification_data: None,
            ..Response::with_flags(false, true, true, false, false)
        }
    }

    mod validate_responses {
        use super::*;

        fn responses_to_validate_responses_args<const SIZE: usize>(
            responses: [Response; SIZE],
        ) -> [Option<Response>; SIZE] {
            let mut result: [Option<Response>; SIZE] = [const { None }; SIZE];
            for (i, response) in responses.iter().enumerate() {
                result[i] = Some(response.clone());
            }
            result
        }

        #[test]
        fn all_responses_must_not_be_none() {
            let responses =
                responses_to_validate_responses_args([response(), response(), response()]);
            assert!(validate_responses(&responses).is_some());
        }

        #[test]
        fn passes_for_equal_results() {
            let responses =
                responses_to_validate_responses_args([response(), response(), response()]);
            assert!(validate_responses(&responses).is_some());
        }

        #[test]
        fn fails_for_non_matching_responses() {
            let mut responses = [response(), response(), response()];
            responses[1].answer = Some(vec![]);

            assert!(validate_responses(&responses_to_validate_responses_args(responses)).is_none());
        }

        #[test]
        fn all_responses_must_be_successful() {
            let passing_responses = responses_to_validate_responses_args([response(), response()]);
            let failing_responses = responses_to_validate_responses_args([
                Response {
                    status: 1,
                    ..response()
                },
                response(),
            ]);

            assert!(validate_responses(&passing_responses).is_some());
            assert!(validate_responses(&failing_responses).is_none());
        }
    }

    mod resolver {
        use super::*;
        use crate::{
            dns_over_https::provider::test_utils::MockProvider,
            verifiable_dns::time::tests_utils::MockClock,
        };

        type R = Resolver<MockClock<64>, MockProvider, 1>;
        fn resolver() -> R {
            R::new([MockProvider::new(response())])
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
            use crate::dns_over_https::{provider::test_utils::MockProvider, types::RecordType};

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

                let result = Resolver::<MockClock<11>, _, 1>::new([MockProvider::default(response())])
                    .sign_record(&record);
                assert_eq!(result.valid_until, 311);
            }
        }
    }
}
