#[cfg(test)]
mod test {
    use alloy_primitives::{address, b256, uint, Address, U256};
    use alloy_sol_types::{sol, SolCall};
    use host::Call;
    use vlayer_engine::config::{MAINNET_ID, SEPOLIA_ID};
    use IERC20::balanceOfCall;

    use crate::{
        host::{Host, HostConfig},
        provider::EthFileProvider,
    };

    const RPC_CACHE_FILE: &str = "testdata/rpc_cache.json";

    fn run(call: Call, chain_id: u64, block_number: u64) -> anyhow::Result<Vec<u8>> {
        let test_provider = EthFileProvider::from_file(&RPC_CACHE_FILE.into())?;
        let null_rpc_url = "a null url value as url is not needed in tests";
        let config = HostConfig::new(null_rpc_url, chain_id, block_number);
        let host = Host::try_new_with_provider(test_provider, config)?;
        Ok(host.run(call)?.evm_call_result)
    }

    #[cfg(test)]
    #[ctor::ctor]
    fn before_all() {
        use std::env::set_var;
        set_var("RISC0_DEV_MODE", "1")
    }

    const ERC20_TEST_CONTRACT: Address = address!("dAC17F958D2ee523a2206206994597C13D831ec7"); // USDT
    const ERC20_TEST_BLOCK_NO: u64 = 19_493_153;
    sol! {
        #[derive(Debug, PartialEq, Eq)]
        interface IERC20 {
            function balanceOf(address account) external view returns (uint);
        }
    }

    #[test]
    fn erc20_balance_of() -> anyhow::Result<()> {
        let call = IERC20::balanceOfCall {
            account: address!("F977814e90dA44bFA03b6295A0616a897441aceC"), // Binance 8
        };
        let call = Call {
            caller: ERC20_TEST_CONTRACT,
            to: ERC20_TEST_CONTRACT,
            data: call.abi_encode(),
        };
        let result =
            balanceOfCall::abi_decode_returns(&run(call, SEPOLIA_ID, ERC20_TEST_BLOCK_NO)?, false)?;
        assert_eq!(result._0, uint!(3_000_000_000_000_000_U256));
        Ok(())
    }

    #[test]
    fn uniswap_exact_output_single() -> anyhow::Result<()> {
        // mimic tx 0x241c81c3aa4c68cd07ae03a756050fc47fd91918a710250453d34c6db9d11997
        let block_no = 19493153;
        let caller = address!("f5213a6a2f0890321712520b8048D9886c1A9900");
        let contract = address!("E592427A0AEce92De3Edee1F18E0157C05861564"); // Uniswap V3
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

        // swap USDT for 34.1973 WETH
        let call = ISwapRouter::exactOutputSingleCall {
            params: ISwapRouter::ExactOutputSingleParams {
                tokenIn: address!("dAC17F958D2ee523a2206206994597C13D831ec7"), // USDT
                tokenOut: address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"), // WETH
                fee: 500,
                recipient: caller,
                deadline: uint!(1711146836_U256),
                amountOut: uint!(34197300000000000000_U256),
                amountInMaximum: U256::MAX,
                sqrtPriceLimitX96: U256::ZERO,
            },
        };
        let call = Call {
            caller,
            to: contract,
            data: call.abi_encode(),
        };
        let result = ISwapRouter::exactOutputSingleCall::abi_decode_returns(
            &run(call, MAINNET_ID, block_no)?,
            false,
        )?;
        assert_eq!(result.amountIn, uint!(112537714517_U256));
        Ok(())
    }

