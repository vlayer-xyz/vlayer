use std::{collections::HashMap, sync::Arc};

use call_engine::{Call, HostOutput};
use call_host::Host;
use derive_more::{Deref, DerefMut};
use parking_lot::RwLock;
use v_call::types::CallHash;

use crate::{
    error::AppError,
    gas_meter::{Client as GasMeterClient, ComputationStage},
};

pub mod v_call;
pub mod v_get_proof_receipt;
pub mod v_versions;

pub type SharedState = Arc<RwLock<State>>;

#[derive(Deref, DerefMut, Default, Debug)]
pub struct State(HashMap<CallHash, Result<HostOutput, AppError>>);

pub async fn generate_proof(
    call: Call,
    host: Host,
    gas_meter_client: Option<GasMeterClient>,
) -> Result<HostOutput, AppError> {
    let prover = host.prover();
    let call_guest_id = host.call_guest_id();
    let preflight_result = host.preflight(call).await?;
    let gas_used = preflight_result.gas_used;

    if let Some(client) = gas_meter_client.as_ref() {
        client
            .refund_unused_gas(ComputationStage::Preflight, gas_used)
            .await?;
    }

    let host_output = Host::prove(&prover, call_guest_id, preflight_result)?;

    if let Some(client) = gas_meter_client {
        client
            .refund_unused_gas(ComputationStage::Proving, gas_used)
            .await?;
    }

    Ok(host_output)
}
