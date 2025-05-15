use std::env::set_var;

use alloy_chains::{Chain, NamedChain::AnvilHardhat};
use alloy_primitives::{address, uint};
use alloy_sol_types::SolCall;

use crate::{
    host::tests::call,
    test_harness::{ExecutionLocation, run, run_with_teleport},
};

#[cfg(test)]
#[ctor::ctor]
fn before_all() {
    unsafe {
        set_var("RISC0_DEV_MODE", "1");
    }
}

mod erc20 {
    use super::*;
    use crate::test_harness::contracts::usdt::{
        BLOCK_NO,
        IERC20::{balanceOfCall, balanceOfReturn},
        USDT,
    };

    #[tokio::test]
    async fn erc20_balance_of() -> anyhow::Result<()> {
        let location: ExecutionLocation = (Chain::mainnet().id(), BLOCK_NO).into();
        let binance_8 = address!("F977814e90dA44bFA03b6295A0616a897441aceC");
        let call = call(USDT, &balanceOfCall { account: binance_8 });
        let result = run("usdt_erc20_balance_of", call, &location).await?;
        let raw_call_result = result.guest_output.evm_call_result;
        let balanceOfReturn { _0: balance } =
            balanceOfCall::abi_decode_returns(&raw_call_result, true)?;

        assert_eq!(balance, uint!(3_000_000_000_000_000_U256));

        Ok(())
    }
}

mod teleport {
    use call_rpc::OP_ANVIL;
    use optimism::client::factory::cached;

    use super::*;
    use crate::test_harness::contracts::teleport::{
        BLOCK_NO, JOHN, OUTPUT, SIMPLE_TELEPORT,
        SimpleTeleportProver::{crossChainBalanceOfCall, crossChainBalanceOfReturn},
        TOKEN,
    };

    #[tokio::test(flavor = "multi_thread")]
    async fn success() -> anyhow::Result<()> {
        let location: ExecutionLocation = (AnvilHardhat, BLOCK_NO).into();
        let call = call(
            SIMPLE_TELEPORT,
            &crossChainBalanceOfCall {
                owner: JOHN,
                tokens: vec![TOKEN],
            },
        );
        let op_client_factory =
            cached::Factory::from_single_sequencer_output(OP_ANVIL, OUTPUT.clone());
        let result = run_with_teleport("teleport", call, &location, op_client_factory)
            .await
            .unwrap();
        let raw_call_result = result.guest_output.evm_call_result;
        let crossChainBalanceOfReturn {
            _2: cross_chain_balance,
            ..
        } = crossChainBalanceOfCall::abi_decode_returns(&raw_call_result, true)?;
        assert_eq!(cross_chain_balance, uint!(100_U256));

        Ok(())
    }
}
