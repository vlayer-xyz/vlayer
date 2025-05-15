use alloy_chains::{Chain, NamedChain::AnvilHardhat};
use alloy_primitives::{address, b256, uint};
use ethers_core::types::BlockNumber as BlockTag;

use crate::{
    host::tests::call,
    test_harness::{ExecutionLocation, preflight},
};

mod usdt {
    use call_engine::Call;

    use super::*;
    use crate::test_harness::contracts::usdt::{
        BLOCK_NO, IERC20::balanceOfCall, OPTIMISM_BLOCK_NO, OPTIMISM_USDT, USDT,
    };

    #[tokio::test(flavor = "multi_thread")]
    async fn erc20_balance_of() -> anyhow::Result<()> {
        let location: ExecutionLocation = (Chain::mainnet().id(), BLOCK_NO).into();
        let binance_8 = address!("F977814e90dA44bFA03b6295A0616a897441aceC");
        let call = call(USDT, &balanceOfCall { account: binance_8 });
        let result = preflight::<balanceOfCall>("usdt_erc20_balance_of", call, &location).await?;
        assert_eq!(result._0, uint!(3_000_000_000_000_000_U256));

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn optimism_erc20_balance_of() -> anyhow::Result<()> {
        let location: ExecutionLocation =
            (Chain::optimism_mainnet().id(), OPTIMISM_BLOCK_NO).into();
        let binance = address!("acD03D601e5bB1B275Bb94076fF46ED9D753435A");
        let call = call(OPTIMISM_USDT, &balanceOfCall { account: binance });
        let result = preflight::<balanceOfCall>("usdt_erc20_balance_of", call, &location).await?;

        assert_eq!(result._0, uint!(40_819_866_868_520_U256));

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    #[should_panic(
        expected = "called `Result::unwrap()` on an `Err` value: EVM error: transaction validation error: call gas cost exceeds the gas limit"
    )]
    async fn fails_when_no_gas() {
        let location: ExecutionLocation = (Chain::mainnet().id(), BLOCK_NO).into();
        let binance_8 = address!("F977814e90dA44bFA03b6295A0616a897441aceC");
        let call = Call::new(USDT, &balanceOfCall { account: binance_8 }, 0);

        preflight::<balanceOfCall>("usdt_erc20_balance_of", call, &location)
            .await
            .unwrap();
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
        let call = call(UNISWAP, &ownerCall {});
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
        BLOCK_NO, VIEW_CALL,
        ViewCallTest::{
            testBlockhashCall, testChainidCall, testEoaAccountCall, testMuliContractCallsCall,
            testNonexistentAccountCall, testPrecompileCall,
        },
    };

