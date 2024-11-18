use std::env::set_var;

use alloy_chains::Chain;
use alloy_primitives::{address, uint};
use alloy_sol_types::SolCall;
use lazy_static::lazy_static;

use crate::{
    test_harness::{
        contracts::{
            IERC20::{balanceOfCall, balanceOfReturn},
            USDT, USDT_BLOCK_NO,
        },
        run, ExecutionLocation,
    },
    Call,
};

#[cfg(test)]
#[ctor::ctor]
fn before_all() {
    set_var("RISC0_DEV_MODE", "1");
}

lazy_static! {
    static ref LOCATION: ExecutionLocation = (Chain::mainnet().id(), USDT_BLOCK_NO).into();
}

#[tokio::test]
async fn erc20_balance_of() -> anyhow::Result<()> {
    let binance_8 = address!("F977814e90dA44bFA03b6295A0616a897441aceC");
    let call = Call::new(USDT, balanceOfCall { account: binance_8 });
    let result = run("usdt_erc20_balance_of", call, &LOCATION).await?;
    let raw_call_result = result.guest_output.evm_call_result;
    let balanceOfReturn { _0: balance } =
        balanceOfCall::abi_decode_returns(&raw_call_result, true)?;

    assert_eq!(balance, uint!(3_000_000_000_000_000_U256));

    Ok(())
}
