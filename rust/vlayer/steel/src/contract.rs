// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
pub mod engine;
pub mod db;

use crate::{guest::Call, EvmBlockHeader};
use alloy_primitives::Sealed;
use revm::{
    primitives::{
        CfgEnvWithHandlerCfg, ExecutionResult, ResultAndState, SuccessReason, TransactTo,
    },
    Database, Evm,
};
use std::fmt::Debug;

/// Executes the call in the provided [Evm].
fn transact<DB>(mut evm: Evm<'_, (), DB>, tx: &Call) -> Result<Vec<u8>, String>
where
    DB: Database,
    <DB as Database>::Error: Debug,
{
    let tx_env = evm.tx_mut();
    tx_env.caller = tx.caller;
    tx_env.transact_to = TransactTo::call(tx.to);
    tx_env.data = tx.data.clone().into();

    let ResultAndState { result, .. } = evm
        .transact_preverified()
        .map_err(|err| format!("Call failed: {:?}", err))?;
    let ExecutionResult::Success { reason, output, .. } = result else {
        return Err("Call failed".into());
    };

    // there must be a return value to decode
    if reason != SuccessReason::Return {
        return Err(format!("Call did not return: {:?}", reason));
    }

    Ok(output.into_data().into())
}

fn new_evm<'a, DB, H>(db: DB, cfg: CfgEnvWithHandlerCfg, header: &Sealed<H>) -> Evm<'a, (), DB>
where
    DB: Database,
    H: EvmBlockHeader,
{
    Evm::builder()
        .with_db(db)
        .with_cfg_env_with_handler_cfg(cfg)
        .modify_block_env(|blk_env| header.fill_block_env(blk_env))
        .build()
}
