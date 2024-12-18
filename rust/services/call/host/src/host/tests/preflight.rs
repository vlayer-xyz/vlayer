use alloy_chains::{Chain, NamedChain::AnvilHardhat};
use alloy_primitives::{address, b256, uint, Address};
use ethers_core::types::BlockNumber as BlockTag;

use crate::{
    host::tests::GAS_LIMIT,
    test_harness::{preflight, ExecutionLocation},
    Call,
};

mod usdt {
    use super::*;
    use crate::test_harness::contracts::usdt::{
        BLOCK_NO, IERC20::balanceOfCall, OPTIMISM_BLOCK_NO, OPTIMISM_USDT, USDT,
    };

    #[tokio::test(flavor = "multi_thread")]
    async fn erc20_balance_of() -> anyhow::Result<()> {
        let location: ExecutionLocation = (Chain::mainnet().id(), BLOCK_NO).into();
        let binance_8 = address!("F977814e90dA44bFA03b6295A0616a897441aceC");
        let call = Call::new(USDT, &balanceOfCall { account: binance_8 }, GAS_LIMIT);
        let result = preflight::<balanceOfCall>("usdt_erc20_balance_of", call, &location).await?;
        assert_eq!(result._0, uint!(3_000_000_000_000_000_U256));

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn optimism_erc20_balance_of() -> anyhow::Result<()> {
        let location: ExecutionLocation =
            (Chain::optimism_mainnet().id(), OPTIMISM_BLOCK_NO).into();
        let binance = address!("acD03D601e5bB1B275Bb94076fF46ED9D753435A");
        let call = Call::new(OPTIMISM_USDT, &balanceOfCall { account: binance }, GAS_LIMIT);
        let result = preflight::<balanceOfCall>("usdt_erc20_balance_of", call, &location).await?;

        assert_eq!(result._0, uint!(40_819_866_868_520_U256));

        Ok(())
    }
}

mod uniswap {
    use super::*;
    use crate::test_harness::contracts::uniswap::{
        IUniswapV3Factory::{ownerCall, ownerReturn},
        UNISWAP,
    };

    #[tokio::test(flavor = "multi_thread")]
    async fn factory_owner() -> anyhow::Result<()> {
        let location: ExecutionLocation = (Chain::mainnet().id(), BlockTag::Latest).into();
        let call = Call::new(UNISWAP, &ownerCall {}, GAS_LIMIT);
        let ownerReturn { _0: owner } =
            preflight::<ownerCall>("uniswap_factory_owner", call, &location).await?;
        assert_eq!(owner, address!("1a9c8182c09f50c8318d769245bea52c32be35bc")); // UNI Timelock

        Ok(())
    }
}

mod view {
    use lazy_static::lazy_static;

    use super::*;
    use crate::test_harness::contracts::view::{
        ViewCallTest::{
            testBlockhashCall, testChainidCall, testEoaAccountCall, testMuliContractCallsCall,
            testNonexistentAccountCall, testPrecompileCall,
        },
        BLOCK_NO, VIEW_CALL,
    };

