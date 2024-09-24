use alloy_chains::Chain;
use alloy_primitives::{address, b256, uint, Address, ChainId};
use alloy_sol_types::{sol, SolCall};
use dotenv::dotenv;
use ethers_core::types::BlockNumber as BlockTag;

use call_host::{
    host::{config::HostConfig, error::HostError, Host},
    Call,
};
use provider::{BlockingProvider, CachedProviderFactory, FileProviderFactory, ProviderFactory};
use std::{collections::HashMap, env};

// To activate recording, set UPDATE_SNAPSHOTS to true.
// Recording creates new testdata directory and writes return data from Alchemy into files in that directory.
const UPDATE_SNAPSHOTS: bool = false;
const LATEST_BLOCK: BlockTag = BlockTag::Latest;

fn create_test_provider_factory(test_name: &str) -> FileProviderFactory {
    let rpc_file_cache: HashMap<_, _> = HashMap::from([
        (Chain::mainnet().id(), format!("testdata/mainnet_{test_name}_rpc_cache.json")),
        (Chain::sepolia().id(), format!("testdata/sepolia_{test_name}_rpc_cache.json")),
    ]);

    FileProviderFactory::new(rpc_file_cache)
}

fn create_recording_provider_factory(test_name: &str) -> CachedProviderFactory {
    let rpc_file_cache: HashMap<_, _> = HashMap::from([
        (Chain::mainnet().id(), format!("testdata/mainnet_{test_name}_rpc_cache.json")),
        (Chain::sepolia().id(), format!("testdata/sepolia_{test_name}_rpc_cache.json")),
    ]);
    dotenv().ok();
    let alchemy_key = env::var("ALCHEMY_KEY").expect(
        "To use recording provider you need to set ALCHEMY_KEY in an .env file. See .env.example",
    );
    let mainnet_url = format!("https://eth-mainnet.g.alchemy.com/v2/{alchemy_key}");
    let sepolia_url = format!("https://eth-sepolia.g.alchemy.com/v2/{alchemy_key}");
    let rpc_urls: HashMap<_, _> =
        HashMap::from([(Chain::mainnet().id(), mainnet_url), (Chain::sepolia().id(), sepolia_url)]);

    CachedProviderFactory::new(rpc_urls, rpc_file_cache)
}

fn create_host<P>(
    provider_factory: impl ProviderFactory<P> + 'static,
    config: &HostConfig,
    block_number: BlockTag,
) -> Result<Host<P>, HostError>
where
    P: BlockingProvider + 'static,
{
    match block_number {
        BlockTag::Latest => Host::try_new_with_provider_factory(provider_factory, config),
        BlockTag::Number(block_no) => Host::try_new_with_provider_factory_and_block_number(
            provider_factory,
            config,
            block_no.as_u64(),
        ),
        _ => panic!("Only Latest and specific block numbers are supported, got {:?}", block_number),
    }
}

fn run<C>(
    test_name: &str,
    call: Call,
    chain_id: ChainId,
    block_number: BlockTag,
) -> anyhow::Result<C::Return>
where
    C: SolCall,
{
    let config = HostConfig {
        start_chain_id: chain_id,
        ..Default::default()
    };

    let raw_return_value = if UPDATE_SNAPSHOTS {
        let provider_factory = create_recording_provider_factory(test_name);
        let host = create_host(provider_factory, &config, block_number)?;
        host.run(call)?.guest_output.evm_call_result
    } else {
        let provider_factory = create_test_provider_factory(test_name);
        let host = create_host(provider_factory, &config, block_number)?;
        host.run(call)?.guest_output.evm_call_result
    };
    let return_value = C::abi_decode_returns(&raw_return_value, false)?;
    Ok(return_value)
}

