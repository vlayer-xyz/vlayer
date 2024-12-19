use alloy_primitives::Bytes;
use alloy_sol_types::SolCall;
use hickory_resolver::{
    config::{ResolverConfig, ResolverOpts},
    Resolver,
};
use mailparse::MailHeaderMap;
use thiserror::Error;

use crate::cheatcodes::{preverifyEmailCall, UnverifiedEmail};

#[derive(Error, Debug)]
pub enum EnhanceEmailError {
    #[error("Failed to decode input: {0}, {1}")]
    DecodeInputError(Bytes, alloy_sol_types::Error),
    #[error("Failed to parse email: {0}")]
    ParseEmailError(String),
    #[error("Failed to resolve DNS: {0}")]
    ResolveDnsError(String),
    #[error("No DKIM-Signature header found")]
    NoDkimHeaderError,
    #[error("No DNS record found")]
    NoDnsRecordError,
}

pub fn preverify_email(input: &Bytes) -> Result<UnverifiedEmail, EnhanceEmailError> {
    let email = decode_input(input)?;
    let dkim_header = get_dkim_header(&email)?;
    let (selector, domain) = parse_dkim_header(&dkim_header)?;
    let dns = resolve_dns(selector, domain)?;

    Ok(UnverifiedEmail {
        email,
        dnsRecords: vec![dns],
    })
}

fn decode_input(input: &Bytes) -> Result<String, EnhanceEmailError> {
    let input = preverifyEmailCall::abi_decode(input, true)
        .map_err(|e| EnhanceEmailError::DecodeInputError(input.clone(), e))?;
    Ok(input.email)
}

fn get_dkim_header(email: &str) -> Result<String, EnhanceEmailError> {
    let parsed = mailparse::parse_mail(email.as_bytes())
        .map_err(|e| EnhanceEmailError::ParseEmailError(format!("{e:?}")))?;

    let dkim_header = parsed
        .get_headers()
        .get_first_value("DKIM-Signature")
        .ok_or(EnhanceEmailError::NoDkimHeaderError)?;

    Ok(dkim_header)
}

