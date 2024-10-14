use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GuestOutput {
    pub total_cycles: u64,
}

impl From<GuestOutput> for Vec<u8> {
    fn from(value: GuestOutput) -> Self {
        bincode::serialize(&value).expect("Failed to serialize GuestOutput")
    }
}

impl TryFrom<&[u8]> for GuestOutput {
    type Error = bincode::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        bincode::deserialize(value)
    }
}
