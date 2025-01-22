pub mod chain_proof;
pub mod time_travel;
pub mod travel_call;
pub mod zk_proof;

#[cfg(test)]
mod tests;

/// This is a framework for defining and implementing verifier interfaces in a security-critical environment. The goal is to:
/// * Ensure verifier traits are strictly controlled to prevent unauthorized implementations (sealed_trait!).
/// * Provide clear interfaces for both synchronous and asynchronous verifiers (verifier_trait!).
/// * Allow for lightweight, mockable verifier implementations in testing (impl_verifier_for_fn!).
/// * Consolidate these components for easy reuse (setup_verifier_mocking!).
mod mocking {
    /// Defines a seal module containing the Sealed trait, ensuring verifier traits cannot be implemented externally.
    /// This protects against unintended extensions or security bypasses.
    macro_rules! sealed_trait {
        () => {
            mod seal {
                pub trait Sealed {}
            }
        };
    }
    pub(crate) use sealed_trait;

    /// Defines a verifier trait (`IVerifier`) with a `verify` method. Supports both synchronous and asynchronous variants.
    /// Uses async_trait for async verifiers.
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

    /// Implements the `IVerifier`` trait for functions, enabling simple mock implementations during testing.
    /// Is disabled in non-testing environments.
    /// "testing" feature is needed to enable this functionality in dependent crates tests.
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

    /// Implements the `Sealed` trait for functions with the appropriate signature.
    /// Is disabled in non-testing environments.
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

    /// Combines all macros into a single invocation.
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
}
