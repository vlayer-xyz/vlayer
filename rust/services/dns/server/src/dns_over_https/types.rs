use serde::Serialize;

#[derive(Serialize, Clone, Default, PartialEq, Debug)]
pub(crate) struct Query {
    name: String,
    #[serde(rename = "type")]
    record_type: RecordType,
}

#[derive(Serialize, Clone, Default, PartialEq, Debug)]
pub(crate) enum RecordType {
    #[default]
    TXT,
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

#[derive(Serialize, Default, PartialEq, Debug)]
pub(crate) struct Record {
    pub name: String,
    #[serde(rename = "type")]
    pub record_type: u8,
    #[serde(rename = "TTL")]
    pub ttl: u32,
    pub data: String,
}

impl From<String> for Query {
    fn from(value: String) -> Self {
        Self { name: value }
    }
}

impl From<&str> for Query {
    fn from(value: &str) -> Self {
        Self {
            name: value.to_string(),
        }
    }
}
