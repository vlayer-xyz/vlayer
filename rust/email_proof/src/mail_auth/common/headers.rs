impl<'x, T> Header<'x, T> {
    pub const fn new(name: &'x [u8], value: &'x [u8], header: T) -> Self {
        Header {
            name,
            value,
            header,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header<'x, T> {
    pub name: &'x [u8],
    pub value: &'x [u8],
    pub header: T,
}
