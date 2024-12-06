mod private {
    alloy_sol_types::sol!(
        #[sol(all_derives = true)]
        "../../../../contracts/vlayer/src/Seal.sol"
    );
}

pub mod ser {
    use alloy_primitives::FixedBytes;
    use serde::{Serialize, Serializer};

    use super::{ProofMode, Seal};

    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn ser_proof_mode<S>(mode: &ProofMode, state: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let as_u8: u8 = match mode {
            ProofMode::GROTH16 => 0,
            ProofMode::FAKE => 1,
            _ => panic!("unexpected enum variant for ProofMode"),
        };
        state.serialize_u8(as_u8)
    }

    #[derive(Serialize)]
    #[serde(remote = "Seal")]
    #[allow(non_snake_case)]
    pub struct SealDef {
        verifierSelector: FixedBytes<4>,
        seal: [FixedBytes<32>; 8_usize],
        #[serde(serialize_with = "ser_proof_mode")]
        mode: ProofMode,
    }
}

pub use private::{ProofMode, Seal};
