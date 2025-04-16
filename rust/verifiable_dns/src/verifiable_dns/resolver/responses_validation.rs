use std::{collections::HashSet, hash::Hash};

use crate::{
    dns_over_https::{Response, types::Record},
    verifiable_dns::resolver::ResolverError,
};

pub(super) fn validate_response<PError>(response: &Response) -> Result<(), ResolverError<PError>> {
    let error_message =
        |msg: &str| Err(ResolverError::InvalidResponse(response.clone(), msg.into()));

    if response.status != 0 {
        return error_message("Non-zero status");
    }
    if !response.recursive_desired {
        return error_message("Recursion not desired");
    }
    if !response.recursion_available {
        return error_message("Recursion not available");
    }
    if response.truncated {
        return error_message("Response is truncated");
    }
    if response.answer.as_ref().is_none_or(Vec::is_empty) {
        return error_message("No answers");
    }

    Ok(())
}

pub(super) fn responses_match(l: &Response, r: &Response) -> bool {
    (l.status == r.status) && (l.question == r.question) && compare_answers(&l.answer, &r.answer)
}

#[allow(clippy::unwrap_used)]
fn compare_answers(l: &Option<Vec<Record>>, r: &Option<Vec<Record>>) -> bool {
    if l.is_none() || r.is_none() {
        return l.is_none() && r.is_none();
    }

    let l = l.as_ref().unwrap().iter().filter(|r| is_txt_or_cname(r));
    let r = r.as_ref().unwrap().iter().filter(|r| is_txt_or_cname(r));

    compare_unordered(l.map(canonized), r.map(canonized))
}

fn compare_unordered<T: Eq + Hash>(
    l: impl IntoIterator<Item = T>,
    r: impl IntoIterator<Item = T>,
) -> bool {
    HashSet::<T>::from_iter(l) == HashSet::<T>::from_iter(r)
}

fn is_txt_or_cname(r: &Record) -> bool {
    r.record_type != crate::dns_over_https::types::RecordType::OTHER
}

fn canonized(r: &Record) -> String {
    canonize_data(&r.data)
}

