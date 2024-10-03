#![allow(dead_code)]
mod relaxed;
mod simple;

#[derive(Debug, PartialEq)]
pub struct CanonicalHeaders(Vec<u8>);

#[derive(Debug, PartialEq)]
pub struct CanonicalBody(Vec<u8>);

#[derive(Debug, PartialEq)]
enum CanonicalizationType {
    Relaxed,
    Simple,
}

#[derive(Debug, PartialEq)]
pub struct Canonicalization {
    headers: CanonicalizationType,
    body: CanonicalizationType,
}

impl TryFrom<&str> for CanonicalizationType {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim() {
            "relaxed" => Ok(CanonicalizationType::Relaxed),
            "simple" => Ok(CanonicalizationType::Simple),
            _ => Err("Invalid canonicalization type"),
        }
    }
}

impl TryFrom<&str> for Canonicalization {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (header, body) = value
            .split_once('/')
            .ok_or("Invalid canonicalization type")?;
        let headers = CanonicalizationType::try_from(header)?;
        let body = CanonicalizationType::try_from(body)?;
        Ok(Self { headers, body })
    }
}

impl CanonicalHeaders {
    fn new(value: String) -> Self {
        Self(value.into_bytes())
    }
}

impl CanonicalBody {
    fn new(value: String) -> Self {
        Self(value.into_bytes())
    }
}

impl Canonicalization {
    pub fn canonize_body(&self, body: &str) -> CanonicalBody {
        let canonical_body = match self.body {
            CanonicalizationType::Relaxed => relaxed::canonize_body(body),
            CanonicalizationType::Simple => simple::canonize_body(body),
        };

        CanonicalBody::new(canonical_body)
    }

    pub fn canonize_headers<'a>(
        &self,
        headers: impl Iterator<Item = (&'a str, &'a str)>,
    ) -> CanonicalHeaders {
        let canonical_headers = match self.headers {
            CanonicalizationType::Relaxed => relaxed::canonize_headers(headers),
            CanonicalizationType::Simple => simple::canonize_headers(headers),
        };

        CanonicalHeaders::new(canonical_headers)
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
    fn canonicalization_all_types() {
        let simple_simple = Canonicalization::try_from("simple/simple").unwrap();
        let relaxed_relaxed = Canonicalization::try_from("relaxed/relaxed").unwrap();

        for TestCase {
            headers,
            body,
            simple_body,
            simple_headers,
            relaxed_body,
            relaxed_headers,
        } in TEST_FIXTURES
        {
            assert_eq!(
                simple_simple.canonize_headers(headers.iter().copied()).0,
                simple_headers.as_bytes()
            );
            assert_eq!(simple_simple.canonize_body(body).0, simple_body.as_bytes());

            assert_eq!(
                relaxed_relaxed.canonize_headers(headers.iter().copied()).0,
                relaxed_headers.as_bytes()
            );
            assert_eq!(relaxed_relaxed.canonize_body(body).0, relaxed_body.as_bytes());
        }
    }

    mod try_from {
        use super::*;

        #[test]
        fn simple_simple() {
            let canonicalization = Canonicalization::try_from("simple/simple");
            assert_eq!(
                canonicalization,
                Ok(Canonicalization {
                    body: CanonicalizationType::Simple,
                    headers: CanonicalizationType::Simple,
                })
            );
        }

        #[test]
        fn relaxed_relaxed() {
            let canonicalization = Canonicalization::try_from("relaxed/relaxed");
            assert_eq!(
                canonicalization,
                Ok(Canonicalization {
                    body: CanonicalizationType::Relaxed,
                    headers: CanonicalizationType::Relaxed,
                })
            );
        }

        #[test]
        fn simple_relaxed() {
            let canonicalization = Canonicalization::try_from("simple/relaxed");
            assert_eq!(
                canonicalization,
                Ok(Canonicalization {
                    headers: CanonicalizationType::Simple,
                    body: CanonicalizationType::Relaxed,
                })
            );
        }

        #[test]
        fn relaxed_simple() {
            let canonicalization = Canonicalization::try_from("relaxed/simple");
            assert_eq!(
                canonicalization,
                Ok(Canonicalization {
                    headers: CanonicalizationType::Relaxed,
                    body: CanonicalizationType::Simple,
                })
            );
        }

        #[test]
        fn invalid() {
            let canonicalization = Canonicalization::try_from("relaxed");
            assert_eq!(canonicalization, Err("Invalid canonicalization type"));
        }
    }
}
