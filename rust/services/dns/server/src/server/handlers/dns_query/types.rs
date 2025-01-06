use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(super) struct Params {
    #[serde(rename(deserialize = "name"))]
    pub name: String,
    #[serde(rename(deserialize = "type"))]
    _query_type: DNSQueryType,
}

#[derive(Deserialize, Debug)]
pub(super) enum DNSQueryType {
    #[allow(clippy::upper_case_acronyms)]
    #[serde(alias = "txt")]
    TXT,
}
