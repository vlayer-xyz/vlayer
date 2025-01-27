use chrono::prelude::Utc;

pub type Timestamp = u64;

pub trait Now {
    fn now() -> Timestamp;
}

#[derive(Clone)]
pub struct RTClock;

impl Now for RTClock {
    #[allow(clippy::cast_sign_loss)]
    fn now() -> Timestamp {
        let now = Utc::now();
        if now.timestamp() < 0 {
            panic!("Timestamp < 0");
        }
        now.timestamp() as Timestamp
    }
}

#[cfg(test)]
pub(super) mod tests_utils {
    use super::*;

    pub(crate) struct MockClock<const T: Timestamp>;

    impl<const T: Timestamp> Now for MockClock<T> {
        fn now() -> super::Timestamp {
            T
        }
    }
}
