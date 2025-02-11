mod responses_validation;

use std::marker::PhantomData;

use futures::future::join_all;
use responses_validation::{responses_match, validate_response};

use super::{sign_record, signer::Signer, time::Now};
use crate::{
    dns_over_https::{
        types::{Record as DNSRecord, Record, RecordType},
        Provider as DoHProvider, Query, Response,
    },
    VerificationData,
};

#[derive(thiserror::Error, Debug)]
pub enum ResolverError<PError> {
    #[error("Some responses are missing")]
    MissingResponses,
    #[error("Invalid response, {1}: {0:?}")]
    InvalidResponse(Response, String),
    #[error("Responses mismatch: {0:?} {1:?}")]
    ResponsesMismatch(Response, Response),
    #[error("Provider error: {0}")]
    ProviderError(PError),
}

#[derive(Clone)]
pub struct Resolver<C: Now, P: DoHProvider, const Q: usize> {
    providers: [P; Q],
    signer: Signer,
    clock: PhantomData<C>,
}

impl<C: Now, P: DoHProvider, const Q: usize> Resolver<C, P, Q> {
    pub fn new(providers: [P; Q]) -> Self {
        Self {
            providers,
            signer: Signer::default(),
            clock: PhantomData,
        }
    }

    fn sign_record(&self, record: &DNSRecord) -> VerificationData {
        let now = C::now();
        let valid_until = now + record.ttl;

        sign_record::sign_record(&self.signer, record, valid_until)
    }
}

fn validate_responses<PError>(responses: &[Response]) -> Result<(), ResolverError<PError>> {
    responses
        .iter()
        .map(validate_response::<PError>)
        .collect::<Result<Vec<_>, _>>()?;

    let first_response: &Response = responses.first().ok_or(ResolverError::MissingResponses)?;

    if let Some(mismatched) = responses
        .iter()
        .skip(1)
        .find(|r| !responses_match(first_response, r))
    {
        return Err(ResolverError::ResponsesMismatch(first_response.clone(), mismatched.clone()));
    }

    Ok(())
}

impl<C: Now + Sync, P: DoHProvider + Sync, const Q: usize> DoHProvider for Resolver<C, P, Q> {
    type Error = ResolverError<P::Error>;

    async fn resolve(&self, query: &Query) -> Result<Response, Self::Error> {
        let jobs: Vec<_> = self.providers.iter().map(|p| p.resolve(query)).collect();
        let responses = &join_all(jobs)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, P::Error>>()
            .map_err(ResolverError::<P::Error>::ProviderError)?;

        validate_responses(responses)?;

        let provider_response = responses
            .first()
            .ok_or(ResolverError::MissingResponses)?
            .clone();

        let mut response = Response {
            status: 0,
            question: vec![query.clone()],
            answer: keep_only_txt_and_cname_records(provider_response.answer),
            comment: provider_response.comment,
            ..Response::with_flags(false, true, true, false, false)
        };

        response.verification_data = Some(
            response
                .answer
                .as_ref()
                .and_then(|answer| answer.iter().rfind(|r| r.record_type == RecordType::TXT))
                .map(|record| self.sign_record(record))
                .ok_or(ResolverError::MissingResponses)?,
        );

        Ok(response)
    }
}

fn keep_only_txt_and_cname_records(records: Option<Vec<Record>>) -> Option<Vec<DNSRecord>> {
    records.map(|r| {
        r.into_iter()
            .filter(|r| r.record_type != RecordType::OTHER)
            .collect()
    })
}

#[cfg(test)]
pub(crate) mod tests_utils {
    use super::*;
    use crate::common::test_utils::{MockClock, MockProvider};

    pub(crate) type MockResolver = Resolver<MockClock<64>, MockProvider, 1>;

    pub(crate) fn response() -> Response {
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

    pub(crate) fn resolver() -> MockResolver {
        MockResolver::new([MockProvider::new(response())])
    }
}

#[cfg(test)]
mod tests {
    use super::{tests_utils::*, *};

    mod validate_responses {
        use super::*;