    lazy_static! {
        static ref LOCATION: ExecutionLocation = (Chain::sepolia().id(), BlockTag::Latest).into();
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn precompile() -> anyhow::Result<()> {
        let call = Call::new(VIEW_CALL, &testPrecompileCall {}, GAS_LIMIT);
        let result = preflight::<testPrecompileCall>("view_precompile", call, &LOCATION).await?;
        assert_eq!(
            result._0,
            b256!("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
        );

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn nonexistent_account() -> anyhow::Result<()> {
        let sol_call = testNonexistentAccountCall {};
        let call = Call::new(VIEW_CALL, &sol_call, GAS_LIMIT);
        let result =
            preflight::<testNonexistentAccountCall>("view_nonexistent_account", call, &LOCATION)
                .await?;
        assert_eq!(result.size, uint!(0_U256));

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn eoa_account() -> anyhow::Result<()> {
        let call = Call::new(VIEW_CALL, &testEoaAccountCall {}, GAS_LIMIT);
        let result = preflight::<testEoaAccountCall>("view_eoa_account", call, &LOCATION).await?;
        assert_eq!(result.size, uint!(0_U256));

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn blockhash() -> anyhow::Result<()> {
        let call = Call::new(VIEW_CALL, &testBlockhashCall {}, GAS_LIMIT);
        let result = preflight::<testBlockhashCall>(
            "view_blockhash",
            call,
            &(Chain::sepolia().id(), BLOCK_NO).into(),
        )
        .await?;
        assert_eq!(
            result._0,
            b256!("7703fe4a3d6031a579d52ce9e493e7907d376cfc3b41f9bc7710b0dae8c67f68")
        );

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn chainid() -> anyhow::Result<()> {
        let call = Call::new(VIEW_CALL, &testChainidCall {}, GAS_LIMIT);
        let result = preflight::<testChainidCall>("view_chainid", call, &LOCATION).await?;
        assert_eq!(result._0, uint!(11_155_111_U256));

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn multi_contract_calls() -> anyhow::Result<()> {
        let call = Call::new(VIEW_CALL, &testMuliContractCallsCall {}, GAS_LIMIT);
        let result =
            preflight::<testMuliContractCallsCall>("view_multi_contract_calls", call, &LOCATION)
                .await?;
        assert_eq!(result._0, uint!(84_U256));

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn call_eoa() -> anyhow::Result<()> {
        let vitalik = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
        let call = Call::new(vitalik, &testEoaAccountCall {}, GAS_LIMIT);
        preflight::<testEoaAccountCall>("view_call_eoa", call, &LOCATION)
            .await
            .expect_err("calling an EOA should fail");

        Ok(())
    }
}

// Generated using `simple_teleport` example
mod teleport {
    use super::*;
    use crate::test_harness::contracts::teleport::{
        SimpleTravelProver::crossChainBalanceOfCall, BLOCK_NO, SIMPLE_TELEPORT,
    };

    #[tokio::test(flavor = "multi_thread")]
    async fn teleport_to_unknown_chain_returns_an_error_but_does_not_panic() -> anyhow::Result<()> {
        let location: ExecutionLocation = (AnvilHardhat, BLOCK_NO).into();
        let owner = Address::ZERO;
        let call = Call::new(SIMPLE_TELEPORT, &crossChainBalanceOfCall { owner }, GAS_LIMIT);
        let result = preflight::<crossChainBalanceOfCall>("simple_teleport", call, &location).await;
        assert_eq!(
            result.unwrap_err().to_string(),
            "TravelCallExecutor error: Panic: Intercepted call failed: EvmEnv(\"No rpc cache for chain: 8453\")"
        );

        Ok(())
    }
}

// Generated using `simple_time_travel` example
// Computes average balance of OP Sepolia USDC for TOKEN_OWNER on blocks from 17915294 to 17985294 with a step of 9000
// Accesses 9 blocks in total
mod time_travel {
    use super::*;
    use crate::test_harness::contracts::time_travel::{
        AverageBalance::{averageBalanceOfCall, averageBalanceOfReturn},
        AVERAGE_BALANCE_OF_CALL, BLOCK_NO, SIMPLE_TIME_TRAVEL,
    };

    #[tokio::test(flavor = "multi_thread")]
    #[ignore = "Fails due to chain proofs issue"]
    async fn time_travel() -> anyhow::Result<()> {
        let location: ExecutionLocation = (Chain::optimism_sepolia().id(), BLOCK_NO).into();
        let call = Call::new(SIMPLE_TIME_TRAVEL, &AVERAGE_BALANCE_OF_CALL, GAS_LIMIT);

        let averageBalanceOfReturn {
            _2: average_balance,
            ..
        } = preflight::<averageBalanceOfCall>("simple_time_travel", call, &location).await?;

        assert_eq!(average_balance, uint!(1_874_845_031_590_000_U256));

        Ok(())
    }
}
