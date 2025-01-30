#[cfg(test)]
mod ensure_latest_teleport_location_is_confirmed {
    use alloy_primitives::B256;
    use anyhow::Result;

    use crate::verifier::teleport::{ensure_latest_teleport_location_is_confirmed, Error};

    #[test]
    fn success() -> Result<()> {
        let hash = B256::ZERO;
        let blocks = &[(1, hash), (2, hash)];

        ensure_latest_teleport_location_is_confirmed(blocks, 2)?;
        Ok(())
    }

    #[test]
    fn unconfirmed() -> Result<()> {
        let hash = B256::ZERO;
        let blocks = &[(1, hash), (2, hash)];

        let err = ensure_latest_teleport_location_is_confirmed(blocks, 1).unwrap_err();
        assert_eq!(err, Error::TeleportOnUnconfirmed);
        Ok(())
    }
}
