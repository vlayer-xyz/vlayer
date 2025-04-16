use chrono::prelude::Utc;

pub type Timestamp = u64;

pub trait Now {
    fn now() -> Timestamp;
}

#[derive(Clone)]
pub struct RTClock;

impl Now for RTClock {
    #[allow(clippy::cast_sign_loss, clippy::panic)]
    fn now() -> Timestamp {
        let now = Utc::now();
        if now.timestamp() < 0 {
            panic!("Timestamp < 0");
        }
        now.timestamp() as Timestamp
    }
}
