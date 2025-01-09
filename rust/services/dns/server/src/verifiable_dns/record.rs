use serde::Serialize;

use super::Timestamp;
use crate::dns_over_https::types::{Record as DNSRecord, RecordType};

#[derive(Serialize)]
pub(super) struct Record<'a> {
    name: &'a String,
    #[serde(rename = "type")]
    #[allow(clippy::struct_field_names)]
    record_type: &'a RecordType,
    data: &'a String,
    valid_until: Timestamp,
}

impl<'a> Record<'a> {
    pub const fn new(record: &'a DNSRecord, valid_until: Timestamp) -> Self {
        Self {
            name: &record.name,
            data: &record.data,
            record_type: &record.record_type,
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
