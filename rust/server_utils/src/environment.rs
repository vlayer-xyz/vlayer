use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(
    Debug, Clone, Deserialize, Serialize, Copy, PartialEq, Eq, Display, EnumString, Default,
)]
#[strum(ascii_case_insensitive)]
pub enum Environment {
    #[default]
    Testnet,
    Mainnet,
}
