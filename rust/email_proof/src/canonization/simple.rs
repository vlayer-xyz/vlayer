pub fn canonize_body(body: &str) -> String {
    format!("{}\r\n", body.trim_end_matches("\r\n"))
}

pub fn canonize_headers<'a>(headers: impl Iterator<Item = (&'a str, &'a str)>) -> String {
    headers
        .map(|(name, value)| format!("{}:{}", name, value))
        .collect::<Vec<_>>()
        .join("")
}

#[cfg(test)]
mod test {
    use super::*;

    mod canonize_body_simple {
        use super::*;

        #[test]
        fn canonize_body_removes_trailing_newlines() {
            let body = "Hello, world!\r\n\r\n\r\n";
            assert_eq!("Hello, world!\r\n", canonize_body(body));
        }

        #[test]
        fn canonize_body_does_not_add_newline() {
            let body = "Hello, world!";
            assert_eq!("Hello, world!\r\n", canonize_body(body));
        }

        #[test]
        fn canonize_empty_body() {
            let body = "";
            assert_eq!("\r\n", canonize_body(body));
        }

        #[test]
        fn canonize_multiline_body() {
            let body = "Hello, world!\r\nThis is a test.";
            assert_eq!("Hello, world!\r\nThis is a test.\r\n", canonize_body(body));
        }
    }

    mod canonize_headers_simple {
        use super::*;

        #[test]
        fn canonize_headers_empty() {
            let headers = vec![].into_iter();
            assert_eq!("", canonize_headers(headers));
        }

        #[test]
        fn canonize_headers_single() {
            let headers = vec![("From", "aa\r\n\r\n"), ("To", "bb\r\n")].into_iter();
            assert_eq!("From:aa\r\n\r\nTo:bb\r\n", canonize_headers(headers));
        }
    }
}
