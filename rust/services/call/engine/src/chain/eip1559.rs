use serde::{Deserialize, Serialize};

/// [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559) parameters.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct Eip1559Constants {
    pub base_fee_change_denominator: u64,
    pub base_fee_max_increase_denominator: u64,
    pub base_fee_max_decrease_denominator: u64,
    pub elasticity_multiplier: u64,
}

/// The gas constants as defined in [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559).
impl Default for Eip1559Constants {
    fn default() -> Self {
        Self {
            base_fee_change_denominator: 8,
            base_fee_max_increase_denominator: 8,
            base_fee_max_decrease_denominator: 8,
            elasticity_multiplier: 2,
        }
    }
}
