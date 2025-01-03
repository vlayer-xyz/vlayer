use clap::builder::{IntoResettable, Resettable, Str};
pub use version::version;

pub struct Version;

impl IntoResettable<Str> for Version {
    fn into_resettable(self) -> Resettable<Str> {
        version().into_resettable()
    }
}