        fn responses_to_validate_responses_args<const SIZE: usize>(
            responses: &[Response; SIZE],
        ) -> [Response; SIZE] {
            responses.clone()
        }

        #[test]
        fn all_responses_must_not_be_none() {
            let responses =
                responses_to_validate_responses_args(&[response(), response(), response()]);
            assert!(validate_responses::<()>(&responses).is_ok());
        }

        #[test]
        fn passes_for_equal_results() {
            let responses =
                responses_to_validate_responses_args(&[response(), response(), response()]);
            assert!(validate_responses::<()>(&responses).is_ok());
        }

        #[test]
        fn fails_for_non_matching_responses() {
            let mut responses = [response(), response(), response()];
            responses[1].answer = Some(vec![]);

            assert!(validate_responses::<()>(&responses_to_validate_responses_args(&responses))
                .is_err());
        }

        #[test]
        fn all_responses_must_be_successful() {
            let passing_responses = responses_to_validate_responses_args(&[response(), response()]);
            let failing_responses = responses_to_validate_responses_args(&[
                Response {
                    status: 1,
                    ..response()
                },
                response(),
            ]);

            assert!(validate_responses::<()>(&passing_responses).is_ok());
            assert!(validate_responses::<()>(&failing_responses).is_err());
        }
    }

    mod resolver {
        use super::*;

        mod resolve {
            use super::*;
            use crate::common::test_utils::{MockClock, MockProvider};

            #[tokio::test]
            async fn resolves_vlayer_default_dkim_selector() {
                let resolver = resolver();
                let result = resolver
                    .resolve(&"google._domainkey.vlayer.xyz".into())
                    .await;
                assert!(result.is_ok());
            }

            #[tokio::test]
            async fn passes_when_one_of_providers_has_additional_dns_records() {
                let mut responses = [response(), response(), response()];
                responses[0].answer.as_mut().unwrap().extend_from_slice(&[
                    DNSRecord {
                        name: "vlayer.xyz".into(),
                        record_type: RecordType::OTHER,
                        data: "some data".into(),
                        ttl: 300,
                    },
                    DNSRecord {
                        name: "vlayer.xyz".into(),
                        record_type: RecordType::CNAME,
                        data: "some data".into(),
                        ttl: 300,
                    },
                ]);
                type R = Resolver<MockClock<64>, MockProvider, 3>;
                let resolver = R::new(responses.map(MockProvider::new));

                let query = "google._domainkey.vlayer.xyz".into();
                let result = resolver.resolve(&query).await.unwrap();
                let answer = result.answer.unwrap();
                assert_eq!(answer.len(), 2);
                assert_eq!(answer[0].record_type, RecordType::TXT);
                assert_eq!(answer[1].record_type, RecordType::CNAME);
            }

            #[tokio::test]
            async fn question_field_equals_to_question() {
                let resolver = resolver();
                let query = "google._domainkey.vlayer.xyz".into();
                let result = resolver.resolve(&query).await.unwrap();

                assert_eq!(result.question.len(), 1);
                assert_eq!(result.question[0], query);
            }

            #[tokio::test]
            async fn status_code_is_successful() {
                let resolver = resolver();
                let query = "google._domainkey.vlayer.xyz".into();
                let result = resolver.resolve(&query).await.unwrap();

                assert_eq!(result.status, 0);
            }

            #[tokio::test]
            async fn flags_are_set() {
                let resolver = resolver();
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
            use crate::{
                common::test_utils::{MockClock, MockProvider},
                dns_over_https::types::RecordType,
            };

            #[test]
            fn valid_until_is_ttl_seconds_from_now() {
                let resolver = resolver();
                let record = DNSRecord {
                    name: "google._domainkey.vlayer.xyz".into(),
                    record_type: RecordType::TXT,
                    ttl: 300,
                    data: "Hello".into(),
                };

                let result = resolver.sign_record(&record);
                assert_eq!(result.valid_until, 364);

                let result = Resolver::<MockClock<11>, _, 1>::new([MockProvider::new(response())])
                    .sign_record(&record);
                assert_eq!(result.valid_until, 311);
            }
        }
    }
}
