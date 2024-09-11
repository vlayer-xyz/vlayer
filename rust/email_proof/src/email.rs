use alloy_sol_types::SolValue;
use chrono::{DateTime, FixedOffset};
use mailparse::headers::Headers;
use mailparse::{MailHeaderMap, MailParseError, ParsedMail};

#[derive(Debug)]
pub struct Email {
    pub from: Option<String>,
    pub to: Option<String>,
    pub subject: Option<String>,
    pub date: Option<DateTime<FixedOffset>>,
    pub body: String,
}

mod private {
    use alloy_sol_types::sol;

    sol!(#![sol(all_derives)] "../../contracts/src/EmailProof.sol");
}

impl Email {
    pub fn abi_encode(self) -> Vec<u8> {
        let email_sol: private::Email = self.into();
        email_sol.abi_encode()
    }
}

impl Into<private::Email> for Email {
    fn into(self) -> private::Email {
        private::Email {
            from: self.from.unwrap_or_default(),
            to: self.to.unwrap_or_default(),
            subject: self.subject.unwrap_or_default(),
            date: self.date.map(|d| d.timestamp()).unwrap_or_default() as u64,
            body: self.body,
        }
    }
}

impl TryFrom<ParsedMail<'_>> for Email {
    type Error = MailParseError;

    fn try_from(mail: ParsedMail) -> Result<Self, Self::Error> {
        let get_header = header_getter(mail.get_headers());
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

fn header_getter(headers: Headers) -> impl Fn(&str) -> Option<String> + '_ {
    move |key: &str| headers.get_first_value(key).map(String::from)
}
