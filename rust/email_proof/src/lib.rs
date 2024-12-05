mod dkim;
mod dns;
mod email;
mod errors;
mod from_header;
#[cfg(test)]
mod test_utils;

pub use email::sol::UnverifiedEmail;

pub use crate::{email::Email, errors::Error};

pub fn parse_and_verify(calldata: &[u8]) -> Result<Email, Error> {
    let (raw_email, dns_records) =
        UnverifiedEmail::parse_calldata(calldata).map_err(Error::Calldata)?;

    let email = mailparse::parse_mail(&raw_email).map_err(Error::EmailParse)?;
    let dns_record = dns_records
        .first()
        .ok_or(Error::InvalidDkimRecord("No DNS records provided".into()))?;

    let from_domain = from_header::extract_from_domain(&email)?;

    dkim::verify_email(email, &from_domain, dns::parse_dns_record(dns_record)?)
        .map_err(Error::DkimVerification)?
        .try_into()
        .map_err(Error::EmailParse)
}
