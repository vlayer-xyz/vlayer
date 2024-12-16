use cfdkim::{verify_email_with_key, DKIMError, DkimPublicKey};
use mailparse::{MailHeaderMap, ParsedMail};
use slog::{o, Discard, Logger};

pub fn verify_email<'a>(
    email: ParsedMail<'a>,
    from_domain: &str,
    dkim_public_key: DkimPublicKey,
) -> Result<ParsedMail<'a>, DKIMError> {
    let dkim_headers = email.headers.get_all_headers("DKIM-Signature");
    if dkim_headers.is_empty() {
        return Err(DKIMError::SignatureSyntaxError("No DKIM-Signature header".into()));
    }

    let logger = Logger::root(Discard, o!());
    let result = verify_email_with_key(&logger, from_domain, &email, dkim_public_key)?;

    match result {
        result if result.with_detail().starts_with("pass") => Ok(email),
        result if result.error().is_some() => Err(result.error().unwrap()),
        _ => Err(DKIMError::SignatureDidNotVerify),
    }
}
