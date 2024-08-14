pub struct Entry {
    pub key: Box<[u8]>,
    pub value: Box<[u8]>,
}

impl<K, V> From<(K, V)> for Entry
where
    K: AsRef<[u8]>,
    V: AsRef<[u8]>,
{
    fn from((key, value): (K, V)) -> Self {
        Entry {
            key: key.as_ref().into(),
            value: value.as_ref().into(),
        }
    }
}
