use crate::canonization::simple;
use regex::Regex;

pub fn canonize_body(body: &str) -> String {
    let with_replaced_whitespaces = replace_whitespace_sequences(body);
    let with_replaced_eols = remove_end_of_line_spaces(&with_replaced_whitespaces);
    simple::canonize_body(&with_replaced_eols)
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

#[cfg(test)]
mod test {
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

    mod canonize_body_relaxed {
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
}
