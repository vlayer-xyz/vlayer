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
pub mod call_builder;
pub mod contract;
pub mod db;

use crate::EvmBlockHeader;
use alloy_primitives::{Address, Sealed, U256};
use alloy_sol_types::{SolCall, SolType};
use revm::{
    primitives::{
        CfgEnvWithHandlerCfg, ExecutionResult, ResultAndState, SuccessReason, TransactTo,
    },
    Database, Evm,
};
use std::{fmt::Debug, marker::PhantomData, mem};

/// Represents a contract that is initialized with a specific environment and contract address.
///
/// **Note:** This contract is not type-safe. Ensure that the deployed contract at the specified
/// address matches the ABI used for making calls.
///
/// ### Usage
/// - **Preflight calls on the Host:** To prepare calls on the host environment and build the
///   necessary proof, use [Contract::preflight]. The environment can be initialized using
///   [EthEvmEnv::from_rpc] or [EvmEnv::new].
/// - **Calls in the Guest:** To initialize the contract in the guest environment, use
///   [Contract::new]. The environment should be constructed using [EvmInput::into_env].
///
/// ### Examples
/// ```rust no_run
/// # use vlayer_steel::{ethereum::EthEvmEnv, Contract, contract::call_builder::{guest_evm_call, evm_call}};
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
/// let call_builder = CallBuilder::new(contract_address, &get_balance);
/// evm_call(call_builder, &mut env)?;
///
/// let evm_input = env.into_input()?;
///
/// // Guest:
/// let evm_env = evm_input.into_env();
/// let call_builder = CallBuilder::new(contract_address, &get_balance);
/// guest_evm_call(call_builder, &evm_env);
///
/// # Ok(())
/// # }
/// ```
///
/// [EvmInput::into_env]: crate::EvmInput::into_env
/// [EvmEnv::new]: crate::EvmEnv::new
/// [EthEvmEnv::from_rpc]: crate::ethereum::EthEvmEnv::from_rpc

/// Transaction data to be used with [CallBuilder] for an execution.
#[derive(Debug, Clone)]
struct CallTxData<C> {
    caller: Address,
    gas_limit: u64,
    gas_price: U256,
    to: Address,
    value: U256,
    data: Vec<u8>,
    phantom: PhantomData<C>,
}

impl<C: SolCall> CallTxData<C> {
    /// Compile-time assertion that the call C has a return value.
    const RETURNS: () = assert!(
        mem::size_of::<C::Return>() > 0,
        "Function call must have a return value"
    );
}

/// Executes the call in the provided [Evm].
fn transact<C, DB>(mut evm: Evm<'_, (), DB>, tx: CallTxData<C>) -> Result<C::Return, String>
where
    C: SolCall,
    DB: Database,
    <DB as Database>::Error: Debug,
{
    #[allow(clippy::let_unit_value)]
    let _ = CallTxData::<C>::RETURNS;

    let tx_env = evm.tx_mut();
    tx_env.caller = tx.caller;
    tx_env.gas_limit = tx.gas_limit;
    tx_env.gas_price = tx.gas_price;
    tx_env.transact_to = TransactTo::call(tx.to);
    tx_env.value = tx.value;
    tx_env.data = tx.data.into();

    let ResultAndState { result, .. } = evm
        .transact_preverified()
        .map_err(|err| format!("Call '{}' failed: {:?}", C::SIGNATURE, err))?;
    let ExecutionResult::Success { reason, output, .. } = result else {
        return Err(format!("Call '{}' failed", C::SIGNATURE));
    };
    // there must be a return value to decode
    if reason != SuccessReason::Return {
        return Err(format!(
            "Call '{}' did not return: {:?}",
            C::SIGNATURE,
            reason
        ));
    }
    let returns = C::abi_decode_returns(&output.into_data(), true).map_err(|err| {
        format!(
            "Call '{}' returned invalid type; expected '{}': {:?}",
            C::SIGNATURE,
            <C::ReturnTuple<'_> as SolType>::SOL_NAME,
            err
        )
    })?;

    Ok(returns)
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