    const VIEW_CALL_TEST_CONTRACT: Address = address!("C5096d96dbC7594B3d0Ba50e708ba654A7ae1F3E");
    const VIEW_CALL_TEST_BLOCK_NO: u64 = 5702743;
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
        let call = ViewCallTest::testPrecompileCall {};
        let call = Call {
            caller: VIEW_CALL_TEST_CONTRACT,
            to: VIEW_CALL_TEST_CONTRACT,
            data: call.abi_encode(),
        };
        let result = ViewCallTest::testPrecompileCall::abi_decode_returns(
            &run(call, SEPOLIA_ID, VIEW_CALL_TEST_BLOCK_NO)?,
            false,
        )?;
        assert_eq!(
            result._0,
            b256!("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
        );
        Ok(())
    }

    #[test]
    fn nonexistent_account() -> anyhow::Result<()> {
        let call = ViewCallTest::testNonexistentAccountCall {};
        let call = Call {
            caller: VIEW_CALL_TEST_CONTRACT,
            to: VIEW_CALL_TEST_CONTRACT,
            data: call.abi_encode(),
        };
        let result = ViewCallTest::testNonexistentAccountCall::abi_decode_returns(
            &run(call, SEPOLIA_ID, VIEW_CALL_TEST_BLOCK_NO)?,
            false,
        )?;
        assert_eq!(result.size, uint!(0_U256));
        Ok(())
    }

    #[test]
    fn eoa_account() -> anyhow::Result<()> {
        let call = ViewCallTest::testEoaAccountCall {};
        let call = Call {
            caller: VIEW_CALL_TEST_CONTRACT,
            to: VIEW_CALL_TEST_CONTRACT,
            data: call.abi_encode(),
        };
        let result = ViewCallTest::testEoaAccountCall::abi_decode_returns(
            &run(call, SEPOLIA_ID, VIEW_CALL_TEST_BLOCK_NO)?,
            false,
        )?;
        assert_eq!(result.size, uint!(0_U256));
        Ok(())
    }

    #[test]
    fn blockhash() -> anyhow::Result<()> {
        let call = ViewCallTest::testBlockhashCall {};
        let call = Call {
            caller: VIEW_CALL_TEST_CONTRACT,
            to: VIEW_CALL_TEST_CONTRACT,
            data: call.abi_encode(),
        };
        let result = ViewCallTest::testBlockhashCall::abi_decode_returns(
            &run(call, SEPOLIA_ID, VIEW_CALL_TEST_BLOCK_NO)?,
            false,
        )?;
        assert_eq!(
            result._0,
            b256!("7703fe4a3d6031a579d52ce9e493e7907d376cfc3b41f9bc7710b0dae8c67f68")
        );
        Ok(())
    }

    #[test]
    fn chainid() -> anyhow::Result<()> {
        let call = ViewCallTest::testChainidCall {};
        let call = Call {
            caller: VIEW_CALL_TEST_CONTRACT,
            to: VIEW_CALL_TEST_CONTRACT,
            data: call.abi_encode(),
        };
        let result = ViewCallTest::testChainidCall::abi_decode_returns(
            &run(call, SEPOLIA_ID, VIEW_CALL_TEST_BLOCK_NO)?,
            false,
        )?;
        assert_eq!(result._0, uint!(11155111_U256));
        Ok(())
    }

    #[test]
    fn multi_contract_calls() -> anyhow::Result<()> {
        let call = ViewCallTest::testMuliContractCallsCall {};
        let call = Call {
            caller: VIEW_CALL_TEST_CONTRACT,
            to: VIEW_CALL_TEST_CONTRACT,
            data: call.abi_encode(),
        };
        let result = ViewCallTest::testMuliContractCallsCall::abi_decode_returns(
            &run(call, SEPOLIA_ID, VIEW_CALL_TEST_BLOCK_NO)?,
            false,
        )?;
        assert_eq!(result._0, uint!(84_U256));
        Ok(())
    }

    #[test]
    fn call_eoa() -> anyhow::Result<()> {
        let call = Call {
            to: address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045"), // vitalik.eth
            ..Default::default()
        };
        run(call, SEPOLIA_ID, VIEW_CALL_TEST_BLOCK_NO).expect_err("calling an EOA should fail");

        Ok(())
    }
}
