use serde::{Deserialize, Deserializer, Serialize};
use serde_repr::Serialize_repr;

use crate::{RecordVerifierError, VerificationData, verifier::verify_signature};

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct Query {
    pub name: String,
    #[serde(rename = "type")]
    record_type: RecordType,
}

#[derive(Serialize_repr, Clone, Default, PartialEq, Debug)]
#[repr(u8)]
pub enum RecordType {
    #[allow(clippy::upper_case_acronyms)]
    CNAME = 5,
    #[default]
    #[allow(clippy::upper_case_acronyms)]
    TXT = 16,
    OTHER = 0,
}

impl<'de> Deserialize<'de> for RecordType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        Ok(match value {
            5 => RecordType::CNAME,
            16 => RecordType::TXT,
            _ => RecordType::OTHER,
        })
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
    // of the following fields can be found here: https://developers.cloudflare.com/1.1.1.1/encryption/dns-over-https/make-api-requests/dns-json/#successful-response
    #[serde(rename = "TC")]
    pub(crate) truncated: bool,
    #[serde(rename = "RD")]
    pub(crate) recursive_desired: bool,
    #[serde(rename = "RA")]
    pub(crate) recursion_available: bool,
    #[serde(rename = "AD")]
    pub(crate) verified_with_dnssec: bool,
    #[serde(rename = "CD")]
    pub(crate) dnssec_validation_disabled: bool,

    pub(crate) status: u32,
    pub(crate) question: Vec<Query>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) answer: Option<Vec<Record>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) verification_data: Option<VerificationData>,
}

impl Default for Response {
    fn default() -> Self {
        Response {
            truncated: false,
            recursive_desired: true,
            recursion_available: true,
            verified_with_dnssec: false,
            dnssec_validation_disabled: false,
            status: 0,
            question: Default::default(),
            answer: Default::default(),
            comment: Default::default(),
            verification_data: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct Record {
    pub name: String,
    #[serde(rename = "type")]
    #[allow(clippy::struct_field_names)]
    pub record_type: RecordType,
    #[serde(rename = "TTL")]
    pub ttl: u64,
    pub data: String,
}

impl Record {
    pub fn verify(&self, verification_data: &VerificationData) -> Result<(), RecordVerifierError> {
        verify_signature(
            self,
            verification_data.valid_until,
            &verification_data.pub_key,
            &verification_data.signature,
        )
    }
}

impl Response {
    pub fn with_flags(tc: bool, rd: bool, ra: bool, ad: bool, cd: bool) -> Self {
        Self {
            truncated: tc,
            recursive_desired: rd,
            recursion_available: ra,
            verified_with_dnssec: ad,
            dnssec_validation_disabled: cd,
            ..Default::default()
        }
    }
}

impl From<String> for Query {
    fn from(mut value: String) -> Self {
        if !value.ends_with(".") {
            value.push('.')
        }

        Self {
            name: value,
            record_type: RecordType::TXT,
        }
    }
}

impl From<&str> for Query {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

#[cfg(test)]
mod tests {

    use serde_json::json;

    use super::*;

    mod response {

        use super::*;

        const RESPONSE: &str = r#"{"Status":0,"TC":false,"RD":true,"RA":true,"AD":false,"CD":false,"Question":[{"name":"vlayer.xyz","type":16}],"Answer":[{"name":"vlayer.xyz","type":16,"TTL":300,"data":"\"google-site-verification=RXMeofGg_JU-u2AlIN7iB5tiC6HoAz4Z6YfLZ2F5ziU\""},{"name":"vlayer.xyz","type":16,"TTL":300,"data":"\"v=spf1 mx include:_spf.google.com include:servers.mcsv.net -all\""}]}"#;

        #[test]
        fn decodes_real_response() {
            let response: Response = serde_json::from_str(RESPONSE).unwrap();

            assert_eq!(
                response,
                Response {
                    status: 0,
                    truncated: false,
                    recursive_desired: true,
                    recursion_available: true,
                    dnssec_validation_disabled: false,
                    verified_with_dnssec: false,
                    question: vec![ Query{name: "vlayer.xyz".into(), record_type: RecordType::TXT} ],
                    answer: Some(vec![ Record {
                        name: "vlayer.xyz".into(),
                        record_type: RecordType::TXT,
                        ttl: 300,
                        data: "\"google-site-verification=RXMeofGg_JU-u2AlIN7iB5tiC6HoAz4Z6YfLZ2F5ziU\""
                            .into()
                        },
                        Record {
                            name: "vlayer.xyz".into(),
                            record_type: RecordType::TXT,
                            ttl: 300,
                            data: "\"v=spf1 mx include:_spf.google.com include:servers.mcsv.net -all\""
                                .into()
                        },
                    ]),
                    comment: None,
                    verification_data: None
                }
            )
        }
    }

    #[test]
    fn serialized_json_skips_empty_fields() {
        let response = Response {
            status: 0,
            truncated: false,
            recursive_desired: true,
            recursion_available: true,
            dnssec_validation_disabled: false,
            verified_with_dnssec: false,
            question: vec![],
            answer: None,
            comment: None,
            verification_data: None,
        };

        assert_eq!(
            serde_json::to_value(&response).unwrap(),
            json!({
                "Status": 0,
                "TC": false,
                "RD": true,
                "RA": true,
                "AD": false,
                "CD": false,
                "Question": [],
            })
        );
    }

    mod query {

        use super::*;

        mod from_string {
            use super::*;
            #[test]
            fn query_ends_with_dot() {
                let query: Query = "subdomain.vlayer.xyz".into();

                assert!(query.name.ends_with("."));
                assert_eq!(query, "subdomain.vlayer.xyz.".into())
            }
        }

        #[test]
        fn encodes_into_url_query() {
            let query: Query = "subdomain.vlayer.xyz".into();

            assert_eq!(
                serde_urlencoded::to_string(&query).unwrap(),
                "name=subdomain.vlayer.xyz.&type=16"
            );
        }

        #[test]
        fn encodes_unicode_into_url_query() {
            let query: Query = "👍ąćźż".into();

            assert_eq!(
                serde_urlencoded::to_string(&query).unwrap(),
                "name=%F0%9F%91%8D%C4%85%C4%87%C5%BA%C5%BC.&type=16"
            );
        }
    }

    mod record_type {
        use serde_json::json;

        use super::*;

        #[test]
        fn encodes_into_int() {
            assert_eq!(serde_json::to_value(&RecordType::TXT).unwrap(), json!(16));
            assert_eq!(&serde_json::to_string(&RecordType::TXT).unwrap(), "16");
        }

        #[test]
        fn decodes_from_int() {
            let decoded: RecordType = serde_json::from_str("16").unwrap();
            assert_eq!(decoded, RecordType::TXT);
        }

        #[test]
        fn decodes_from_unknown_int() {
            let dnssec_type = "46";
            let decoded: RecordType = serde_json::from_str(dnssec_type).unwrap();
            assert_eq!(decoded, RecordType::OTHER);
        }
    }
}
