use derive_new::new;

#[derive(Debug, Clone, PartialEq, Eq, new)]
pub struct Header<'x, T> {
    pub name: &'x [u8],
    pub value: &'x [u8],
    pub header: T,
}
