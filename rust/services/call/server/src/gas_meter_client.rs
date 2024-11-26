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

use std::ops::Deref;

use serde::Serialize;
use server_utils::{RpcClient, RpcError, RpcServerMock};
use tracing::info;

use crate::handlers::v_call::types::CallHash;

#[derive(Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct StartGasMeter {
    hash: CallHash,
    gas_limit: u64,
    /// Time-to-live expressed in milliseconds.
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

pub struct GasMeterClient {
    client: RpcClient,
    hash: CallHash,
    time_to_live: u64,
}

const START_GAS_METER_METHOD: &str = "v_startGasMeter";

impl GasMeterClient {
    pub fn new(url: &str, hash: CallHash, time_to_live: u64) -> Self {
        let client = RpcClient::new(url, START_GAS_METER_METHOD);
        Self {
            client,
            hash,
            time_to_live,
        }
    }

    pub async fn start_gas_meter(&self, gas_limit: u64) -> Result<(), RpcError> {
        let req = StartGasMeter {
            hash: self.hash,
            gas_limit,
            time_to_live: self.time_to_live,
        };
        info!("{START_GAS_METER_METHOD} => {req:#?}");
        let resp = self.client.call(&req).await?;
        info!("  <= {resp:#?}");
        // We need to validate response here.
        Ok(())
    }
}

pub struct GasMeterServerMock {
    mock_server: RpcServerMock,
}

impl GasMeterServerMock {
    pub async fn start(params: impl Serialize, result: impl Serialize) -> GasMeterServerMock {
        let mock_server = RpcServerMock::start(START_GAS_METER_METHOD, true, params, result).await;
        Self { mock_server }
    }
}

impl Deref for GasMeterServerMock {
    type Target = RpcServerMock;

    fn deref(&self) -> &Self::Target {
        &self.mock_server
    }
}
