pub(crate) type Timestamp = u64;

pub(crate) trait Now {
    fn now() -> Timestamp;
}

#[derive(Clone)]
pub(crate) struct RTClock;

impl Now for RTClock {
    fn now() -> Timestamp {
        0
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
