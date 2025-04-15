use alloy_primitives::Bytes;
use alloy_sol_types::SolCall;
use hickory_resolver::{
    Resolver,
    config::{ResolverConfig, ResolverOpts},
};
use mailparse::MailHeaderMap;
use thiserror::Error;
use verifiable_dns::{
    DNSRecord, RecordType,
    verifiable_dns::{sign_record::sign_record, signer::Signer},
};

use crate::cheatcodes::{DnsRecord, UnverifiedEmail, preverifyEmailCall};

#[derive(Error, Debug)]
pub enum EnhanceEmailError {
    #[error("Failed to decode input: {0}, {1}")]
    DecodeInput(Bytes, alloy_sol_types::Error),
    #[error("Failed to parse email: {0}")]
    ParseEmail(String),
    #[error("Failed to resolve DNS: {0}")]
    ResolveDns(String),
    #[error("No DKIM-Signature header found")]
    NoDkimHeader,
    #[error("No DNS record found")]
    NoDnsRecord,
}

pub fn preverify_email(input: &Bytes) -> Result<UnverifiedEmail, EnhanceEmailError> {
    let email = decode_input(input)?;
    let dkim_header = get_dkim_header(&email)?;
    let (selector, domain) = parse_dkim_header(&dkim_header)?;
    let dns_record = resolve_dkim_dns(selector, domain)?;
    let verification_data = sign_record(
        &Signer::default(),
        &DNSRecord {
            data: dns_record.data.clone(),
            name: dns_record.name.clone(),
            record_type: RecordType::TXT,
            ttl: dns_record.ttl,
        },
        u64::MAX,
    );

    Ok(UnverifiedEmail {
        email,
        dnsRecord: dns_record,
        verificationData: verification_data.into(),
    })
}

fn decode_input(input: &Bytes) -> Result<String, EnhanceEmailError> {
    let input = preverifyEmailCall::abi_decode(input, true)
        .map_err(|e| EnhanceEmailError::DecodeInput(input.clone(), e))?;
    Ok(input.email)
}

fn get_dkim_header(email: &str) -> Result<String, EnhanceEmailError> {
    let parsed = mailparse::parse_mail(email.as_bytes())
        .map_err(|e| EnhanceEmailError::ParseEmail(format!("{e:?}")))?;

    let dkim_header = parsed
        .get_headers()
        .get_first_value("DKIM-Signature")
        .ok_or(EnhanceEmailError::NoDkimHeader)?;

    Ok(dkim_header)
}

fn parse_dkim_header(header: &str) -> Result<(&str, &str), EnhanceEmailError> {
    let selector = extract_from_header('s', header)
        .ok_or(EnhanceEmailError::ParseEmail("Missing selector in DKIM-Signature".into()))?;

    let domain = extract_from_header('d', header)
        .ok_or(EnhanceEmailError::ParseEmail("Missing domain in DKIM-Signature".into()))?;

    Ok((selector, domain))
}

fn extract_from_header(tag: char, header: &str) -> Option<&str> {
    let regex = regex::Regex::new(&format!("{tag}=([^;]+)")).expect("Invalid regex");
    regex
        .captures(header)
        .and_then(|c| c.get(1))
        .map(|c| c.as_str().trim())
}

fn resolve_dkim_dns(selector: &str, domain: &str) -> Result<DnsRecord, EnhanceEmailError> {
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let name = format!("{selector}._domainkey.{domain}");

    let response = resolver
        .txt_lookup(&name)
        .map_err(|e| EnhanceEmailError::ResolveDns(format!("{e:?}")))?;

    let record = response
        .iter()
        .last()
        .ok_or(EnhanceEmailError::NoDnsRecord)?
        .to_string();

    let record = if record.starts_with("p=") {
        format!("v=DKIM1; k=rsa; {record}")
    } else {
        record
    };

    Ok(DnsRecord {
        name,
        recordType: 16,
        data: record,
        ttl: u64::MAX,
    })
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
            let input = bytes!(
                "64646ba00000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000568656c6c6f000000000000000000000000000000000000000000000000000000"
            );
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
