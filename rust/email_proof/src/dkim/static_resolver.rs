use mail_auth::common::parse::TxtRecordParser;
use mail_auth::common::resolve::{IntoFqdn, Resolve, UnwrapTxtRecord};
use mail_auth::Txt;
use std::sync::Arc;

pub struct StaticResolver {}

const HARDCODED_DNS_RECORD: &str =
    "newengland._domainkey.example.com v=DKIM1; \
p=MIGJAoGBALVI635dLK4cJJAH3Lx6upo3X/Lm1tQz3mezcWTA3BUBnyIsdnRf57aD5BtNmhPrYYDlWlzw3UgnKisIxktkk5+iMQMlFtAS10JB8L3YadXNJY+JBcbeSi5TgJe4WFzNgW95FWDAuSTRXSWZfA/8xjflbTLDx0euFZOM7C4T0GwLAgMBAAE=";

impl Resolve for StaticResolver {
    async fn txt_lookup<'x, T: TxtRecordParser + Into<Txt> + UnwrapTxtRecord>(
        &self,
        _key: impl IntoFqdn<'x>,
    ) -> mail_auth::Result<Arc<T>> {
        Ok(Arc::new(T::parse(HARDCODED_DNS_RECORD.as_bytes())?))
    }
}
