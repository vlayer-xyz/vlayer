use std::{collections::HashMap, env};

use alloy_chains::{Chain, NamedChain};
use alloy_primitives::{address, b256, uint, Address, ChainId};
use alloy_sol_types::{sol, SolCall};
use call_host::{
    host::{config::HostConfig, error::HostError, get_block_header, Host},
    Call,
};
use chain_client::RpcClient as RpcChainProofClient;
use dotenvy::dotenv;
use ethers_core::types::BlockNumber as BlockTag;
use lazy_static::lazy_static;
use mock_chain_server::{fake_proof_result, ChainProofServerMock};
use provider::{BlockNumber, CachedMultiProvider, CachedProviderFactory};
use serde_json::json;

// To activate recording, set UPDATE_SNAPSHOTS to true.
// Recording creates new testdata directory and writes return data from Alchemy into files in that directory.
const UPDATE_SNAPSHOTS: bool = false;
const LATEST_BLOCK: BlockTag = BlockTag::Latest;

struct ExecutionLocation {
    pub chain_id: ChainId,
    pub block_tag: BlockTag,
}

impl<C, B> From<(C, B)> for ExecutionLocation
where
    C: Into<ChainId>,
    B: Into<BlockTag>,
{
    fn from((chain_id, block_tag): (C, B)) -> Self {
        ExecutionLocation {
            chain_id: chain_id.into(),
            block_tag: block_tag.into(),
        }
    }
}

fn get_alchemy_key() -> String {
    dotenv().ok();
    env::var("ALCHEMY_KEY").expect(
        "To use recording provider you need to set ALCHEMY_KEY in an .env file. See .env.example",
    )
}

lazy_static! {
    static ref alchemy_key: String = get_alchemy_key();
    static ref mainnet_url: String =
        format!("https://eth-mainnet.g.alchemy.com/v2/{}", *alchemy_key);
    static ref sepolia_url: String =
        format!("https://eth-sepolia.g.alchemy.com/v2/{}", *alchemy_key);
    static ref anvil_url: String = format!("http://localhost:8545");
    static ref sepolia_latest_block: ExecutionLocation =
        (Chain::sepolia().id(), LATEST_BLOCK).into();
}

fn rpc_file_cache(test_name: &str) -> HashMap<ChainId, String> {
    HashMap::from([
        (Chain::mainnet().id(), format!("testdata/mainnet_{test_name}_rpc_cache.json")),
        (Chain::sepolia().id(), format!("testdata/sepolia_{test_name}_rpc_cache.json")),
        (
            NamedChain::AnvilHardhat.into(),
            format!("testdata/anvil_{test_name}_rpc_cache.json"),
        ),
    ])
}

fn rpc_urls() -> HashMap<ChainId, String> {
    HashMap::from([
        (Chain::mainnet().id(), mainnet_url.clone()),
        (Chain::sepolia().id(), sepolia_url.clone()),
        (NamedChain::AnvilHardhat.into(), anvil_url.clone()),
    ])
}

async fn run<C>(
    test_name: &str,
    call: Call,
    location: &ExecutionLocation,
) -> anyhow::Result<C::Return>
where
    C: SolCall,
{
    let multi_provider = create_multi_provider(test_name);
    let chain_proof_server = create_chain_proof_server(&multi_provider, location).await?;
    let host = create_host(multi_provider, location, chain_proof_server.url())?;
    let host_output = host.main(call).await?;
    let return_value = C::abi_decode_returns(&host_output.guest_output.evm_call_result, false)?;

    chain_proof_server.assert();

    Ok(return_value)
}

fn create_multi_provider(test_name: &str) -> CachedMultiProvider {
    let maybe_ethers_provider_factory =
        UPDATE_SNAPSHOTS.then(|| provider::EthersProviderFactory::new(rpc_urls()));
    let provider_factory =
        CachedProviderFactory::new(rpc_file_cache(test_name), maybe_ethers_provider_factory);
    CachedMultiProvider::new(provider_factory)
}

async fn create_chain_proof_server(
    multi_provider: &CachedMultiProvider,
    location: &ExecutionLocation,
) -> Result<ChainProofServerMock, HostError> {
    let block_header = get_block_header(multi_provider, location.chain_id, location.block_tag)?;
    let block_number = block_header.number();
    let result = fake_proof_result(block_header);

    let chain_proof_server_mock = ChainProofServerMock::start(
        json!({
            "chain_id": location.chain_id,
            "block_numbers": [block_number]
        }),
        result,
    )
    .await;

    Ok(chain_proof_server_mock)
}

fn create_host(
    multi_provider: CachedMultiProvider,
    location: &ExecutionLocation,
    chain_proof_server_url: impl AsRef<str>,
) -> Result<Host, HostError> {
    let config = HostConfig {
        start_chain_id: location.chain_id,
        ..Default::default()
    };
    let block_number =
        block_tag_to_block_number(&multi_provider, location.chain_id, location.block_tag)?;
    let chain_proof_client = RpcChainProofClient::new(chain_proof_server_url);
    Host::try_new_with_components(multi_provider, block_number, chain_proof_client, &config)
}

fn block_tag_to_block_number(
    multi_provider: &CachedMultiProvider,
    chain_id: ChainId,
    block_tag: BlockTag,
) -> Result<BlockNumber, HostError> {
    match block_tag {
        BlockTag::Latest => {
            Ok(get_block_header(multi_provider, chain_id, BlockTag::Latest)?.number())
        }
        BlockTag::Number(block_no) => Ok(block_no.as_u64()),
        _ => panic!("Only Latest and specific block numbers are supported, got {:?}", block_tag),
    }
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
    use super::*;

    const UNISWAP: Address = address!("1F98431c8aD98523631AE4a59f267346ea31F984");
    sol! {
        #[derive(Debug, PartialEq, Eq)]
        interface IUniswapV3Factory {
            function owner() external view returns (address);
        }
    }

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

    use super::*;

    // Generated using `simple_teleport` example
    const SIMPLE_TELEPORT: Address = address!("5fbdb2315678afecb367f032d93f642f64180aa3");
    const BLOCK_NO: u64 = 3;
    sol! {
        contract SimpleTravelProver {
            #[derive(Debug)]
            function crossChainBalanceOf(address owner) public returns (address, uint256);
        }
    }

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
