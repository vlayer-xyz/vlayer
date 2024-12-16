use addr::{email::Host, parse_email_address};

pub struct EmailAddress {}

impl EmailAddress {
    pub fn is_valid(email_address: &str) -> bool {
        parse_email_address(email_address)
            .ok()
            .and_then(|addr| match addr.host() {
                Host::Domain(domain) => domain
                    .as_str()
                    .rsplit_once('.')
                    .map(|(_, tld)| tld.len() >= 2),
                _ => None,
            })
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const VALID_EMAILS: [&str; 13] = [
        r#"email@example.com"#,
        r#"firstname.lastname@example.com"#,
        r#"email@subdomain.example.com"#,
        r#"firstname+lastname@example.com"#,
        r#""email"@example.com"#,
        r#"1234567890@example.com"#,
        r#"email@example-one.com"#,
        r#"_______@example.com"#,
        r#"email@example.name"#,
        r#"email@example.museum"#,
        r#"email@example.co.jp"#,
        r#"firstname-lastname@example.com"#, //
        r#"あいうえお@example.com"#,
        // r#"much."more\ unusual"@example.com"#,
        // r#"very.unusual."@".unusual.com@example.com"#,
        // r#"very."(),:;<>[]".VERY."very@\\ "very.unusual@strange.example.com"#,
    ];

    #[test]
    fn valid_emails() {
        for address in VALID_EMAILS {
            assert!(
                EmailAddress::is_valid(address),
                "{}",
                format!("Expected {address} to be valid")
            );
        }
    }

    const INVALID_EMAILS: [&str; 19] = [
        r#"plainaddress"#,
        r#"#@%^%#$@#$@#.com"#,
        r#"@example.com"#,
        r#"Joe Smith <email@example.com>"#,
        r#"<email@example.com>"#,
        r#"email.example.com"#,
        r#"email@example@example.com"#,
        r#"quotes@inside"domain".com"#,
        r#".email@example.com"#,
        r#"email.@example.com"#,
        r#"email..email@example.com"#,
        r#"email@example.com (Joe Smith)"#,
        r#"email@example"#,
        r#"email@-example.com"#,
        r#"email@[123.123.123.123]"#,
        r#"email@123.123.123.123"#,
        r#"email@example..com"#,
        r#"Abc..123@example.com"#,
        r#"unclosed"quote@example@example.com"#,
    ];

    #[test]
    fn invalid_emails() {
        for address in INVALID_EMAILS {
            assert!(
                !EmailAddress::is_valid(address),
                "{}",
                format!("Expected {address} to be invalid")
            );
        }
    }
}
