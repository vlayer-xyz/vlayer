use super::{
    common::{crypto::HashAlgorithm, headers::Header},
    dkim::{signature::Signature, Canonicalization},
};

pub struct AuthenticatedMessage<'x> {
    pub headers: Vec<(&'x [u8], &'x [u8])>,
    pub dkim_headers: Vec<Header<'x, crate::mail_auth::Result<Signature>>>,
    pub body_hashes: Vec<(Canonicalization, HashAlgorithm, u64, Vec<u8>)>,
}

impl<'x> AuthenticatedMessage<'x> {
    pub fn signed_headers<'z: 'x>(
        &'z self,
        headers: &'x [String],
        dkim_hdr_name: &'x [u8],
        dkim_hdr_value: &'x [u8],
    ) -> impl Iterator<Item = (&'x [u8], &'x [u8])> {
        let mut last_header_pos: Vec<(&[u8], usize)> = Vec::new();
        headers
            .iter()
            .filter_map(move |h| {
                let header_pos = if let Some((_, header_pos)) = last_header_pos
                    .iter_mut()
                    .find(|(lh, _)| lh.eq_ignore_ascii_case(h.as_bytes()))
                {
                    header_pos
                } else {
                    last_header_pos.push((h.as_bytes(), 0));
                    &mut last_header_pos.last_mut().unwrap().1
                };
                if let Some((last_pos, result)) = self
                    .headers
                    .iter()
                    .rev()
                    .enumerate()
                    .skip(*header_pos)
                    .find(|(_, (mh, _))| h.as_bytes().eq_ignore_ascii_case(mh))
                {
                    *header_pos = last_pos + 1;
                    Some(*result)
                } else {
                    *header_pos = self.headers.len();
                    None
                }
            })
            .chain([(dkim_hdr_name, dkim_hdr_value)])
    }
}
