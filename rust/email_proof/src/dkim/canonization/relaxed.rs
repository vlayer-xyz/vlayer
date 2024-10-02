use super::simple;
use regex::Regex;

pub fn canonize_body(body: &str) -> String {
    let with_replaced_whitespaces = replace_whitespace_sequences(body);
    let with_replaced_eols = remove_end_of_line_spaces(&with_replaced_whitespaces);
    simple::canonize_body(&with_replaced_eols)
}

pub fn canonize_headers<'a>(headers: impl Iterator<Item = (&'a str, &'a str)>) -> String {
    headers
        .map(|(name, value)| canonize_header(name, value))
        .collect::<Vec<_>>()
        .join("")
}

// Replace sequences of WSP characters with a single space.
// See https://www.rfc-editor.org/rfc/rfc6376#section-3.4.4
fn replace_whitespace_sequences(text: &str) -> String {
    let whitespaces_regex = Regex::new(r"[\x20\x09]+").unwrap();
    whitespaces_regex.replace_all(text, " ").into()
}

fn remove_end_of_line_spaces(text: &str) -> String {
    let end_of_line_spaces_regex = Regex::new(r"[ \t]+\r\n").unwrap();
    end_of_line_spaces_regex.replace_all(text, "\r\n").into()
}

fn canonize_header(name: &str, value: &str) -> String {
    let canonized_name = canonize_header_name(name);
    let canonized_value = canonize_header_value(value);
    format!("{}:{}", canonized_name, canonized_value)
}

fn canonize_header_name(name: &str) -> String {
    let name = name.to_ascii_lowercase();
    let name = replace_whitespace_sequences(&name);
    let name = name.trim_end();

    name.into()
}

fn canonize_header_value(value: &str) -> String {
    let value = unfold_continuation_lines(value);
    let value = replace_whitespace_sequences(&value);
    let value = remove_end_of_line_spaces(&value);
    let value = value.trim_start();

    value.into()
}

fn unfold_continuation_lines(value: &str) -> String {
    let terminators_followed_with_wsp_regex = Regex::new(r"\r\n[\x20\x09]+").unwrap();
    terminators_followed_with_wsp_regex
        .replace_all(value, " ")
        .into()
}

#[cfg(test)]
mod test {
    use super::*;

    mod relaxed_canonizaton {
        use super::*;

        mod replace_whitespace_sequence {
            use super::*;

            #[test]
            fn keeps_single_spaces() {
                let text = " Hello, world! ";
                assert_eq!(" Hello, world! ", replace_whitespace_sequences(text));
            }

            #[test]
            fn keeps_tabs_with_space() {
                let text = "\tHello, world!";
                assert_eq!(" Hello, world!", replace_whitespace_sequences(text));
            }

            #[test]
            fn replaces_sequences_os_spaces() {
                let text = "\t  \t Hello,   world! \t  \t";
                assert_eq!(" Hello, world! ", replace_whitespace_sequences(text));
            }

            #[test]
            fn works_for_multiline_strings() {
                let multiline = "\t  \t Hello,  \r
                 world \t  \t !  \t  \t\r
                 Hi    \r\n";
                assert_eq!(
                    " Hello, \r\n world ! \r\n Hi \r\n",
                    replace_whitespace_sequences(multiline)
                );
            }
        }

        mod remove_end_of_line_spaces {
            use super::*;

            #[test]
            fn removes_spaces_before_crlf() {
                let text = "Hello, world! \r\n";
                assert_eq!("Hello, world!\r\n", remove_end_of_line_spaces(text));
            }

            #[test]
            fn removes_tabs_before_crlf() {
                let text = "Hello, world!\t\r\n";
                assert_eq!("Hello, world!\r\n", remove_end_of_line_spaces(text));
            }

            #[test]
            fn removes_spaces_and_tabs_before_crlf() {
                let text = "Hello, world! \t\t  \t \r\n";
                assert_eq!("Hello, world!\r\n", remove_end_of_line_spaces(text));
            }

            #[test]
            fn works_for_multiline_string() {
                let text =
                    "Hello, world! \t\t  \t \r\nHello, world! \t\t  \t \r\nHello, world! \t\t  \t \r\n";
                assert_eq!(
                    "Hello, world!\r\nHello, world!\r\nHello, world!\r\n",
                    remove_end_of_line_spaces(text)
                );
            }
        }

        mod canonize_body {
            use super::*;

            #[test]
            fn canonize_body_removes_trailing_newlines() {
                let body = "Hello, world!  \r\n \t  \r\n  \t\r\n";
                assert_eq!("Hello, world!\r\n", canonize_body(body));
            }

            #[test]
            fn canonize_body_keeps_non_empty_lines() {
                let body = "Hello, world  \r\n \t  \r\n  !\t\r\n  \r\n";
                assert_eq!("Hello, world\r\n\r\n !\r\n", canonize_body(body));
            }

            #[test]
            fn canonize_body_adds_newline() {
                let body = "Hello, world!";
                assert_eq!("Hello, world!\r\n", canonize_body(body));
            }

            #[test]
            fn empty_string_becomes_crlf() {
                let body = "";
                assert_eq!("\r\n", canonize_body(body));
            }
        }

        mod unfold_continuation_lines {
            use super::*;

            #[test]
            fn keeps_single_line() {
                let line = "Hello, world!";
                assert_eq!("Hello, world!", unfold_continuation_lines(line));
            }

            #[test]
            fn replaces_terminators_followed_with_whitespace_with_whitespace() {
                let line = "Hello, world!\r\n   \t  \r\n  !\t\r\n  \r\n";
                assert_eq!("Hello, world!  !\t \r\n", unfold_continuation_lines(line));
            }

            #[test]
            fn keeps_crlf_on_the_end_of_text() {
                let line = "Hello, world!\r\n";
                assert_eq!("Hello, world!\r\n", unfold_continuation_lines(line));
            }

            #[test]
            fn keeps_crlf_not_followed_by_wsp() {
                let line = "Hello, \r\nworld!";
                assert_eq!("Hello, \r\nworld!", unfold_continuation_lines(line));
            }
        }

        mod canonize_single_header {
            use super::*;

            #[test]
            fn lowercases_only_header_name() {
                let name = "FrOm";
                let value = "aAaA";
                assert_eq!("from:aAaA", canonize_header(name, value));
            }

            #[test]
            fn removes_trailing_whitespaces_in_name() {
                let name = "From \t ";
                let value = "aAaA";
                assert_eq!("from:aAaA", canonize_header(name, value));
            }

            #[test]
            fn removes_leading_whitespaces_in_value() {
                let name = "From";
                let value = " \t aAaA";
                assert_eq!("from:aAaA", canonize_header(name, value));
            }

            #[test]
            fn removes_whitespace_sequences() {
                let name = "Fr \t om";
                let value = "aA \t aA";
                assert_eq!("fr om:aA aA", canonize_header(name, value));
            }

            #[test]
            fn replaces_continuation_lines() {
                let name = "From";
                let value = "Hello\r\n \tWorld  \r\n  !\t\r\n  \r\n";
                assert_eq!("from:Hello World !\r\n", canonize_header(name, value));
            }
        }

        mod canonize_headers {
            use super::*;

            #[test]
            fn empty_headers() {
                let headers = vec![].into_iter();
                assert_eq!("", canonize_headers(headers));
            }

            #[test]
            fn joins_headers_into_canonized_sequence() {
                let headers = vec![("From ", " aa \r\n\r\n"), ("To", "bb\r\n")].into_iter();
                assert_eq!("from:aa\r\n\r\nto:bb\r\n", canonize_headers(headers));
            }
        }
    }
}
