use alloy_primitives::Address;
use alloy_sol_types::{sol, SolCall};
use anyhow::Context;
use host::db::proof::ProofDb;
use host::provider::Provider;
use std::fmt::Debug;
use test_log::test;
use vlayer_engine::{config::SEPOLIA_ID, engine::Engine, io::Call};

macro_rules! provider {
    () => {
        test_provider!()
        // use the following to fill the cache file
        // test_provider!("<RPC-URL>")
    };
}

const RPC_CACHE_FILE: &str = "testdata/rpc_cache.json";

// Create a file provider or a cached Ethers provider when an URL is specified.
macro_rules! test_provider {
    () => {
        host::provider::EthFileProvider::from_file(&RPC_CACHE_FILE.into()).unwrap()
    };
    ($url:tt) => {{
        let client = host::provider::EthersClient::new_client($url, 3, 500).unwrap();
        let provider = host::provider::EthersProvider::new(client);
        host::provider::CachedProvider::new(RPC_CACHE_FILE.into(), provider).unwrap()
    }};
}

pub fn from_provider<P: Provider>(
    provider: P,
    block_number: u64,
) -> anyhow::Result<(ProofDb<P>, P::Header)> {
    let header = provider
        .get_block_header(block_number)?
        .with_context(|| format!("block {block_number} not found"))?;

    // create a new database backed by the provider
    let db = ProofDb::new(provider, block_number);

    Ok((db, header))
}

const VIEW_CALL_TEST_BLOCK: u64 = 5702743;
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
fn call_eoa() {
    let (db, header) = from_provider(provider!(), VIEW_CALL_TEST_BLOCK).unwrap();
    Engine::try_new(db, header, SEPOLIA_ID)
        .unwrap()
        .call(&Call {
            caller: Address::ZERO,
            to: Address::ZERO,
            data: (ViewCallTest::testBlockhashCall {}).abi_encode(),
        })
        .expect_err("calling an EOA should fail");
}
