use alloy_primitives::{address, b256, uint, Address, U256};
use alloy_sol_types::{sol, SolCall};
use host::{
    host::{config::HostConfig, Host},
    provider::factory::FileProviderFactory,
    Call,
};
use std::collections::HashMap;
use vlayer_engine::{
    config::{MAINNET_ID, SEPOLIA_ID},
    evm::env::location::ExecutionLocation,
};

fn create_test_provider_factory() -> FileProviderFactory {
    let rpc_file_cache = HashMap::from([
        (MAINNET_ID, "testdata/mainnet_rpc_cache.json".to_string()),
        (SEPOLIA_ID, "testdata/sepolia_rpc_cache.json".to_string()),
    ]);

    FileProviderFactory::new(rpc_file_cache)
}

fn run<C>(call: Call, chain_id: u64, block_number: u64) -> anyhow::Result<C::Return>
where
    C: SolCall,
{
    let provider_factory = create_test_provider_factory();
    let null_rpc_url = "a null url value as url is not needed in tests";
    let execution_location = ExecutionLocation::new(block_number, chain_id);
    let config = HostConfig::new(null_rpc_url, execution_location);
    let host = Host::try_new_with_provider_factory(provider_factory, config)?;
    let raw_return_value = host.run(call)?.guest_output.evm_call_result;
    let return_value = C::abi_decode_returns(&raw_return_value, false)?;
    Ok(return_value)
}

#[cfg(test)]
#[ctor::ctor]
fn before_all() {
    use std::env::set_var;
    set_var("RISC0_DEV_MODE", "1")
}

mod usdt {
    use super::*;

    const USDT: Address = address!("dAC17F958D2ee523a2206206994597C13D831ec7");
    const USDT_BLOCK_NO: u64 = 19_493_153;
    sol! {
        #[derive(Debug, PartialEq, Eq)]
        interface IERC20 {
            function balanceOf(address account) external view returns (uint);
        }
    }

    #[test]
    fn erc20_balance_of() -> anyhow::Result<()> {
        let sol_call = IERC20::balanceOfCall {
            account: address!("F977814e90dA44bFA03b6295A0616a897441aceC"), // Binance 8
        };
        let call = Call {
            caller: USDT,
            to: USDT,
            data: sol_call.abi_encode(),
        };
        let result = run::<IERC20::balanceOfCall>(call, MAINNET_ID, USDT_BLOCK_NO)?;
        assert_eq!(result._0, uint!(3_000_000_000_000_000_U256));
        Ok(())
    }
}

mod uniswap {
    use super::*;

    const UNISWAP: Address = address!("E592427A0AEce92De3Edee1F18E0157C05861564");
    const UNISWAP_USER: Address = address!("f5213a6a2f0890321712520b8048D9886c1A9900");
    const USDT: Address = address!("dAC17F958D2ee523a2206206994597C13D831ec7");
    const WETH: Address = address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");
    const BLOCK_NO: u64 = 19_493_153;
    sol! {
        #[derive(Debug, PartialEq, Eq)]
        interface ISwapRouter {
            struct ExactOutputSingleParams {
                address tokenIn;
                address tokenOut;
                uint24 fee;
                address recipient;
                uint256 deadline;
                uint256 amountOut;
                uint256 amountInMaximum;
                uint160 sqrtPriceLimitX96;
            }
            function exactOutputSingle(ExactOutputSingleParams calldata params) external payable returns (uint256 amountIn);
        }
    }

    #[test] // mimic tx 0x241c81c3aa4c68cd07ae03a756050fc47fd91918a710250453d34c6db9d11997
    fn uniswap_exact_output_single() -> anyhow::Result<()> {
        // swap USDT for 34.1973 WETH
        let sol_call = ISwapRouter::exactOutputSingleCall {
            params: ISwapRouter::ExactOutputSingleParams {
                tokenIn: USDT,
                tokenOut: WETH,
                fee: 500,
                recipient: UNISWAP_USER,
                deadline: uint!(1_711_146_836_U256),
                amountOut: uint!(34_197_300_000_000_000_000_U256),
                amountInMaximum: U256::MAX,
                sqrtPriceLimitX96: U256::ZERO,
            },
        };
        let call = Call {
            caller: UNISWAP_USER,
            to: UNISWAP,
            data: sol_call.abi_encode(),
        };
        let result = run::<ISwapRouter::exactOutputSingleCall>(call, MAINNET_ID, BLOCK_NO)?;
        assert_eq!(result.amountIn, uint!(112_537_714_517_U256));
        Ok(())
    }
}

mod view {
    use super::*;

