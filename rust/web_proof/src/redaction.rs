use std::{fmt, iter::zip, str::from_utf8};

use strum::Display;

use crate::{errors::ParsingError, utils::bytes::all_match};

pub(crate) const REDACTED_BYTE_CODE: u8 = 0;

// Both '*' and 'X' are valid header characters. Replacing redacted '\0' bytes with
// two different characters ensures the request is parsable and allows analysis
// of redacted content via diffs.
pub(crate) const REDACTION_REPLACEMENT_CHAR_PRIMARY: char = '*';
pub(crate) const REDACTION_REPLACEMENT_CHAR_SECONDARY: char = 'X';

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct RedactedTranscriptNameValue {
    pub(crate) name: String,
    pub(crate) value: Vec<u8>,
}

impl fmt::Display for RedactedTranscriptNameValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            self.name,
            from_utf8(&self.value).unwrap_or(format!("{:?}", self.value).as_str())
        )
    }
}

impl From<(String, String)> for RedactedTranscriptNameValue {
    fn from(pair: (String, String)) -> Self {
        Self {
            name: pair.0,
            value: pair.1.as_bytes().to_vec(),
        }
    }
}

impl<'a> From<&httparse::Header<'a>> for RedactedTranscriptNameValue {
    fn from(header: &httparse::Header<'a>) -> Self {
        Self {
            name: header.name.to_string(),
            value: header.value.to_vec(),
        }
    }
}

#[derive(Debug, Display, PartialEq, Eq)]
pub enum RedactionElementType {
    #[strum(to_string = "request header")]
    RequestHeader,
    #[strum(to_string = "request url parameter")]
    RequestUrlParam,
    #[strum(to_string = "response header")]
    ResponseHeader,
    #[strum(to_string = "response body")]
    ResponseBody,
}

pub(crate) fn validate_name_value_redaction(
    name_values_with_replacement_primary: &[RedactedTranscriptNameValue],
    name_values_with_replacement_secondary: &[RedactedTranscriptNameValue],
    redaction_element_type: RedactionElementType,
) -> Result<(), ParsingError> {
    let zipped_pairs = zip(
        name_values_with_replacement_primary.iter(),
        name_values_with_replacement_secondary.iter(),
    );

    let redacted_name = zipped_pairs.clone().find(|(l, r)| l.name != r.name);

    if let Some(pair) = redacted_name {
        return Err(ParsingError::RedactedName(redaction_element_type, pair.0.to_string()));
    }

    let partially_redacted_value = zipped_pairs.clone().find(|(l, r)| {
        if l.value == r.value {
            return false;
        }
        if all_match(&l.value, REDACTION_REPLACEMENT_CHAR_PRIMARY as u8)
            && all_match(&r.value, REDACTION_REPLACEMENT_CHAR_SECONDARY as u8)
        {
            return false;
        }
        true
    });

    if let Some(pair) = partially_redacted_value {
        return Err(ParsingError::PartiallyRedactedValue(
            redaction_element_type,
            pair.0.to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod test_validate_name_value_redaction {
    use super::*;

    impl From<(&str, &str)> for RedactedTranscriptNameValue {
        fn from(pair: (&str, &str)) -> Self {
            (pair.0.to_string(), pair.1.to_string()).into()
        }
    }

    #[test]
    fn success_no_redaction() {
        let name_values_primary = vec![("name1", "value1").into(), ("name2", "value2").into()];
        let name_values_secondary = vec![("name1", "value1").into(), ("name2", "value2").into()];

        assert!(
            validate_name_value_redaction(
                &name_values_primary,
                &name_values_secondary,
                RedactionElementType::RequestHeader
            )
            .is_ok()
        );
    }

    #[test]
    fn success_value_full_redaction() {
        let name_values_primary = vec![("name1", "******").into(), ("name2", "value2").into()];
        let name_values_secondary = vec![("name1", "XXXXX").into(), ("name2", "value2").into()];

        assert!(
            validate_name_value_redaction(
                &name_values_primary,
                &name_values_secondary,
                RedactionElementType::RequestHeader
            )
            .is_ok()
        );
    }

    #[test]
    fn fail_partial_name_redaction() {
        let primary = vec![("name*", "value1").into(), ("name2", "value2").into()];
        let secondary = vec![("nameX", "value1").into(), ("name2", "value2").into()];

        let err = validate_name_value_redaction(
            &primary,
            &secondary,
            RedactionElementType::RequestHeader,
        )
        .unwrap_err();
        assert!(matches!(
            err,
            ParsingError::RedactedName(RedactionElementType::RequestHeader, err_string) if err_string == "name*: value1"
        ));
    }

    #[test]
    fn fail_full_name_redaction() {
        let primary = vec![("*****", "value1").into(), ("name2", "value2").into()];
        let secondary = vec![("XXXXX", "value1").into(), ("name2", "value2").into()];

        let err = validate_name_value_redaction(
            &primary,
            &secondary,
            RedactionElementType::RequestHeader,
        )
        .unwrap_err();
        assert!(matches!(
            err,
            ParsingError::RedactedName(RedactionElementType::RequestHeader, err_string) if err_string == "*****: value1"
        ));
    }

    #[test]
    fn fail_partial_value_redaction() {
        let primary = vec![("name1", "value*").into(), ("name2", "value2").into()];
        let secondary = vec![("name1", "valueX").into(), ("name2", "value2").into()];

        let err = validate_name_value_redaction(
            &primary,
            &secondary,
            RedactionElementType::RequestHeader,
        )
        .unwrap_err();
        assert!(matches!(
            err,
            ParsingError::PartiallyRedactedValue(RedactionElementType::RequestHeader, err_string) if err_string == "name1: value*"
        ));
    }

    #[test]
    fn fail_partial_value_redaction_using_primary_redaction_character() {
        let primary = vec![("name1", "**").into()];
        let secondary = vec![("name1", "*X").into()];

        let err = validate_name_value_redaction(
            &primary,
            &secondary,
            RedactionElementType::RequestHeader,
        )
        .unwrap_err();
        assert!(matches!(
            err,
            ParsingError::PartiallyRedactedValue(RedactionElementType::RequestHeader, err_string) if err_string == "name1: **"
        ));
    }
}
