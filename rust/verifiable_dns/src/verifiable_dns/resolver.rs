mod responses_validation;

use std::marker::PhantomData;

use futures::future::join_all;
use responses_validation::{responses_match, validate_response};

use super::{sign_record, signer::Signer, time::Now};
use crate::{
    VerificationData,
    dns_over_https::{
        Provider as DoHProvider, Query, Response,
        types::{Record as DNSRecord, Record, RecordType},
    },
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

    pub fn with_key(mut self, priv_key: &str) -> Self {
        self.signer = Signer::new(priv_key);
        self
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

            assert!(
                validate_responses::<()>(&responses_to_validate_responses_args(&responses))
                    .is_err()
            );
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

        mod constructor {
            use super::*;

            const TEST_PRIVATE_KEY: &str = "-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQC2xCxRXxCmqvKC
xj7b4kJDoXDz+iYzvUgzY39Hyk9vNuA6XSnvwxkayA85DYdLOeMPQU/Owfyg7YHl
R+3CzTgsdvYckBiXPbn6U3lyp8cB9rd+CYLfwV/AGSfuXnzZS09Zn/BwE6fIKBvf
Ity8mtfKu3xDEcmC9Y7bchOtRVizMiZtdDrtgZLRiEytuLFHOaja2mbclwgG2ces
RQyxPQ18V1+xmFNPxhvEG8DwV04OATDHu7+9/cn2puLj4q/xy+rIm6V4hFKNVc+w
gyeh6MifTgA88oiOkzJB2daVvLus3JC0Tj4JX6NwWOolsT9eKVy+rG3oOKuMUK9h
4piXW4cvAgMBAAECggEAfsyDYsDtsHQRZCFeIvdKudkboGkAcAz2NpDlEU2O5r3P
uy4/lhRpKmd6CD8Wil5S5ZaOZAe52XxuDkBk+C2gt1ihTxe5t9QfX0jijWVRcE9W
5p56qfpjD8dkKMBtJeRV3PxVt6wrT3ZkP97T/hX/eKuyfmWsxKrQvfbbJ+9gppEM
XEoIXtQydasZwdmXoyxu/8598tGTX25gHu3hYaErXMJ8oh+B0smcPR6gjpDjBTqw
m++nJN7w0MOjwel0DA2fdhJqFJ7Aqn2AeCBUhCVNlR2wfEz5H7ZFTAlliP1ZJNur
6zWcogJSaNAE+dZus9b3rcETm61A8W3eY54RZHN2wQKBgQDcwGEkLU6Sr67nKsUT
ymW593A2+b1+Dm5hRhp+92VCJewVPH5cMaYVem5aE/9uF46HWMHLM9nWu+MXnvGJ
mOQi7Ny+149Oz9vl9PzYrsLJ0NyGRzypvRbZ0jjSH7Xd776xQ8ph0L1qqNkfM6CX
eQ6WQNvJEIXcXyY0O6MTj2stZwKBgQDT8xR1fkDpVINvkr4kI2ry8NoEo0ZTwYCv
Z+lgCG2T/eZcsj79nQk3R2L1mB42GEmvaM3XU5T/ak4G62myCeQijbLfpw5A9/l1
ClKBdmR7eI0OV3eiy4si480mf/cLTzsC06r7DhjFkKVksDGIsKpfxIFWsHYiIUJD
vRIn76fy+QKBgQDOaLesGw0QDWNuVUiHU8XAmEP9s5DicF33aJRXyb2Nl2XjCXhh
fi78gEj0wyQgbbhgh7ZU6Xuz1GTn7j+M2D/hBDb33xjpqWPE5kkR1n7eNAQvLibj
06GtNGra1rm39ncIywlOYt7p/01dZmmvmIryJV0c6O0xfGp9hpHaNU0S2wKBgCX2
5ZRCIChrTfu/QjXA7lhD0hmAkYlRINbKeyALgm0+znOOLgBJj6wKKmypacfww8oa
sLxAKXEyvnU4177fTLDvxrmO99ulT1aqmaq85TTEnCeUfUZ4xRxjx4x84WhyMbTI
61h65u8EgMuvT8AXPP1Yen5nr1FfubnedREYOXIpAoGAMZlUBtQGIHyt6uo1s40E
DF+Kmhrggn6e0GsVPYO2ghk1tLNqgr6dVseRtYwnJxpXk9U6HWV8CJl5YLFDPlFx
mH9FLxRKfHIwbWPh0//Atxt1qwjy5FpILpiEUcvkeOEusijQdFbJJLZvbO0EjYU/
Uz4xpoYU8cPObY7JmDznKvc=
-----END PRIVATE KEY-----";

            #[test]
            fn with_key_modifies_signer() {
                let resolver = resolver();
                let default_key = resolver.signer.public_key();
                let modified_key = resolver.with_key(TEST_PRIVATE_KEY).signer.public_key();
                assert_ne!(default_key, modified_key);
            }
        }

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
