use std::env::set_var;

use alloy_chains::Chain;
use alloy_primitives::{address, uint};
use alloy_sol_types::SolCall;

use crate::{
    test_harness::{
        contracts::usdt::{
            BLOCK_NO,
            IERC20::{balanceOfCall, balanceOfReturn},
            USDT,
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

#[tokio::test]
async fn erc20_balance_of() -> anyhow::Result<()> {
    let location: ExecutionLocation = (Chain::mainnet().id(), BLOCK_NO).into();
    let binance_8 = address!("F977814e90dA44bFA03b6295A0616a897441aceC");
    let call = Call::new(USDT, &balanceOfCall { account: binance_8 });
    let result = run("usdt_erc20_balance_of", call, &location).await?;
    let raw_call_result = result.guest_output.evm_call_result;
    let balanceOfReturn { _0: balance } =
        balanceOfCall::abi_decode_returns(&raw_call_result, true)?;

    assert_eq!(balance, uint!(3_000_000_000_000_000_U256));

    Ok(())
}