    const VIEW_CALL: Address = address!("C5096d96dbC7594B3d0Ba50e708ba654A7ae1F3E");
    const VIEW_CALL_BLOCK_NO: u64 = 5_702_743;
    sol!(
        #[derive(Debug, PartialEq, Eq)]
        contract ViewCallTest {
            /// Tests the SHA256 precompile.
            function testPrecompile() external view returns (bytes32) {
                (bool ok, bytes memory out) = address(0x02).staticcall("");
                require(ok);
                return abi.decode(out, (bytes32));
            }

            /// Tests accessing the code of a nonexistent account.
            function testNonexistentAccount() external view returns (uint256 size) {
                address a = address(uint160(block.prevrandao));
                assembly { size := extcodesize(a) }
            }

            /// Tests accessing the code of the EOA account 0x0000000000000000000000000000000000000000.
            function testEoaAccount() external view returns (uint256 size) {
                assembly { size := extcodesize(0) }
            }

            /// Tests the blockhash opcode.
            function testBlockhash() external view returns (bytes32) {
                return blockhash(block.number - 2);
            }

            /// Tests retrieving the chain ID.
            function testChainid() external view returns (uint256) {
                return block.chainid;
            }

            /// Tests retrieving the gas price.
            function testGasprice() external view returns (uint256) {
                return tx.gasprice;
            }

            /// Tests calling multiple contracts with the same and different storage.
            function testMuliContractCalls() external view returns (uint256) {
                return VALUE0.value() + VALUE42_a.value() + VALUE42_b.value();
            }
        }
    );

    #[test]
    fn precompile() -> anyhow::Result<()> {
        let sol_call = ViewCallTest::testPrecompileCall {};
        let call = Call {
            caller: VIEW_CALL,
            to: VIEW_CALL,
            data: sol_call.abi_encode(),
        };
        let result = run::<ViewCallTest::testPrecompileCall>(call, SEPOLIA_ID, VIEW_CALL_BLOCK_NO)?;
        assert_eq!(
            result._0,
            b256!("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
        );
        Ok(())
    }

    #[test]
    fn nonexistent_account() -> anyhow::Result<()> {
        let sol_call = ViewCallTest::testNonexistentAccountCall {};
        let call = Call {
            caller: VIEW_CALL,
            to: VIEW_CALL,
            data: sol_call.abi_encode(),
        };
        let result =
            run::<ViewCallTest::testNonexistentAccountCall>(call, SEPOLIA_ID, VIEW_CALL_BLOCK_NO)?;
        assert_eq!(result.size, uint!(0_U256));
        Ok(())
    }

    #[test]
    fn eoa_account() -> anyhow::Result<()> {
        let sol_call = ViewCallTest::testEoaAccountCall {};
        let call = Call {
            caller: VIEW_CALL,
            to: VIEW_CALL,
            data: sol_call.abi_encode(),
        };
        let result = run::<ViewCallTest::testEoaAccountCall>(call, SEPOLIA_ID, VIEW_CALL_BLOCK_NO)?;
        assert_eq!(result.size, uint!(0_U256));
        Ok(())
    }

    #[test]
    fn blockhash() -> anyhow::Result<()> {
        let sol_call = ViewCallTest::testBlockhashCall {};
        let call = Call {
            caller: VIEW_CALL,
            to: VIEW_CALL,
            data: sol_call.abi_encode(),
        };
        let result = run::<ViewCallTest::testBlockhashCall>(call, SEPOLIA_ID, VIEW_CALL_BLOCK_NO)?;
        assert_eq!(
            result._0,
            b256!("7703fe4a3d6031a579d52ce9e493e7907d376cfc3b41f9bc7710b0dae8c67f68")
        );
        Ok(())
    }

    #[test]
    fn chainid() -> anyhow::Result<()> {
        let sol_call = ViewCallTest::testChainidCall {};
        let call = Call {
            caller: VIEW_CALL,
            to: VIEW_CALL,
            data: sol_call.abi_encode(),
        };
        let result = run::<ViewCallTest::testChainidCall>(call, SEPOLIA_ID, VIEW_CALL_BLOCK_NO)?;
        assert_eq!(result._0, uint!(11_155_111_U256));
        Ok(())
    }

    #[test]
    fn multi_contract_calls() -> anyhow::Result<()> {
        let sol_call = ViewCallTest::testMuliContractCallsCall {};
        let call = Call {
            caller: VIEW_CALL,
            to: VIEW_CALL,
            data: sol_call.abi_encode(),
        };
        let result =
            run::<ViewCallTest::testMuliContractCallsCall>(call, SEPOLIA_ID, VIEW_CALL_BLOCK_NO)?;
        assert_eq!(result._0, uint!(84_U256));
        Ok(())
    }

    #[test]
    fn call_eoa() -> anyhow::Result<()> {
        let call = Call {
            to: address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045"), // vitalik.eth
            ..Default::default()
        };
        run::<ViewCallTest::testEoaAccountCall>(call, SEPOLIA_ID, VIEW_CALL_BLOCK_NO)
            .expect_err("calling an EOA should fail");

        Ok(())
    }
}
