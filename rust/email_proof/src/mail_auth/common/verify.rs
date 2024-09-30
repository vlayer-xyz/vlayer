use super::crypto::Algorithm;

pub trait VerifySignature {
    fn selector(&self) -> &str;

    fn domain(&self) -> &str;

    fn signature(&self) -> &[u8];

    fn algorithm(&self) -> Algorithm;

    fn domain_key(&self) -> String {
        let s = self.selector();
        let d = self.domain();
        let mut key = String::with_capacity(s.len() + d.len() + 13);
        key.push_str(s);
        key.push_str("._domainkey.");
        key.push_str(d);
        key.push('.');
        key
    }
}
