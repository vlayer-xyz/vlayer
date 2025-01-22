pub mod chain_proof;
pub mod time_travel;
pub mod travel_call;
pub mod zk_proof;

#[cfg(test)]
mod tests;

macro_rules! sealed_trait {
    ($($arg_type:ty),*) => {
        mod seal {
            pub trait Sealed {}

            #[cfg(any(test, feature = "testing"))]
            impl<F> Sealed for F
            where
                F: Fn($($arg_type),*) -> super::Result + Send + Sync
            {
            }
        }
    };
}
pub(crate) use sealed_trait;

macro_rules! verifier_trait {
    (($($arg_name:ident: $arg_type:ty),*) -> $result:ty) => {
        pub trait IVerifier: seal::Sealed + Send + Sync {
            fn verify(&self, $($arg_name: $arg_type),*) -> $result;
        }
    };
    (async ($($arg_name:ident: $arg_type:ty),*) -> $result:ty) => {
        #[async_trait]
        pub trait IVerifier: seal::Sealed + Send + Sync {
            async fn verify(&self, $($arg_name: $arg_type),*) -> $result;
        }
    };
}
pub(crate) use verifier_trait;
