use mail_auth::common::parse::TxtRecordParser;
use mail_auth::common::resolve::{IntoFqdn, Resolve, UnwrapTxtRecord};
use mail_auth::Txt;
use std::sync::Arc;

pub struct StaticResolver<'a> {
    dns_record: &'a [u8],
}

impl<'a> StaticResolver<'a> {
    pub fn new(dns_record: &'a str) -> Self {
        Self {
            dns_record: dns_record.as_bytes(),
        }
    }
}

impl Resolve for StaticResolver<'_> {
    async fn txt_lookup<'x, T: TxtRecordParser + Into<Txt> + UnwrapTxtRecord>(
        &self,
        _key: impl IntoFqdn<'x>,
    ) -> mail_auth::Result<Arc<T>> {
        Ok(Arc::new(T::parse(self.dns_record)?))
    }
}
