use serde::Serialize;

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Query {
    name: String,
}

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Response {
    question: Query,
    answer: Record,
}

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
