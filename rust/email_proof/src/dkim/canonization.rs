#![allow(dead_code)]
mod relaxed;
mod simple;

#[derive(Debug, PartialEq)]
enum CanonizationType {
    Relaxed,
    Simple,
}

#[derive(Debug, PartialEq)]
pub struct Canonization {
    headers: CanonizationType,
    body: CanonizationType,
}

impl TryFrom<&str> for CanonizationType {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim() {
            "relaxed" => Ok(CanonizationType::Relaxed),
            "simple" => Ok(CanonizationType::Simple),
            _ => Err("Invalid canonization type"),
        }
    }
}

impl TryFrom<&str> for Canonization {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (header, body) = value.split_once('/').ok_or("Invalid canonization type")?;
        let headers = CanonizationType::try_from(header)?;
        let body = CanonizationType::try_from(body)?;
        Ok(Self { headers, body })
    }
}

impl Canonization {
    pub fn canonize_body(&self, body: &str) -> String {
        match self.body {
            CanonizationType::Relaxed => relaxed::canonize_body(body),
            CanonizationType::Simple => simple::canonize_body(body),
        }
    }

    pub fn canonize_headers<'a>(
        &self,
        headers: impl Iterator<Item = (&'a str, &'a str)>,
    ) -> String {
        match self.headers {
            CanonizationType::Relaxed => relaxed::canonize_headers(headers),
            CanonizationType::Simple => simple::canonize_headers(headers),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestCase {
        pub headers: &'static [(&'static str, &'static str)],
        pub body: &'static str,
        pub relaxed_headers: &'static str,
        pub relaxed_body: &'static str,
        pub simple_headers: &'static str,
        pub simple_body: &'static str,
    }

    impl TestCase {
        const fn new(
            input: (&'static [(&'static str, &'static str)], &'static str),
            relaxed_headers: &'static str,
            relaxed_body: &'static str,
            simple_headers: &'static str,
            simple_body: &'static str,
        ) -> Self {
            Self {
                headers: input.0,
                body: input.1,
                relaxed_headers,
                relaxed_body,
                simple_headers,
                simple_body,
            }
        }
    }

    const TEST_FIXTURES: [TestCase; 5] = [
        TestCase::new(
            (
                &[("A", " X\r\n"), ("B ", " Y\t\r\n\tZ  \r\n")],
                concat!(" C \r\n", "D \t E\r\n"),
            ),
            concat!("a:X\r\n", "b:Y Z\r\n",),
            concat!(" C\r\n", "D E\r\n"),
            "A: X\r\nB : Y\t\r\n\tZ  \r\n",
            " C \r\nD \t E\r\n",
        ),
        TestCase::new(
            (
                &[
                    ("  From ", " John\tdoe <jdoe@domain.com>\t\r\n"),
                    ("SUB JECT", "\ttest  \t  \r\n"),
                ],
                concat!(" body \t   \r\n", "\r\n", "\r\n",),
            ),
            concat!(" from:John doe <jdoe@domain.com>\r\n", "sub ject:test\r\n"),
            " body\r\n",
            concat!("  From : John\tdoe <jdoe@domain.com>\t\r\n", "SUB JECT:\ttest  \t  \r\n"),
            " body \t   \r\n",
        ),
        TestCase::new(
            (&[("H", " value\t\r\n")], "\r\n"),
            "h:value\r\n",
            "\r\n",
            "H: value\t\r\n",
            "\r\n",
        ),
        TestCase::new(
            (&[("\tx\t", " \t\t\tz\r\n")], "abc"),
            " x:z\r\n",
            "abc\r\n",
            "\tx\t: \t\t\tz\r\n",
            "abc\r\n",
        ),
        TestCase::new(
            (&[("Subject", " hello\r\n\r\n")], "\r\n"),
            "subject:hello\r\n\r\n",
            "\r\n",
            "Subject: hello\r\n\r\n",
            "\r\n",
        ),
    ];

    #[test]
    fn canonization_all_types() {
        let simple_simple = Canonization::try_from("simple/simple").unwrap();
        let relaxed_relaxed = Canonization::try_from("relaxed/relaxed").unwrap();

        for TestCase {
            headers,
            body,
            simple_body,
            simple_headers,
            relaxed_body,
            relaxed_headers,
        } in TEST_FIXTURES
        {
            assert_eq!(simple_simple.canonize_headers(headers.iter().copied()), simple_headers);
            assert_eq!(simple_simple.canonize_body(body), simple_body);

            assert_eq!(relaxed_relaxed.canonize_headers(headers.iter().copied()), relaxed_headers);
            assert_eq!(relaxed_relaxed.canonize_body(body), relaxed_body);
        }
    }

    mod try_from {
        use super::*;

        #[test]
        fn simple_simple() {
            let canonization = Canonization::try_from("simple/simple");
            assert_eq!(
                canonization,
                Ok(Canonization {
                    body: CanonizationType::Simple,
                    headers: CanonizationType::Simple,
                })
            );
        }

        #[test]
        fn relaxed_relaxed() {
            let canonization = Canonization::try_from("relaxed/relaxed");
            assert_eq!(
                canonization,
                Ok(Canonization {
                    body: CanonizationType::Relaxed,
                    headers: CanonizationType::Relaxed,
                })
            );
        }

        #[test]
        fn simple_relaxed() {
            let canonization = Canonization::try_from("simple/relaxed");
            assert_eq!(
                canonization,
                Ok(Canonization {
                    headers: CanonizationType::Simple,
                    body: CanonizationType::Relaxed,
                })
            );
        }

        #[test]
        fn relaxed_simple() {
            let canonization = Canonization::try_from("relaxed/simple");
            assert_eq!(
                canonization,
                Ok(Canonization {
                    headers: CanonizationType::Relaxed,
                    body: CanonizationType::Simple,
                })
            );
        }

        #[test]
        fn invalid() {
            let canonization = Canonization::try_from("relaxed");
            assert_eq!(canonization, Err("Invalid canonization type"));
        }
    }
}
