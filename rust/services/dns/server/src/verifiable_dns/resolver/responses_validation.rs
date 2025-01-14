use crate::dns_over_https::{types::Record, Response};

pub(super) fn validate_response(response: &Response) -> bool {
    response.status == 0
        && response.recursive_desired
        && response.recursion_available
        && !response.truncated
        && response
            .answer
            .as_ref()
            .is_some_and(|answers| !answers.is_empty())
}

pub(super) fn responses_match(l: &Response, r: &Response) -> bool {
    (l.status == r.status) && (l.question == r.question) && compare_answers(&l.answer, &r.answer)
}

fn compare_answers(l: &Option<Vec<Record>>, r: &Option<Vec<Record>>) -> bool {
    if l.is_none() || r.is_none() {
        return l.is_none() && r.is_none();
    }

    let mut l: Vec<_> = l.as_ref().unwrap().iter().collect();
    let mut r: Vec<_> = r.as_ref().unwrap().iter().collect();

    if l.len() != r.len() {
        return false;
    }

    l.sort_by(|lr, rr| canonize_data(&lr.data).cmp(&canonize_data(&rr.data)));
    r.sort_by(|lr, rr| canonize_data(&lr.data).cmp(&canonize_data(&rr.data)));

    l.iter()
        .zip(r)
        .all(|(record_l, record_r)| compare_records(record_l, record_r))
}

fn compare_records(l: &Record, r: &Record) -> bool {
    canonize_data(&l.data) == canonize_data(&r.data)
}

fn canonize_data(data: &String) -> String {
    data.clone().replace(r#"" ""#, "").replace("\"", "")
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
    mod validate_response {
        use super::*;

        #[test]
        fn passes_for_zeroed_status() {
            let response = mock_response();

            assert!(validate_response(&response));
        }

        #[test]
        fn fails_for_non_zeroed_status() {
            let response = Response {
                status: 1,
                ..mock_response()
            };
            assert!(!validate_response(&response));
        }

        #[test]
        fn result_should_not_be_truncated() {
            let response = Response {
                truncated: true,
                ..mock_response()
            };
            assert!(!validate_response(&response));
        }

        #[test]
        fn must_have_at_least_one_answer() {
            let response = Response {
                answer: Some(Default::default()),
                ..mock_response()
            };
            assert!(!validate_response(&response));
        }

        #[test]
        fn recursion_should_be_available() {
            let response = Response {
                recursion_available: false,
                ..mock_response()
            };
            assert!(!validate_response(&response));

            let response = Response {
                recursive_desired: false,
                ..mock_response()
            };
            assert!(!validate_response(&response));
        }
    }

    mod responses_match {

        use super::*;
        use crate::{
            dns_over_https::Query,
            verifiable_dns::{
                signer::{PublicKey, Signature},
                VerificationData,
            },
        };

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
                    ..records.get(0).unwrap().clone()
                }]);
                assert!(!responses_match(&l, &r));
            }

            #[test]
            fn fails_for_different_number_of_records() {
                let l = mock_response();
                let mut r = mock_response();
                let mut records = r.answer.take().unwrap();
                records.push(records.get(0).unwrap().clone());
                r.answer = Some(records);

                assert!(!responses_match(&l, &r));
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
                    ..records.get(0).unwrap().clone()
                }]);
                assert!(responses_match(&l, &r));
            }

            mod records {
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

                    assert!(compare_records(&l, &r));
                }

                #[test]
                fn can_canonize_data() {
                    assert_eq!(&canonize_data(&r#""hello""world""#.to_string()), "helloworld");
                    assert_eq!(&canonize_data(&r#""hello" "world""#.to_string()), "helloworld");
                }
            }
        }
    }
}
