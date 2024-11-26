//! This module is responsible for communicating with a gas metering service.
//! Gas metering is optional in which case every computation (preflight and proving)
//! will always be executed regardless of set gas limits by the user.
//!
//! The general diagram of communication happening between the Prover and the GasMeter can
//! be summarised as follows:
//!
//! -------                    ------------           v_startGasMeter         -------------
//! | SDK | ---- v_call ---->  |  Prover  |   ------- v_reportGasUsage --->   |  GasMeter |
//! -------                    ------------                                   -------------
//!
//! Thus as you can see, GasMeter is completely transparent to the SDK and may or may not
//! be used to schedule computations on the Prover.
//!
//! Communication is facilitated via JSON RPC using the following two methods:
//!
//! v_startGasMeter:
//! {
//!   jsonrpc: 2.0,
//!   method: "v_startGasMeter",
//!   id: number,
//!   params: [{
//!     hash: hex,
//!     gas_limit: number,
//!     time_to_live: number,
//!   }],
//! }
//!
//! v_reportGasUsage:
//! {
//!   jsonrpc: 2.0,
//!   method: "v_reportGasUsage",
//!   id: number,
//!   params: [{
//!     hash: hex,
//!     computation_kind: [preflight|proving],
//!     gas_amount: number,
//!   }],
//! }
//!

use derive_more::Deref;
use derive_new::new;
use serde::{Deserialize, Serialize};
use server_utils::{RpcClient, RpcError, RpcServerMock};
use tracing::info;

use crate::handlers::v_call::types::CallHash;

#[derive(new, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct StartGasMeter {
    hash: CallHash,
    gas_limit: u64,
    /// Time-to-live expressed in minutes.
    time_to_live: u64,
}

#[derive(Serialize, Debug)]
#[allow(unused)]
pub enum ComputationKind {
    Preflight,
    Proving,
}

#[derive(Serialize, Debug)]
#[serde(deny_unknown_fields)]
#[allow(unused)]
struct ReportGasUsage {
    hash: CallHash,
    computation_kind: ComputationKind,
    gas_amount: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub url: String,
    /// Time-to-live expressed in minutes.
    pub time_to_live: u64,
}

pub struct Client {
    client: RpcClient,
    hash: CallHash,
    time_to_live: u64,
}

const V_START_GAS_METER: &str = "v_startGasMeter";

impl Client {
    pub fn new(url: &str, hash: CallHash, time_to_live: u64) -> Self {
        let client = RpcClient::new(url, V_START_GAS_METER);
        Self {
            client,
            hash,
            time_to_live,
        }
    }

    pub async fn start_gas_meter(&self, gas_limit: u64) -> Result<(), RpcError> {
        let req = StartGasMeter::new(self.hash, gas_limit, self.time_to_live);
        info!("{V_START_GAS_METER} => {req:#?}");
        let resp = self.client.call(&req).await?;
        info!("  <= {resp:#?}");
        // We need to validate response here.
        Ok(())
    }
}

#[derive(Deref)]
pub struct ServerMock(RpcServerMock);

impl ServerMock {
    pub async fn start(params: impl Serialize, result: impl Serialize) -> ServerMock {
        let mock_server = RpcServerMock::start(V_START_GAS_METER, true, params, result).await;
        Self(mock_server)
    }
}
