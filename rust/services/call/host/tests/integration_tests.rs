use alloy_chains::Chain;
use alloy_primitives::{address, b256, uint, Address};
use alloy_sol_types::SolCall;
use call_host::{self, Call};
use harness::{run, sepolia_latest_block, LATEST_BLOCK};

mod harness;

mod usdt {
    use harness::contracts::{IERC20, USDT, USDT_BLOCK_NO};

    use super::*;

    #[tokio::test]
    async fn erc20_balance_of() -> anyhow::Result<()> {
        let sol_call = IERC20::balanceOfCall {
            account: address!("F977814e90dA44bFA03b6295A0616a897441aceC"), // Binance 8
        };
        let call = Call {
            to: USDT,
            data: sol_call.abi_encode(),
        };
        let result = run::<IERC20::balanceOfCall>(
            "usdt_erc20_balance_of",
            call,
            &(Chain::mainnet().id(), USDT_BLOCK_NO).into(),
        )
        .await?;
        assert_eq!(result._0, uint!(3_000_000_000_000_000_U256));
        Ok(())
    }
}

mod uniswap {
    use harness::contracts::{IUniswapV3Factory, UNISWAP};

    use super::*;

    #[tokio::test]
    async fn factory_owner() -> anyhow::Result<()> {
        let sol_call = IUniswapV3Factory::ownerCall {};
        let call = Call {
            to: UNISWAP,
            data: sol_call.abi_encode(),
        };
        let result = run::<IUniswapV3Factory::ownerCall>(
            "uniswap_factory_owner",
            call,
            &(Chain::mainnet().id(), LATEST_BLOCK).into(),
        )
        .await?;
        assert_eq!(
            result._0,
            address!("1a9c8182c09f50c8318d769245bea52c32be35bc") // Uniswap V2: UNI Timelock is the current owner of the factory.
        );
        Ok(())
    }
}

mod view {
    use harness::contracts::{ViewCallTest, VIEW_CALL, VIEW_CALL_BLOCK_NO};

    use super::*;

    #[tokio::test]
    async fn precompile() -> anyhow::Result<()> {
        let sol_call = ViewCallTest::testPrecompileCall {};
        let call = Call {
            to: VIEW_CALL,
            data: sol_call.abi_encode(),
        };
        let result =
            run::<ViewCallTest::testPrecompileCall>("view_precompile", call, &sepolia_latest_block)
                .await?;
        assert_eq!(
            result._0,
            b256!("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
        );
        Ok(())
    }

    #[tokio::test]
    async fn nonexistent_account() -> anyhow::Result<()> {
        let sol_call = ViewCallTest::testNonexistentAccountCall {};
        let call = Call {
            to: VIEW_CALL,
            data: sol_call.abi_encode(),
        };
        let result = run::<ViewCallTest::testNonexistentAccountCall>(
            "view_nonexistent_account",
            call,
            &sepolia_latest_block,
        )
        .await?;
        assert_eq!(result.size, uint!(0_U256));
        Ok(())
    }

    #[tokio::test]
    async fn eoa_account() -> anyhow::Result<()> {
        let sol_call = ViewCallTest::testEoaAccountCall {};
        let call = Call {
            to: VIEW_CALL,
            data: sol_call.abi_encode(),
        };
        let result = run::<ViewCallTest::testEoaAccountCall>(
            "view_eoa_account",
            call,
            &sepolia_latest_block,
        )
        .await?;
        assert_eq!(result.size, uint!(0_U256));
        Ok(())
    }

    #[tokio::test]
    async fn blockhash() -> anyhow::Result<()> {
        let sol_call = ViewCallTest::testBlockhashCall {};
        let call = Call {
            to: VIEW_CALL,
            data: sol_call.abi_encode(),
        };
        let result = run::<ViewCallTest::testBlockhashCall>(
            "view_blockhash",
            call,
            &(Chain::sepolia().id(), VIEW_CALL_BLOCK_NO).into(),
        )
        .await?;
        assert_eq!(
            result._0,
            b256!("7703fe4a3d6031a579d52ce9e493e7907d376cfc3b41f9bc7710b0dae8c67f68")
        );
        Ok(())
    }

    #[tokio::test]
    async fn chainid() -> anyhow::Result<()> {
        let sol_call = ViewCallTest::testChainidCall {};
        let call = Call {
            to: VIEW_CALL,
            data: sol_call.abi_encode(),
        };
        let result =
            run::<ViewCallTest::testChainidCall>("view_chainid", call, &sepolia_latest_block)
                .await?;
        assert_eq!(result._0, uint!(11_155_111_U256));
        Ok(())
    }

    #[tokio::test]
    async fn multi_contract_calls() -> anyhow::Result<()> {
        let sol_call = ViewCallTest::testMuliContractCallsCall {};
        let call = Call {
            to: VIEW_CALL,
            data: sol_call.abi_encode(),
        };
        let result = run::<ViewCallTest::testMuliContractCallsCall>(
            "view_multi_contract_calls",
            call,
            &sepolia_latest_block,
        )
        .await?;
        assert_eq!(result._0, uint!(84_U256));
        Ok(())
    }

    #[tokio::test]
    async fn call_eoa() -> anyhow::Result<()> {
        let call = Call {
            to: address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045"), // vitalik.eth
            ..Default::default()
        };
        run::<ViewCallTest::testEoaAccountCall>("view_call_eoa", call, &sepolia_latest_block)
            .await
            .expect_err("calling an EOA should fail");

        Ok(())
    }
}

mod teleport {
    use alloy_chains::NamedChain;
    use harness::contracts::{SimpleTravelProver, BLOCK_NO, SIMPLE_TELEPORT};

    use super::*;

    #[tokio::test]
    async fn teleport_to_unknown_chain_returns_an_error_but_does_not_panic() -> anyhow::Result<()> {
        let sol_call = SimpleTravelProver::crossChainBalanceOfCall {
            owner: Address::ZERO,
        };
        let call = Call {
            to: SIMPLE_TELEPORT,
            data: sol_call.abi_encode(),
        };
        let result = run::<SimpleTravelProver::crossChainBalanceOfCall>(
            "simple_teleport",
            call,
            &(NamedChain::AnvilHardhat, BLOCK_NO).into(),
        )
        .await;
        assert_eq!(
            result.unwrap_err().to_string(),
            "TravelCallExecutor error: Panic: Intercepted call failed: EvmEnv(\"No rpc cache for chain: 8453\")"
        );

        Ok(())
    }
}
