use std::{num::TryFromIntError, time::Duration};

use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Int conversion: {0}")]
    TryFromInt(#[from] TryFromIntError),
}

#[derive(Copy, Clone, Serialize, Default)]
pub struct Metrics {
    pub gas: u64,
    pub cycles: u64,
    pub times: Times,
}

#[derive(Copy, Clone, Serialize, Default)]
pub struct Times {
    pub preflight: u64,
    pub proving: u64,
}

pub fn elapsed_time_as_millis_u64(elapsed_time: Duration) -> Result<u64, Error> {
    Ok(elapsed_time.as_millis().try_into()?)
}
