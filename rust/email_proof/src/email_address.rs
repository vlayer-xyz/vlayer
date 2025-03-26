pub struct EmailAddress {}

const MAX_LOCAL_LENGTH: usize = 64;
const MAX_DOMAIN_LENGTH: usize = 255;

impl EmailAddress {
    pub fn is_valid(email_address: &str) -> bool {
        let Some((username, domainname)) = Self::split_email(email_address) else {
            return false;
        };

        if username.len() > MAX_LOCAL_LENGTH || domainname.len() > MAX_DOMAIN_LENGTH {
            return false;
        }

        let Some(unquoted_username) = Self::remove_parts_inside_quotes(username) else {
            return false;
        };

        if Self::contains_invalid_characters(&unquoted_username)
            || Self::contains_invalid_characters(domainname)
        {
            return false;
        }

        matches!(domainname.rsplit_once('.'), Some((_, domain)) if domain.len() >= 2)
    }

    fn split_email(email_address: &str) -> Option<(&str, &str)> {
        match email_address.rsplit_once('@') {
            Some((username, domainname)) if !username.is_empty() && !domainname.is_empty() => {
                Some((username, domainname))
            }
            _ => None,
        }
    }

    fn remove_parts_inside_quotes(email_part: &str) -> Option<String> {
        let mut inside_quotes = false;
        let result: String = email_part
            .chars()
            .filter_map(|c| Self::handle_quotes(c, &mut inside_quotes))
            .collect();

        if inside_quotes { None } else { Some(result) }
    }

    fn handle_quotes(c: char, inside_quotes: &mut bool) -> Option<char> {
        match c {
            '"' => {
                *inside_quotes = !*inside_quotes;
                Some('_')
            }
            _ if *inside_quotes => None,
            _ => Some(c),
        }
    }

    fn contains_invalid_characters(address_part: &str) -> bool {
        address_part.contains(Self::is_character_not_allowed_in_email_address)
            || address_part.starts_with(Self::is_character_not_allowed_in_email_address_edges)
            || address_part.ends_with(Self::is_character_not_allowed_in_email_address_edges)
            || address_part.contains("..")
    }

    const fn is_character_not_allowed_in_email_address(c: char) -> bool {
        !(c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '+' || c == '_')
    }

    const fn is_character_not_allowed_in_email_address_edges(c: char) -> bool {
        !(c.is_ascii_alphanumeric() || c == '_')
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const VALID_EMAILS: [&str; 16] = [
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
        r#"firstname-lastname@example.com"#,
        r#"much."more\ unusual"@example.com"#,
        r#"very.unusual."@".unusual.com@example.com"#,
        r#"very."(),:;<>[]".VERY."very@\\ "very.unusual@strange.example.com"#,
        r#"email@123.123.123.123"#,
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

    const INVALID_EMAILS: [&str; 22] = [
        r#"plainaddress"#,
        r#"#@%^%#$@#$@#.com"#,
        r#"@example.com"#,
        r#"email@example.com (comment)"#,
        r#"Joe Smith <email@example.com>"#,
        r#"<email@example.com>"#,
        r#"email.example.com"#,
        r#"email@example@example.com"#,
        r#"quotes@inside"domain".com"#,
        r#".email@example.com"#,
        r#"email.@example.com"#,
        r#"email..email@example.com"#,
        r#"あいうえお@example.com"#,
        r#"email@example.com (Joe Smith)"#,
        r#"email@example"#,
        r#"email@-example.com"#,
        r#"email@[123.123.123.123]"#,
        r#"email@example..com"#,
        r#"Abc..123@example.com"#,
        r#"unclosed"quote@example@example.com"#,
        r"long_like_really_looooooooooooooooooooooooooooooooooooooong_email@example.com",
        "lond@domaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaain.com",
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

    mod remove_parts_inside_quotes {
        use super::*;

        #[test]
        fn replaces_string_in_quotes_with_two_underscores() {
            let input = r#""some text is here""#;
            let expected = "__";
            assert_eq!(EmailAddress::remove_parts_inside_quotes(input).unwrap(), expected);
        }

        #[test]
        fn replaces_substring_in_quotes_with_two_underscores() {
            let input = r#"text before "some text is here" text after"#;
            let expected = "text before __ text after";
            assert_eq!(EmailAddress::remove_parts_inside_quotes(input).unwrap(), expected);
        }

        #[test]
        fn multiple_quoted_regions() {
            let input = r#""first one" and "second one""#;
            let expected = "__ and __";
            assert_eq!(EmailAddress::remove_parts_inside_quotes(input).unwrap(), expected);
        }

        #[test]
        fn returns_none_for_unclosed_quote() {
            let input = r#"aaa"aaa"#;
            assert_eq!(EmailAddress::remove_parts_inside_quotes(input), None);
        }
    }

    mod contains_invalid_characters {
        use super::*;

        #[test]
        fn returns_true_for_invalid_characters() {
            let input = r#"!@#$%^&*()=+[]{}|;:'",<>?/\"#;
            for c in input.chars() {
                assert!(EmailAddress::contains_invalid_characters(c.to_string().as_str()));
            }
        }

        #[test]
        fn returns_true_for_strings_starting_with_invalid_character() {
            let input = r#"+-."#;
            for c in input.chars() {
                assert!(EmailAddress::contains_invalid_characters(&format!("{c}a")));
            }
        }

        #[test]
        fn returns_true_for_strings_ending_with_invalid_character() {
            let input = r#"+-."#;
            for c in input.chars() {
                assert!(EmailAddress::contains_invalid_characters(&format!("a{c}")));
            }
        }

        #[test]
        fn returns_false_for_strings_with_no_invalid_characters() {
            let input = r#"_abcxyzABCxyz12314123+-.asd_"#;
            assert!(!EmailAddress::contains_invalid_characters(input));
        }
    }

    mod split_email {
        use super::*;

        #[test]
        fn returns_none_for_empty_string() {
            assert_eq!(EmailAddress::split_email(""), None);
        }

        #[test]
        fn returns_none_for_missing_at_symbol() {
            assert_eq!(EmailAddress::split_email("email.com"), None);
        }

        #[test]
        fn returns_none_for_empty_username() {
            assert_eq!(EmailAddress::split_email("@example.com"), None);
        }

        #[test]
        fn returns_none_for_empty_domainname() {
            assert_eq!(EmailAddress::split_email("email@"), None);
        }

        #[test]
        fn returns_username_and_domainname() {
            let input = "username@example.com";
            let expected = ("username", "example.com");
            assert_eq!(EmailAddress::split_email(input).unwrap(), expected);
        }
    }
}
