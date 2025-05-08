use std::time::Duration;

use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(new, Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub url: String,
    pub poll_interval: Duration,
    pub timeout: Duration,
}
