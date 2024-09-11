use thiserror::Error;

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct DkimHeader {
    v: String,
    a: String,
    d: String,
    s: String,
    c: Option<String>,
    q: Option<String>,
    i: Option<String>,
    // t and x are not required by RFC, but recommended and we should require it
    t: String,
    x: String,
    l: Option<String>,
    h: String,
    z: Option<String>,
    bh: String,
    b: String,
}

#[allow(dead_code)]
#[derive(Error, Debug, PartialEq)]
pub enum DkimError {
    #[error("Parse error: {0}")]
    ParseError(String),
}

#[allow(dead_code)]
impl DkimHeader {
    pub(crate) fn parse(header: &str) -> Result<Self, DkimError> {
        let tags = header.split(';');
        let mut header = DkimHeader::default();
        for tag in tags {
            let mut parts = tag.splitn(2, '=');

            let key = parts
                .next()
                .ok_or(DkimError::ParseError(format!(
                    "Invalid tag: {}",
                    tag.trim()
                )))?
                .trim();

            let value: String = parts
                .next()
                .ok_or(DkimError::ParseError(format!(
                    "Invalid tag: {}",
                    tag.trim()
                )))?
                .trim()
                .into();

            match key {
                "v" => header.v = value,
                "a" => header.a = value,
                "d" => header.d = value,
                "s" => header.s = value,
                "c" => header.c = Some(value),
                "q" => header.q = Some(value),
                "i" => header.i = Some(value),
                "t" => header.t = value,
                "x" => header.x = value,
                "l" => header.l = Some(value),
                "h" => header.h = value,
                "z" => header.z = Some(value),
                "bh" => header.bh = value,
                "b" => header.b = value,
                unknown_tag => {
                    return Err(DkimError::ParseError(format!(
                        "Unknown DKIM tag: {unknown_tag}"
                    )))
                }
            }
        }
        header.validate_required_tags()?;
        Ok(header)
    }

    fn validate_required_tags(&self) -> Result<(), DkimError> {
        Self::validate_required_tag(&self.v, "v")?;
        Self::validate_required_tag(&self.a, "a")?;
        Self::validate_required_tag(&self.d, "d")?;
        Self::validate_required_tag(&self.s, "s")?;
        Self::validate_required_tag(&self.t, "t")?;
        Self::validate_required_tag(&self.x, "x")?;
        Self::validate_required_tag(&self.h, "h")?;
        Self::validate_required_tag(&self.bh, "bh")?;
        Self::validate_required_tag(&self.b, "b")?;
        Ok(())
    }

    fn validate_required_tag(tag: &str, name: &str) -> Result<(), DkimError> {
        if tag.is_empty() {
            return Err(DkimError::ParseError(format!("{name} tag is required")));
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    const TEST_SIGNATURE: &str = "v=1; a=rsa-sha256; d=example.net; s=brisbane;
     c=relaxed/simple; q=dns/txt; i=foo@eng.example.net;
     t=1117574938; x=1118006938; l=200;
     h=from:to:subject:date:keywords:keywords;
     z=From:foo@eng.example.net|To:joe@example.com|\
       Subject:demo=20run|Date:July=205,=202005=203:44:08=20PM=20-0700;
     bh=MTIzNDU2Nzg5MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTI=;
     b=dzdVyOfAKCdLXdJOc9G2q8LoXSlEniSbav+yuU4zGeeruD00lszZ\
              VoG4ZHRNiYzR";

    #[test]
    fn parse_correct_dkim_header() {
        let header = super::DkimHeader::parse(TEST_SIGNATURE).unwrap();
        assert_eq!(header.v, "1");
        assert_eq!(header.a, "rsa-sha256");
        assert_eq!(header.d, "example.net");
        assert_eq!(header.s, "brisbane");
        assert_eq!(header.c, Some("relaxed/simple".to_string()));
        assert_eq!(header.q, Some("dns/txt".to_string()));
        assert_eq!(header.i, Some("foo@eng.example.net".to_string()));
        assert_eq!(header.t, "1117574938");
        assert_eq!(header.x, "1118006938");
        assert_eq!(header.l, Some("200".to_string()));
        assert_eq!(
            header.h,
            "from:to:subject:date:keywords:keywords".to_string()
        );
        assert_eq!(
            header.z,
            Some(
                "From:foo@eng.example.net|To:joe@example.com|Subject:demo=20run|Date:July=205,=202005=203:44:08=20PM=20-0700"
                    .to_string()
            )
        );
        assert_eq!(
            header.bh,
            "MTIzNDU2Nzg5MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTI=".to_string()
        );
        assert_eq!(
            header.b,
            "dzdVyOfAKCdLXdJOc9G2q8LoXSlEniSbav+yuU4zGeeruD00lszZVoG4ZHRNiYzR".to_string()
        );
    }

    #[test]
    fn error_when_no_required_tag() {
        fn expect_error_with_tag_removed(tag: &str) {
            let parse_result = super::DkimHeader::parse(&rm_tag(tag));
            assert!(parse_result.is_err());
            assert_eq!(
                parse_result.unwrap_err().to_string(),
                format!("Parse error: {tag} tag is required")
            );
        }

        expect_error_with_tag_removed("v");
        expect_error_with_tag_removed("a");
        expect_error_with_tag_removed("d");
        expect_error_with_tag_removed("s");
        expect_error_with_tag_removed("t");
        expect_error_with_tag_removed("x");
        expect_error_with_tag_removed("h");
        expect_error_with_tag_removed("bh");
        expect_error_with_tag_removed("b");
    }

    #[test]
    fn error_when_unexpected_tag() {
        let parse_result = super::DkimHeader::parse(&format!("{TEST_SIGNATURE}; unexpected=tag"));
        assert!(parse_result.is_err());
        assert_eq!(
            parse_result.unwrap_err().to_string(),
            "Parse error: Unknown DKIM tag: unexpected"
        );
    }

    #[test]
    fn error_when_invalid_tag() {
        let parse_result =
            super::DkimHeader::parse(&format!("{TEST_SIGNATURE}; this_tag-has-no-equal-sign"));
        assert!(parse_result.is_err());
        assert_eq!(
            parse_result.unwrap_err().to_string(),
            "Parse error: Invalid tag: this_tag-has-no-equal-sign"
        );
    }

    #[test]
    fn parses_when_optional_tags_missing() {
        assert_eq!(super::DkimHeader::parse(&rm_tag("c")).unwrap().c, None);
        assert_eq!(super::DkimHeader::parse(&rm_tag("q")).unwrap().q, None);
        assert_eq!(super::DkimHeader::parse(&rm_tag("i")).unwrap().i, None);
        assert_eq!(super::DkimHeader::parse(&rm_tag("l")).unwrap().l, None);
        assert_eq!(super::DkimHeader::parse(&rm_tag("z")).unwrap().z, None);
    }

    fn rm_tag(tag_name: &str) -> String {
        TEST_SIGNATURE
            .split(';')
            .filter(|t| !t.trim_start().starts_with(&format!("{tag_name}=")))
            .collect::<Vec<&str>>()
            .join(";")
    }
}