    lazy_static! {
        static ref LOCATION: ExecutionLocation = (Chain::sepolia().id(), BlockTag::Latest).into();
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn precompile() -> anyhow::Result<()> {
        let call = call(VIEW_CALL, &testPrecompileCall {});
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
        let call = call(VIEW_CALL, &sol_call);
        let result =
            preflight::<testNonexistentAccountCall>("view_nonexistent_account", call, &LOCATION)
                .await?;
        assert_eq!(result.size, uint!(0_U256));

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn eoa_account() -> anyhow::Result<()> {
        let call = call(VIEW_CALL, &testEoaAccountCall {});
        let result = preflight::<testEoaAccountCall>("view_eoa_account", call, &LOCATION).await?;
        assert_eq!(result.size, uint!(0_U256));

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn blockhash() -> anyhow::Result<()> {
        let call = call(VIEW_CALL, &testBlockhashCall {});
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
        let call = call(VIEW_CALL, &testChainidCall {});
        let result = preflight::<testChainidCall>("view_chainid", call, &LOCATION).await?;
        assert_eq!(result._0, uint!(11_155_111_U256));

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn multi_contract_calls() -> anyhow::Result<()> {
        let call = call(VIEW_CALL, &testMuliContractCallsCall {});
        let result =
            preflight::<testMuliContractCallsCall>("view_multi_contract_calls", call, &LOCATION)
                .await?;
        assert_eq!(result._0, uint!(84_U256));

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn call_eoa() -> anyhow::Result<()> {
        let vitalik = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
        let call = call(vitalik, &testEoaAccountCall {});
        preflight::<testEoaAccountCall>("view_call_eoa", call, &LOCATION)
            .await
            .expect_err("calling an EOA should fail");

        Ok(())
    }
}

mod teleport {
    use call_rpc::OP_ANVIL;
    use optimism::client::factory::cached;

    use super::*;
    use crate::test_harness::{
        contracts::teleport::{
            BLOCK_NO, JOHN, OUTPUT, SIMPLE_TELEPORT,
            SimpleTeleportProver::{crossChainBalanceOfCall, crossChainBalanceOfReturn},
            TOKEN,
        },
        preflight_with_teleport,
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
        let crossChainBalanceOfReturn {
            _2: cross_chain_balance,
            ..
        } = preflight_with_teleport::<crossChainBalanceOfCall>(
            "teleport",
            call,
            &location,
            op_client_factory,
        )
        .await
        .unwrap();
        assert_eq!(cross_chain_balance, uint!(100_U256));

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn failure() -> anyhow::Result<()> {
        let mut wrong_token = TOKEN;
        wrong_token.chainId = uint!(331337_U256);

        let location: ExecutionLocation = (AnvilHardhat, BLOCK_NO).into();

        let call = call(
            SIMPLE_TELEPORT,
            &crossChainBalanceOfCall {
                owner: JOHN,
                tokens: vec![wrong_token.clone()],
            },
        );

        let error = preflight::<crossChainBalanceOfCall>("teleport", call, &location)
            .await
            .unwrap_err();

        let wrong_chain_id = wrong_token.chainId.to_string();

        assert!(
            error
                .to_string()
                .contains(&format!("No rpc cache for chain: {wrong_chain_id}"))
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
        AVERAGE_BALANCE_OF_CALL,
        AverageBalance::{averageBalanceOfCall, averageBalanceOfReturn},
        BLOCK_NO, SIMPLE_TIME_TRAVEL,
    };

    #[tokio::test(flavor = "multi_thread")]
    async fn time_travel() -> anyhow::Result<()> {
        let location: ExecutionLocation = (Chain::optimism_sepolia().id(), BLOCK_NO).into();
        let call = call(SIMPLE_TIME_TRAVEL, &AVERAGE_BALANCE_OF_CALL);

        let averageBalanceOfReturn {
            _2: average_balance,
            ..
        } = preflight::<averageBalanceOfCall>("time_travel", call, &location).await?;

        assert_eq!(average_balance, uint!(1_874_845_031_590_000_U256));

        Ok(())
    }
}

mod simple {
    use super::*;
    use crate::test_harness::contracts::simple::{
        BLOCK_NO, SIMPLE,
        SimpleProver::{balanceCall, balanceReturn},
    };

    #[tokio::test(flavor = "multi_thread")]
    async fn balance_call() -> anyhow::Result<()> {
        let location: ExecutionLocation = (Chain::optimism_sepolia().id(), BLOCK_NO).into();
        let binance_8 = address!("F977814e90dA44bFA03b6295A0616a897441aceC");
        let call = call(SIMPLE, &balanceCall { _owner: binance_8 });
        let balanceReturn { _2: balance, .. } =
            preflight::<balanceCall>("simple", call, &location).await?;

        assert_eq!(balance, uint!(0_U256));

        Ok(())
    }
}

mod travel_call_with_time_dependent_precompile {
    use super::*;
    use crate::test_harness::contracts::web_proof::{
        ACCOUNT_ADDRESS, WEB_PROOF, WEB_PROOF_PROVER, WebProof, WebProofProver::mainCall,
    };

    #[tokio::test(flavor = "multi_thread")]
    #[should_panic(expected = "Precompile `WebProof` is not allowed for travel calls")]
    async fn fails_after_travel_call() {
        let location: ExecutionLocation = (AnvilHardhat, BlockTag::Latest).into();

        let call_data = mainCall {
            webProof: WebProof {
                webProofJson: WEB_PROOF.clone(),
            },
            account: ACCOUNT_ADDRESS,
        };
        let call = call(WEB_PROOF_PROVER, &call_data);

        let _ = preflight::<mainCall>("travel_call_with_time_dep_precompile", call, &location)
            .await
            .unwrap();
    }
}
