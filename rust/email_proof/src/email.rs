use alloy_sol_types::SolValue;
use chrono::{DateTime, FixedOffset};
use mailparse::{MailHeaderMap, MailParseError, ParsedMail};

mod sol;

#[derive(Debug)]
pub struct Email {
    pub from: Option<String>,
    pub to: Option<String>,
    pub subject: Option<String>,
    pub date: Option<DateTime<FixedOffset>>,
    pub body: String,
}

impl Email {
    pub fn abi_encode(self) -> Vec<u8> {
        sol::SolEmail::from(self).abi_encode()
    }
}

impl TryFrom<ParsedMail<'_>> for Email {
    type Error = MailParseError;

    fn try_from(mail: ParsedMail) -> Result<Self, Self::Error> {
        let headers = mail.get_headers();
        let get_header = |key: &str| headers.get_first_value(key).map(String::from);

        let from = get_header("From");
        let to = get_header("To");
        let subject = get_header("Subject");
        let date = get_header("Date")
            .map(|date| DateTime::parse_from_rfc2822(date.as_str()))
            .transpose()
            .map_err(|_| MailParseError::Generic("Invalid date"))?;

        let body = mail.get_body()?;

        Ok(Email {
            from,
            to,
            subject,
            date,
            body,
        })
    }
}
