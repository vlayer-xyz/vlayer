use serde::Serialize;

use super::Timestamp;
#[cfg(feature = "signer")]
use crate::dns_over_https::types::Record as DNSRecord;
use crate::dns_over_https::types::RecordType;

#[derive(Serialize, PartialEq, Debug)]
pub struct Record {
    pub name: String,
    #[serde(rename = "type")]
    #[allow(clippy::struct_field_names)]
    pub record_type: RecordType,
    pub data: String,
    pub valid_until: Timestamp,
}

impl Record {
    #[cfg(feature = "signer")]
    pub(crate) fn new(record: &DNSRecord, valid_until: Timestamp) -> Self {
        Self {
            name: record.name.clone(),
            data: record.data.clone(),
            record_type: record.record_type.clone(),
            valid_until,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{dns_over_https::types::RecordType, verifiable_dns::signer::ToSignablePayload};

    #[test]
    fn serializes_to_canonical_json() {
        let record = DNSRecord {
            name: "selector._domainkey.vlayer.xyz".to_string(),
            record_type: RecordType::TXT,
            ttl: 300,
            data: "somedata".to_string(),
        };
        let record = Record::new(&record, 64);
        let expected =  br#"{"data":"somedata","name":"selector._domainkey.vlayer.xyz","type":16,"valid_until":64}"#;

        assert_eq!(record.to_payload(), expected);
    }
}
