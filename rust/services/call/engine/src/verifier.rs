pub mod chain_proof;
pub mod time_travel;
pub mod travel_call;
pub mod zk_proof;

#[cfg(test)]
mod tests;

macro_rules! sealed_trait {
    () => {
        mod seal {
            pub trait Sealed {}
        }
    };
}
pub(crate) use sealed_trait;

macro_rules! verifier_trait {
    (($($arg_name:ident: $arg_type:ty),*) -> $result:ty) => {
        pub trait IVerifier: seal::Sealed + Send + Sync {
            fn verify(&self, $($arg_name: $arg_type),*) -> $result;
        }
        static_assertions::assert_obj_safe!(IVerifier);
    };
    (async ($($arg_name:ident: $arg_type:ty),*) -> $result:ty) => {
        #[async_trait]
        pub trait IVerifier: seal::Sealed + Send + Sync {
            async fn verify(&self, $($arg_name: $arg_type),*) -> $result;
        }
        static_assertions::assert_obj_safe!(IVerifier);
    };
}
pub(crate) use verifier_trait;

macro_rules! impl_verifier_for_fn {
    (($($arg_name:ident: $arg_type:ty),*) -> $result:ty) => {
        #[cfg(any(test, feature = "testing"))]
        impl<F> IVerifier for F
        where
            F: Fn($($arg_type),*) -> $result + Send + Sync
        {
            fn verify(&self, $($arg_name: $arg_type),*) -> $result {
                self($($arg_name),*)
            }
        }
    };
    (async ($($arg_name:ident: $arg_type:ty),*) -> $result:ty) => {
        #[cfg(any(test, feature = "testing"))]
        #[async_trait::async_trait]
        impl<F> IVerifier for F
        where
            F: Fn($($arg_type),*) -> $result + Send + Sync
        {
            async fn verify(&self, $($arg_name: $arg_type),*) -> $result {
                self($($arg_name),*)
            }
        }
    };
}
pub(crate) use impl_verifier_for_fn;

macro_rules! impl_sealed_for_fn {
    (($($arg_type:ty),*)) => {
        #[cfg(any(test, feature = "testing"))]
        impl<F> seal::Sealed for F
        where
            F: Fn($($arg_type),*) -> Result + Send + Sync
        {
        }
    };
}
pub(crate) use impl_sealed_for_fn;

macro_rules! setup_verifier_mocking {
    (($($arg_name:ident: $arg_type:ty),*) -> $result:ty) => {
        sealed_trait!();
        impl_sealed_for_fn!(($($arg_type),*));
        verifier_trait!(($($arg_name: $arg_type),*) -> $result);
        impl_verifier_for_fn!(($($arg_name: $arg_type),*) -> $result);
    };
    (async ($($arg_name:ident: $arg_type:ty),*) -> $result:ty) => {
        sealed_trait!();
        impl_sealed_for_fn!(($($arg_type),*));
        verifier_trait!(async ($($arg_name: $arg_type),*) -> $result);
        impl_verifier_for_fn!(async ($($arg_name: $arg_type),*) -> $result);
    };
}
pub(crate) use setup_verifier_mocking;