fn canonize_data(data: &str) -> String {
    data.replace(r#"" ""#, "").replace("\"", "")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dns_over_https::types::{Record, RecordType};

    fn mock_response() -> Response {
        let record = Record {
            name: "hello.vlayer.xyz".into(),
            record_type: RecordType::TXT,
            ttl: 300,
            data: "some data".into(),
        };

        Response {
            answer: Some(vec![record]),
            ..Default::default()
        }
    }

    fn mock_response_with_type(record_type: RecordType) -> Response {
        let record = Record {
            name: "hello.vlayer.xyz".into(),
            record_type,
            ttl: 300,
            data: "some data".into(),
        };

        Response {
            answer: Some(vec![record]),
            ..Default::default()
        }
    }
    mod validate_response {
        use super::*;

        #[test]
        fn passes_for_zeroed_status() {
            let response = mock_response();

            assert!(validate_response::<()>(&response).is_ok());
        }

        #[test]
        fn fails_for_non_zeroed_status() {
            let response = Response {
                status: 1,
                ..mock_response()
            };
            assert!(validate_response::<()>(&response).is_err());
        }

        #[test]
        fn result_should_not_be_truncated() {
            let response = Response {
                truncated: true,
                ..mock_response()
            };
            assert!(validate_response::<()>(&response).is_err());
        }

        #[test]
        fn must_have_at_least_one_answer() {
            let response = Response {
                answer: Some(Default::default()),
                ..mock_response()
            };
            assert!(validate_response::<()>(&response).is_err());
        }

        #[test]
        fn recursion_should_be_available() {
            let response = Response {
                recursion_available: false,
                ..mock_response()
            };
            assert!(validate_response::<()>(&response).is_err());

            let response = Response {
                recursive_desired: false,
                ..mock_response()
            };
            assert!(validate_response::<()>(&response).is_err());
        }
    }

    mod responses_match {
        use super::*;
        use crate::{PublicKey, Signature, VerificationData, dns_over_https::Query};

        #[test]
        fn passes_for_equal_responses() {
            let response = Response::default();
            assert!(responses_match(&response, &response));
        }

        #[test]
        fn status_must_match() {
            let l = Response::default();
            let r = Response {
                status: 1,
                ..l.clone()
            };
            assert!(!responses_match(&l, &r));
        }

        #[test]
        fn query_must_match() {
            let query_l: Query = "hello.vlayer.xyz".into();
            let query_r: Query = "hello1.vlayer.xyz".into();

            let l = Response {
                question: vec![query_l],
                ..Response::default()
            };

            let r = Response {
                question: vec![query_r],
                ..l.clone()
            };
            assert!(!responses_match(&l, &r));
        }

        #[test]
        fn comment_field_is_irrelevant() {
            let a = Response::default();

            let b = Response {
                comment: Some("Some comment".to_string()),
                ..Response::default()
            };

            let c = Response {
                comment: Some("Some other comment".to_string()),
                ..Response::default()
            };

            assert!(responses_match(&a, &b));
            assert!(responses_match(&b, &c));
        }

        #[test]
        fn verification_data_is_irrelevant() {
            let l = Response::default();
            let r = Response {
                verification_data: Some(VerificationData {
                    valid_until: 1,
                    signature: Signature(Default::default()),
                    pub_key: PublicKey(Default::default()),
                }),
                ..Default::default()
            };

            assert!(responses_match(&l, &r));
        }

        #[test]
        fn flags_are_irrelevant() {
            let l = Response::with_flags(true, true, true, true, true);
            let r = Response::with_flags(false, false, false, false, false);

            assert!(responses_match(&l, &r));
        }

        mod answer {
            use super::*;

            #[test]
            fn passes_for_same_answers() {
                let l = mock_response();
                assert!(responses_match(&l, &l));
            }

            #[test]
            fn fails_for_different_values() {
                let l = mock_response();
                let mut r = mock_response();
                let records = r.answer.take().unwrap();
                assert!(!responses_match(&l, &r));

                r.answer = Some(vec![Record {
                    data: "".to_string(),
                    ..records.first().unwrap().clone()
                }]);
                assert!(!responses_match(&l, &r));
            }

            #[test]
            fn passes_for_repeated_arguments() {
                let l = mock_response();
                let mut r = mock_response();
                let mut records = r.answer.take().unwrap();
                records.push(records.first().unwrap().clone());
                r.answer = Some(records);

                assert!(responses_match(&l, &r));
            }

            #[test]
            fn records_may_be_unsorted() {
                let mut records = vec![
                    Record {
                        name: "vlayer.xzy".into(),
                        record_type: RecordType::TXT,
                        ttl: 300,
                        data: "google verification".into(),
                    },
                    Record {
                        name: "vlayer.xzy".into(),
                        record_type: RecordType::TXT,
                        ttl: 300,
                        data: "cloudflare verification".into(),
                    },
                ];

                let l = Response {
                    answer: Some(records.clone()),
                    ..mock_response()
                };

                records.reverse();
                let r = Response {
                    answer: Some(records),
                    ..mock_response()
                };

                assert!(responses_match(&l, &r));
            }

            #[test]
            fn ttls_are_irrelevant() {
                let l = mock_response();
                let mut r = mock_response();
                let records = r.answer.take().unwrap();
                r.answer = Some(vec![Record {
                    ttl: 111,
                    ..records.first().unwrap().clone()
                }]);
                assert!(responses_match(&l, &r));
            }

            #[test]
            fn fails_for_mismatching_cname_records() {
                let l = mock_response_with_type(RecordType::CNAME);
                let mut r = mock_response_with_type(RecordType::CNAME);
                let records = r.answer.take().unwrap();
                assert!(!responses_match(&l, &r));

                r.answer = Some(vec![Record {
                    data: "".to_string(),
                    ..records.first().unwrap().clone()
                }]);
                assert!(!responses_match(&l, &r));
            }

            mod canonization {
                use tests::canonize_data;

                use super::*;
                #[test]
                fn data_is_canonized_before_comparison() {
                    let l = Record {
                        name: "vlayer.xzy".into(),
                        record_type: RecordType::TXT,
                        ttl: 300,
                        data: r#""google verification""#.into(),
                    };
                    let r = Record {
                        name: "vlayer.xzy".into(),
                        record_type: RecordType::TXT,
                        ttl: 300,
                        data: "google verification".into(),
                    };

                    assert!(compare_answers(&Some(vec![l]), &Some(vec![r])));
                }

                #[test]
                fn can_canonize_data() {
                    assert_eq!(&canonize_data(r#""hello""world""#), "helloworld");
                    assert_eq!(&canonize_data(r#""hello" "world""#), "helloworld");
                }
            }
        }
    }
}
