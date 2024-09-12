use std::ops::Deref;

macro_rules! dkim_tag {
    ($name:ident) => {
        #[derive(Default, Debug, PartialEq)]
        pub(crate) struct $name(pub String);

        impl Deref for $name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

dkim_tag!(Version);
dkim_tag!(SigningAlgorithm);
dkim_tag!(SigningDomainIdentifier);
dkim_tag!(Selector);
dkim_tag!(Canonicalization);
dkim_tag!(QueryMethod);
dkim_tag!(Identity);
dkim_tag!(Timestamp);
dkim_tag!(Expiration);
dkim_tag!(BodyLength);
dkim_tag!(SignedHeaders);
dkim_tag!(CopiedHeaders);
dkim_tag!(BodyHash);
dkim_tag!(Signature);
