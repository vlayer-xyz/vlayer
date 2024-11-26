// Cycle count is non-deterministic so we ignore differences up to 10% when comparing.
// 5% was tried and was not enough
const TOLERANCE: f64 = 1.1;

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
pub fn apply_tolerance(cycles: u64) -> u64 {
    (cycles as f64 * TOLERANCE) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tolerance() {
        assert_eq!(apply_tolerance(1), 1);
        assert_eq!(apply_tolerance(100), 110);
        assert_eq!(apply_tolerance(1_027), 1_129);
    }
}
