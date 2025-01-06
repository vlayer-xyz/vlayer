use serde::Serialize;

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Record {
    pub name: String,
    #[serde(rename = "Type")]
    pub record_type: u8,
    #[serde(rename = "TTL")]
    pub ttl: u32,
    pub data: String,
}

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Response {
    answer: Record,
}
