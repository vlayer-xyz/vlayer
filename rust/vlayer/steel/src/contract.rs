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
pub mod call;
pub mod db;

use crate::EvmBlockHeader;
use alloy_primitives::{Address, Sealed, U256};
use revm::{
    primitives::{
        CfgEnvWithHandlerCfg, ExecutionResult, ResultAndState, SuccessReason, TransactTo,
    },
    Database, Evm,
};
use std::fmt::Debug;

#[derive(Debug, Clone, Default)]
pub struct CallTxData {
    pub caller: Address,
    pub gas_limit: u64,
    pub gas_price: U256,
    pub to: Address,
    pub value: U256,
    pub data: Vec<u8>,
}

impl CallTxData {
    const DEFAULT_GAS_LIMIT: u64 = 30_000_000;

    pub fn new_from_bytes(caller: Address, to: Address, data: Vec<u8>) -> Self {
        Self {
            caller,
            to,
            data,
            gas_limit: Self::DEFAULT_GAS_LIMIT,
            ..Default::default()
        }
    }
}

/// Executes the call in the provided [Evm].
fn transact<DB>(mut evm: Evm<'_, (), DB>, tx: CallTxData) -> Result<Vec<u8>, String>
where
    DB: Database,
    <DB as Database>::Error: Debug,
{
    let tx_env = evm.tx_mut();
    tx_env.caller = tx.caller;
    tx_env.gas_limit = tx.gas_limit;
    tx_env.gas_price = tx.gas_price;
    tx_env.transact_to = TransactTo::call(tx.to);
    tx_env.value = tx.value;
    tx_env.data = tx.data.into();

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
