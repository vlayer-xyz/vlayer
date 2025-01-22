pub mod chain_proof;
pub mod time_travel;
pub mod travel_call;
pub mod zk_proof;

#[cfg(test)]
mod tests;

macro_rules! define_sealed_trait {
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
pub(crate) use define_sealed_trait;
