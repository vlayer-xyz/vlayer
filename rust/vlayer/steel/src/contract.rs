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
use alloy_sol_types::SolCall;
use revm::{
    primitives::{
        CfgEnvWithHandlerCfg, ExecutionResult, ResultAndState, SuccessReason, TransactTo,
    },
    Database, Evm,
};
use std::{fmt::Debug, marker::PhantomData};

/// Represents a contract that is initialized with a specific environment and contract address.
///
/// **Note:** This contract is not type-safe. Ensure that the deployed contract at the specified
/// address matches the ABI used for making calls.
///
/// ### Usage
/// - **Preflight calls on the Host:** The environment can be initialized using
///   [EthEvmEnv::from_rpc] or [EvmEnv::new].
/// - **Calls in the Guest:** The environment should be constructed using [EvmInput::into_env].
///
/// ### Examples
/// ```rust no_run
/// # use vlayer_steel::{ethereum::EthEvmEnv, contract::{call::{guest_evm_call, evm_call}, CallTxData}};
/// # use alloy_primitives::{address};
/// # use alloy_sol_types::sol;
///
/// # fn main() -> anyhow::Result<()> {
/// let contract_address = address!("dAC17F958D2ee523a2206206994597C13D831ec7");
/// sol! {
///     interface IERC20 {
///         function balanceOf(address account) external view returns (uint);
///     }
/// }
///
/// let get_balance = IERC20::balanceOfCall {
///     account: address!("F977814e90dA44bFA03b6295A0616a897441aceC"),
/// };
///
/// // Host:
/// let mut env = EthEvmEnv::from_rpc("https://ethereum-rpc.publicnode.com", None)?;
/// let call_data = CallTxData::new(contract_address, &get_balance);
/// evm_call(call_data, &mut env)?;
///
/// let evm_input = env.into_input()?;
///
/// // Guest:
/// let evm_env = evm_input.into_env();
/// let call_data = CallTxData::new(contract_address, &get_balance);
/// guest_evm_call(call_data, &evm_env);
///
/// # Ok(())
/// # }
/// ```
///
/// [EvmInput::into_env]: crate::EvmInput::into_env
/// [EvmEnv::new]: crate::EvmEnv::new
/// [EthEvmEnv::from_rpc]: crate::ethereum::EthEvmEnv::from_rpc

#[derive(Debug, Clone)]
pub struct CallTxData<C> {
    pub caller: Address,
    pub gas_limit: u64,
    pub gas_price: U256,
    pub to: Address,
    pub value: U256,
    pub data: Vec<u8>,
    phantom: PhantomData<C>,
}

// We can't derive `Default` for `CallTxData` as it would require `C: Default`
impl<C> Default for CallTxData<C> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
            // We can't use `..Default::default()` here as it would cause recursion
            caller: Default::default(),
            gas_limit: Default::default(),
            gas_price: Default::default(),
            to: Default::default(),
            value: Default::default(),
            data: Default::default(),
        }
    }
}

impl<C> CallTxData<C> {
    const DEFAULT_GAS_LIMIT: u64 = 30_000_000;

    pub fn new(address: Address, call: &C) -> Self
    where
        C: SolCall,
    {
        Self {
            caller: address, // by default the contract calls itself
            to: address,
            data: call.abi_encode(),
            gas_limit: Self::DEFAULT_GAS_LIMIT,
            ..Default::default()
        }
    }

    pub fn new_from_bytes(address: Address, call: Vec<u8>) -> Self {
        Self {
            caller: address, // by default the contract calls itself
            to: address,
            data: call,
            gas_limit: Self::DEFAULT_GAS_LIMIT,
            ..Default::default()
        }
    }
}

/// Executes the call in the provided [Evm].
fn transact<C, DB>(mut evm: Evm<'_, (), DB>, tx: CallTxData<C>) -> Result<Vec<u8>, String>
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