#[cfg(test)]
#[ctor::ctor]
fn before_all() {
    use std::{env::set_var, fs};
    set_var("RISC0_DEV_MODE", "1");

    if UPDATE_SNAPSHOTS {
        fs::remove_dir_all("testdata").ok();
        fs::create_dir("testdata").ok();
    }
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
            to: USDT,
            data: sol_call.abi_encode(),
        };
        let result = run::<IERC20::balanceOfCall>(
            "usdt_erc20_balance_of",
            call,
            Chain::mainnet().id(),
            USDT_BLOCK_NO.into(),
        )?;
        assert_eq!(result._0, uint!(3_000_000_000_000_000_U256));
        Ok(())
    }
}

mod uniswap {
    use super::*;

    const UNISWAP: Address = address!("1F98431c8aD98523631AE4a59f267346ea31F984");
    sol! {
        #[derive(Debug, PartialEq, Eq)]
        interface IUniswapV3Factory {
            function owner() external view returns (address);
        }
    }

    #[test]
    fn factory_owner() -> anyhow::Result<()> {
        let sol_call = IUniswapV3Factory::ownerCall {};
        let call = Call {
            to: UNISWAP,
            data: sol_call.abi_encode(),
        };
        let result = run::<IUniswapV3Factory::ownerCall>(
            "uniswap_factory_owner",
            call,
            Chain::mainnet().id(),
            LATEST_BLOCK,
        )?;
        assert_eq!(
            result._0,
            address!("1a9c8182c09f50c8318d769245bea52c32be35bc") // Uniswap V2: UNI Timelock is the current owner of the factory.
        );
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
            to: VIEW_CALL,
            data: sol_call.abi_encode(),
        };
        let result = run::<ViewCallTest::testPrecompileCall>(
            "view_precompile",
            call,
            Chain::sepolia().id(),
            LATEST_BLOCK,
        )?;
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
            to: VIEW_CALL,
            data: sol_call.abi_encode(),
        };
        let result = run::<ViewCallTest::testNonexistentAccountCall>(
            "view_nonexistent_account",
            call,
            Chain::sepolia().id(),
            LATEST_BLOCK,
        )?;
        assert_eq!(result.size, uint!(0_U256));
        Ok(())
    }

    #[test]
    fn eoa_account() -> anyhow::Result<()> {
        let sol_call = ViewCallTest::testEoaAccountCall {};
        let call = Call {
            to: VIEW_CALL,
            data: sol_call.abi_encode(),
        };
        let result = run::<ViewCallTest::testEoaAccountCall>(
            "view_eoa_account",
            call,
            Chain::sepolia().id(),
            LATEST_BLOCK,
        )?;
        assert_eq!(result.size, uint!(0_U256));
        Ok(())
    }

    #[test]
    fn blockhash() -> anyhow::Result<()> {
        let sol_call = ViewCallTest::testBlockhashCall {};
        let call = Call {
            to: VIEW_CALL,
            data: sol_call.abi_encode(),
        };
        let result = run::<ViewCallTest::testBlockhashCall>(
            "view_blockhash",
            call,
            Chain::sepolia().id(),
            VIEW_CALL_BLOCK_NO.into(),
        )?;
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
            to: VIEW_CALL,
            data: sol_call.abi_encode(),
        };
        let result = run::<ViewCallTest::testChainidCall>(
            "view_chainid",
            call,
            Chain::sepolia().id(),
            LATEST_BLOCK,
        )?;
        assert_eq!(result._0, uint!(11_155_111_U256));
        Ok(())
    }

    #[test]
    fn multi_contract_calls() -> anyhow::Result<()> {
        let sol_call = ViewCallTest::testMuliContractCallsCall {};
        let call = Call {
            to: VIEW_CALL,
            data: sol_call.abi_encode(),
        };
        let result = run::<ViewCallTest::testMuliContractCallsCall>(
            "view_multi_contract_calls",
            call,
            Chain::sepolia().id(),
            LATEST_BLOCK,
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
        run::<ViewCallTest::testEoaAccountCall>(
            "view_call_eoa",
            call,
            Chain::sepolia().id(),
            LATEST_BLOCK,
        )
        .expect_err("calling an EOA should fail");

        Ok(())
    }
}