fn parse_dkim_header(header: &str) -> Result<(&str, &str), EnhanceEmailError> {
    let selector = regex::Regex::new(r#"s=([^;]+)"#)
        .expect("Invalid regex")
        .captures(header)
        .and_then(|c| c.get(1))
        .map(|c| c.as_str().trim())
        .ok_or(EnhanceEmailError::ParseEmailError("Missing selector in DKIM-Signature".into()))?;

    let domain = regex::Regex::new(r#"d=([^;]+)"#)
        .expect("Invalid regex")
        .captures(header)
        .and_then(|c| c.get(1))
        .map(|c| c.as_str().trim())
        .ok_or(EnhanceEmailError::ParseEmailError("Missing domain in DKIM-Signature".into()))?;

    Ok((selector, domain))
}

fn resolve_dns(selector: &str, domain: &str) -> Result<String, EnhanceEmailError> {
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

    let response = resolver
        .txt_lookup(&format!("{}._domainkey.{}", selector, domain))
        .map_err(|e| EnhanceEmailError::ResolveDnsError(format!("{e:?}")))?;

    let record = response
        .iter()
        .last()
        .ok_or(EnhanceEmailError::NoDnsRecordError)?
        .to_string();

    if record.starts_with("p=") {
        Ok(format!("v=DKIM1; k=rsa; {record}"))
    } else {
        Ok(record)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMAIL_FIXTURE: &str = "Delivered-To: hello@gmail.com
DKIM-Signature: v=1; a=rsa-sha256; c=relaxed/relaxed;
        d=vlayer.xyz; s=google; t=1733995599; x=1734600399; dara=google.com;
        h=to:subject:message-id:date:from:mime-version:from:to:cc:subject
         :date:message-id:reply-to;
        bh=mxM4CoKovnMiqziiIwlmU+GiIdJZluayVv0h5ezVKHk=;
        b=dIfpXPJr7YwemsO93MVu+rbuJ0GtQQ868zoKSWbecKp9Hj4V2tjw+v5qmBYzF1vZtn
         uvu7fqNEh/2as9WvFyQwySwwvxs9qkIvoideFMkgvbefI1Gx2YpwnpVKOcq0m4s8RF+K
         Q9pf1JbSjNVv5WfMSzNpef0tJO10RnSgK6UU7vNUnnbEd4qa/c6JjfjU6DLK5p9buNiJ
         VwqWDWoa+zv0NFtlmZwgLLdkU0C1JcMhn0hcO1mJlypq+kSWfDWjpnVcLBFsRbhlhX1T
         jEEFXgB7Ht7OBE8y7h1JhpoBuDEh2OdF6Crcaoj88jmvrIp28DZkxq6zac1vq/lGUk0o
         8k3A==
Hello
";

    const DKIM_HEADER: &str = r#"v=1; a=rsa-sha256; c=relaxed/relaxed; d=vlayer.xyz; s=google; t=1733995599; x=1734600399; dara=google.com; h=to:subject:message-id:date:from:mime-version:from:to:cc:subject :date:message-id:reply-to; bh=mxM4CoKovnMiqziiIwlmU+GiIdJZluayVv0h5ezVKHk=; b=dIfpXPJr7YwemsO93MVu+rbuJ0GtQQ868zoKSWbecKp9Hj4V2tjw+v5qmBYzF1vZtn uvu7fqNEh/2as9WvFyQwySwwvxs9qkIvoideFMkgvbefI1Gx2YpwnpVKOcq0m4s8RF+K Q9pf1JbSjNVv5WfMSzNpef0tJO10RnSgK6UU7vNUnnbEd4qa/c6JjfjU6DLK5p9buNiJ VwqWDWoa+zv0NFtlmZwgLLdkU0C1JcMhn0hcO1mJlypq+kSWfDWjpnVcLBFsRbhlhX1T jEEFXgB7Ht7OBE8y7h1JhpoBuDEh2OdF6Crcaoj88jmvrIp28DZkxq6zac1vq/lGUk0o 8k3A=="#;

    mod decode_input {
        use alloy_primitives::bytes;

        use super::*;

        #[test]
        fn decodes_input() {
            let input = bytes!("64646ba00000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000568656c6c6f000000000000000000000000000000000000000000000000000000");
            let result = decode_input(&input).unwrap();
            assert_eq!(result, "hello");
        }
    }

    mod get_dkim_header {
        use super::*;

        #[test]
        fn gets_dkim_header() {
            assert_eq!(get_dkim_header(EMAIL_FIXTURE).unwrap(), DKIM_HEADER);
        }
    }

    mod parse_dkim_header {
        use super::*;

        #[test]
        fn returns_selector_and_domain_pair() {
            let (selector, domain) = parse_dkim_header(DKIM_HEADER).unwrap();
            assert_eq!(selector, "google");
            assert_eq!(domain, "vlayer.xyz");
        }

        #[test]
        fn returns_error_for_missing_selector() {
            let header = r#""v=1; a=rsa-sha256; c=relaxed/relaxed; d=vlayer.xyz; t=1733995599; x=1734600399; dara=google.com; h=to:subject:message-id:date:from:mime-version:from:to:cc:subject :date:message-id:reply-to; bh=mxM4CoKovnMiqziiIwlmU+GiIdJZluayVv0h5ezVKHk=; b=dIfpXPJr7YwemsO93MVu+rbuJ0GtQQ868zoKSWbecKp9Hj4V2tjw+v5qmBYzF1vZtn uvu7fqNEh/2as9WvFyQwySwwvxs9qkIvoideFMkgvbefI1Gx2YpwnpVKOcq0m4s8RF+K Q9pf1JbSjNVv5WfMSzNpef0tJO10RnSgK6UU7vNUnnbEd4qa/c6JjfjU6DLK5p9buNiJ VwqWDWoa+zv0NFtlmZwgLLdkU0C1JcMhn0hcO1mJlypq+kSWfDWjpnVcLBFsRbhlhX1T jEEFXgB7Ht7OBE8y7h1JhpoBuDEh2OdF6Crcaoj88jmvrIp28DZkxq6zac1vq/lGUk0o 8k3A==""#;
            assert_eq!(
                parse_dkim_header(header).unwrap_err().to_string(),
                "Failed to parse email: Missing selector in DKIM-Signature"
            );
        }

        #[test]
        fn returns_error_for_missing_domain() {
            let header = r#""v=1; a=rsa-sha256; c=relaxed/relaxed; s=google; t=1733995599; x=1734600399; dara=google.com; h=to:subject:message-id:date:from:mime-version:from:to:cc:subject :date:message-id:reply-to; bh=mxM4CoKovnMiqziiIwlmU+GiIdJZluayVv0h5ezVKHk=; b=dIfpXPJr7YwemsO93MVu+rbuJ0GtQQ868zoKSWbecKp9Hj4V2tjw+v5qmBYzF1vZtn uvu7fqNEh/2as9WvFyQwySwwvxs9qkIvoideFMkgvbefI1Gx2YpwnpVKOcq0m4s8RF+K Q9pf1JbSjNVv5WfMSzNpef0tJO10RnSgK6UU7vNUnnbEd4qa/c6JjfjU6DLK5p9buNiJ VwqWDWoa+zv0NFtlmZwgLLdkU0C1JcMhn0hcO1mJlypq+kSWfDWjpnVcLBFsRbhlhX1T jEEFXgB7Ht7OBE8y7h1JhpoBuDEh2OdF6Crcaoj88jmvrIp28DZkxq6zac1vq/lGUk0o 8k3A==""#;
            assert_eq!(
                parse_dkim_header(header).unwrap_err().to_string(),
                "Failed to parse email: Missing domain in DKIM-Signature"
            );
        }
    }
}
