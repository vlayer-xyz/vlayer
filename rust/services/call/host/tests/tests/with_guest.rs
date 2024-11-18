use alloy_chains::Chain;
use alloy_primitives::{address, uint};
use alloy_sol_types::SolCall;
use call_host::{self, Call};

use crate::harness::run;

mod usdt {

    use super::*;
    use crate::harness::contracts::{IERC20, USDT, USDT_BLOCK_NO};

    #[tokio::test]
    async fn erc20_balance_of() -> anyhow::Result<()> {
        let sol_call = IERC20::balanceOfCall {
            account: address!("F977814e90dA44bFA03b6295A0616a897441aceC"), // Binance 8
        };
        let call = Call {
            to: USDT,
            data: sol_call.abi_encode(),
        };
        let result =
            run("usdt_erc20_balance_of", call, &(Chain::mainnet().id(), USDT_BLOCK_NO).into())
                .await?;
        let raw_call_result = result.guest_output.evm_call_result;
        let call_result = IERC20::balanceOfCall::abi_decode_returns(&raw_call_result, true)?;

        assert_eq!(call_result._0, uint!(3_000_000_000_000_000_U256));

        Ok(())
    }
}
