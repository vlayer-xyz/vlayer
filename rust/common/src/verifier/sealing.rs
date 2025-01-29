//! This is a framework for defining and implementing verifier interfaces in a security-critical environment. The goal is to:
//! * Ensure verifier traits are strictly controlled to prevent unauthorized implementations (sealed_trait!).
//! * Provide clear interfaces for both synchronous and asynchronous verifiers (verifier_trait!).
//! * Allow for lightweight, mockable verifier implementations in testing (impl_verifier_for_fn!).
//! * Consolidate these components for easy reuse (setup_verifier_mocking!).

/// Defines a seal module containing the Sealed trait, ensuring verifier traits cannot be implemented externally.
/// This protects against unintended extensions or security bypasses.
#[macro_export]
macro_rules! sealed_trait {
    () => {
        mod seal {
            pub trait Sealed {}
        }
    };
}
pub use sealed_trait;

/// Defines a verifier trait (`IVerifier`) with a `verify` method. Supports both synchronous and asynchronous variants.
/// Uses async_trait for async verifiers.
#[macro_export]
macro_rules! verifier_trait {
    ($trait_name:ident ($($arg_name:ident: $arg_type:ty),*) -> $result:ty) => {
        pub trait $trait_name: seal::Sealed + Send + Sync {
            fn verify(&self, $($arg_name: $arg_type),*) -> $result;
        }
    };
    (async $trait_name:ident ($($arg_name:ident: $arg_type:ty),*) -> $result:ty) => {
        #[async_trait]
        pub trait $trait_name: seal::Sealed + Send + Sync {
            async fn verify(&self, $($arg_name: $arg_type),*) -> $result;
        }
    };
}
pub use verifier_trait;

/// Implements the `IVerifier`` trait for functions, enabling simple mock implementations during testing.
/// Disabled in non-testing environments.
/// "testing" feature is needed to enable this functionality in dependent crates tests.
#[macro_export]
macro_rules! impl_verifier_for_fn {
    ($trait_name:ident ($($arg_name:ident: $arg_type:ty),*) -> $result:ty) => {
        #[cfg(any(test, feature = "testing"))]
        impl<F> $trait_name for F
        where
            F: Fn($($arg_type),*) -> $result + Send + Sync
        {
            fn verify(&self, $($arg_name: $arg_type),*) -> $result {
                self($($arg_name),*)
            }
        }
    };
    (async $trait_name:ident ($($arg_name:ident: $arg_type:ty),*) -> $result:ty) => {
        #[cfg(any(test, feature = "testing"))]
        #[async_trait::async_trait]
        impl<F> $trait_name for F
        where
            F: Fn($($arg_type),*) -> $result + Send + Sync
        {
            async fn verify(&self, $($arg_name: $arg_type),*) -> $result {
                self($($arg_name),*)
            }
        }
    };
}
pub use impl_verifier_for_fn;

/// Implements the `Sealed` trait for functions with the appropriate signature.
/// Disabled in non-testing environments.
#[macro_export]
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
pub use impl_sealed_for_fn;

/// Combines all macros into a single invocation.
#[macro_export]
macro_rules! sealed_with_test_mock {
    ($trait_name:ident ($($arg_name:ident: $arg_type:ty),*) -> $result:ty) => {
        $crate::verifier::sealing::sealed_trait!();
        $crate::verifier::sealing::impl_sealed_for_fn!(($($arg_type),*));
        $crate::verifier::sealing::verifier_trait!($trait_name ($($arg_name: $arg_type),*) -> $result);
        $crate::verifier::sealing::impl_verifier_for_fn!($trait_name ($($arg_name: $arg_type),*) -> $result);
    };
    (async $trait_name:ident ($($arg_name:ident: $arg_type:ty),*) -> $result:ty) => {
        $crate::verifier::sealing::sealed_trait!();
        $crate::verifier::sealing::impl_sealed_for_fn!(($($arg_type),*));
        $crate::verifier::sealing::verifier_trait!(async $trait_name ($($arg_name: $arg_type),*) -> $result);
        $crate::verifier::sealing::impl_verifier_for_fn!(async $trait_name ($($arg_name: $arg_type),*) -> $result);
    };
}
pub use sealed_with_test_mock;
