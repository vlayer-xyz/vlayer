mod private {
    use crate::{CallAssumptions, Seal};

    alloy_sol_types::sol!("../../../../contracts/vlayer/src/Proof.sol");
}

pub mod ser {
    use alloy_primitives::{B256, U256};
    use serde::{Serialize, Serializer};

    use super::Proof;
    use crate::{
        sol::{call_assumptions::ser::CallAssumptionsDTO, seal::ser::SealDTO},
        CallAssumptions, Seal,
    };

    #[derive(Serialize)]
    #[serde(remote = "Proof")]
    #[allow(non_snake_case)]
    pub struct ProofDef {
        #[serde(with = "SealDTO")]
        seal: Seal,
        callGuestId: B256,
        #[serde(serialize_with = "ser_length")]
        length: U256,
        #[serde(with = "CallAssumptionsDTO")]
        callAssumptions: CallAssumptions,
    }

    fn ser_length<S>(length: &U256, state: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        state.serialize_u64(
            u64::try_from(length)
                .expect("failed to serialize length field of Proof. Value must fit into u64"),
        )
    }
}

pub use private::Proof;
