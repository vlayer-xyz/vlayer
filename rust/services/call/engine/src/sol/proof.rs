mod private {
    use crate::{CallAssumptions, Seal};

    alloy_sol_types::sol!("../../../../contracts/vlayer/src/Proof.sol");
}

pub mod ser {
    use alloy_primitives::{FixedBytes, U256};
    use serde::{Serialize, Serializer};

    use crate::{
        sol::{call_assumptions::ser::CallAssumptionsDef, seal::ser::SealDef},
        CallAssumptions, Seal,
    };

    use super::Proof;

    fn ser_length<S>(length: &U256, state: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        state.serialize_u64(u64::try_from(length).expect("value to fit into u64"))
    }

    #[derive(Serialize)]
    #[serde(remote = "Proof")]
    #[allow(non_snake_case)]
    pub struct ProofDef {
        #[serde(with = "SealDef")]
        seal: Seal,
        callGuestId: FixedBytes<32>,
        #[serde(serialize_with = "ser_length")]
        length: U256,
        #[serde(with = "CallAssumptionsDef")]
        callAssumptions: CallAssumptions,
    }
}

pub use private::Proof;
