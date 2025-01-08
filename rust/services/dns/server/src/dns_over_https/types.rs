use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::verifiable_dns::VerificationData;

#[derive(Serialize, Clone, Default, PartialEq, Debug)]
pub(crate) struct Query {
    pub name: String,
    #[serde(rename = "type")]
    record_type: RecordType,
}

#[derive(Serialize_repr, Deserialize_repr, Clone, Default, PartialEq, Debug)]
#[repr(u8)]
pub(crate) enum RecordType {
    #[default]
    #[allow(clippy::upper_case_acronyms)]
    TXT = 16,
}

#[derive(Serialize, Default, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Response {
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
    pub(crate) question: Query,
    pub(crate) answer: Vec<Record>,
    pub(crate) comment: String,
    pub(crate) verification_data: Option<VerificationData>,
}

#[derive(Serialize, Default, PartialEq, Debug)]
pub(crate) struct Record {
    pub name: String,
    #[serde(rename = "type")]
    #[allow(clippy::struct_field_names)]
    pub record_type: RecordType,
    #[serde(rename = "TTL")]
    pub ttl: u64,
    pub data: String,
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
    fn from(value: String) -> Self {
        Self {
            name: value,
            record_type: RecordType::TXT,
        }
    }
}

impl From<&str> for Query {
    fn from(value: &str) -> Self {
        Self {
            name: value.to_string(),
            record_type: RecordType::TXT,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

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
    }
}
