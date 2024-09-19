use thiserror::Error;

use crate::dkim::tags::*;

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct DkimHeader {
    v: Version,
    a: SigningAlgorithm,
    d: SigningDomainIdentifier,
    s: Selector,
    c: Option<Canonicalization>,
    q: Option<QueryMethod>,
    i: Option<Identity>,
    // t and x are not required by RFC, but recommended and we should require it
    t: Timestamp,
    x: Expiration,
    l: Option<BodyLength>,
    h: SignedHeaders,
    z: Option<CopiedHeaders>,
    bh: BodyHash,
    b: Signature,
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
        let header = tags.fold(Ok(Self::default()), Self::fill_header)?;
        header.validate_required_tags()?;
        Ok(header)
    }

    fn fill_header(header: Result<Self, DkimError>, tag: &str) -> Result<Self, DkimError> {
        let (key, value) = Self::split_tag(tag)?;
        Self::do_fill_header(header?, key, value)
    }

    fn do_fill_header(mut header: Self, key: &str, value: String) -> Result<Self, DkimError> {
        match key {
            "v" => header.v = Version(value),
            "a" => header.a = SigningAlgorithm(value),
            "d" => header.d = SigningDomainIdentifier(value),
            "s" => header.s = Selector(value),
            "c" => header.c = Some(Canonicalization(value)),
            "q" => header.q = Some(QueryMethod(value)),
            "i" => header.i = Some(Identity(value)),
            "t" => header.t = Timestamp(value),
            "x" => header.x = Expiration(value),
            "l" => header.l = Some(BodyLength(value)),
            "h" => header.h = SignedHeaders(value),
            "z" => header.z = Some(CopiedHeaders(value)),
            "bh" => header.bh = BodyHash(value),
            "b" => header.b = Signature(value),
            unknown_tag => {
                return Err(DkimError::ParseError(format!("Unknown DKIM tag: {unknown_tag}")))
            }
        }

        Ok(header)
    }

    fn split_tag(tag: &str) -> Result<(&str, String), DkimError> {
        let mut parts = tag.splitn(2, '=');
        let key: &str = parts
            .next()
            .ok_or(DkimError::ParseError(format!("Invalid tag: {}", tag.trim())))?
            .trim();

        let value: String = parts
            .next()
            .ok_or(DkimError::ParseError(format!("Invalid tag: {}", tag.trim())))?
            .trim()
            .into();

        Ok((key, value))
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
    use super::*;

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
        let header = DkimHeader::parse(TEST_SIGNATURE).unwrap();
        assert_eq!(header.v.0, "1");
        assert_eq!(header.a.0, "rsa-sha256");
        assert_eq!(header.d.0, "example.net");
        assert_eq!(header.s.0, "brisbane");
        assert_eq!(header.c, Some(Canonicalization("relaxed/simple".to_string())));
        assert_eq!(header.q, Some(QueryMethod("dns/txt".to_string())));
        assert_eq!(header.i, Some(Identity("foo@eng.example.net".to_string())));
        assert_eq!(header.t.0, "1117574938");
        assert_eq!(header.x.0, "1118006938");
        assert_eq!(header.l, Some(BodyLength("200".to_string())));
        assert_eq!(header.h.0, "from:to:subject:date:keywords:keywords".to_string());
        assert_eq!(
            header.z,
            Some(CopiedHeaders(
                "From:foo@eng.example.net|To:joe@example.com|Subject:demo=20run|Date:July=205,=202005=203:44:08=20PM=20-0700"
                    .to_string()
            ))
        );
        assert_eq!(header.bh.0, "MTIzNDU2Nzg5MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTI=".to_string());
        assert_eq!(
            header.b.0,
            "dzdVyOfAKCdLXdJOc9G2q8LoXSlEniSbav+yuU4zGeeruD00lszZVoG4ZHRNiYzR".to_string()
        );
    }

    #[test]
    fn error_when_no_required_tag() {
        fn expect_error_with_tag_removed(tag: &str) {
            let parse_result = DkimHeader::parse(&rm_tag(tag));
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
        let parse_result = DkimHeader::parse(&format!("{TEST_SIGNATURE}; unexpected=tag"));
        assert!(parse_result.is_err());
        assert_eq!(
            parse_result.unwrap_err().to_string(),
            "Parse error: Unknown DKIM tag: unexpected"
        );
    }

    #[test]
    fn error_when_invalid_tag() {
        let parse_result =
            DkimHeader::parse(&format!("{TEST_SIGNATURE}; this_tag-has-no-equal-sign"));
        assert!(parse_result.is_err());
        assert_eq!(
            parse_result.unwrap_err().to_string(),
            "Parse error: Invalid tag: this_tag-has-no-equal-sign"
        );
    }

    #[test]
    fn parses_when_optional_tags_missing() {
        assert_eq!(DkimHeader::parse(&rm_tag("c")).unwrap().c, None);
        assert_eq!(DkimHeader::parse(&rm_tag("q")).unwrap().q, None);
        assert_eq!(DkimHeader::parse(&rm_tag("i")).unwrap().i, None);
        assert_eq!(DkimHeader::parse(&rm_tag("l")).unwrap().l, None);
        assert_eq!(DkimHeader::parse(&rm_tag("z")).unwrap().z, None);
    }

    fn rm_tag(tag_name: &str) -> String {
        TEST_SIGNATURE
            .split(';')
            .filter(|t| !t.trim_start().starts_with(&format!("{tag_name}=")))
            .collect::<Vec<&str>>()
            .join(";")
    }
}
