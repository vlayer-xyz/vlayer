#[cfg(test)]
mod test {
    use alloy_primitives::{address, uint, Address};
    use alloy_sol_types::{sol, SolCall};
    use host::Call;
    use vlayer_engine::{config::SEPOLIA_ID, host::provider::EthFileProvider};
    use IERC20::balanceOfCall;

    use crate::host::{Host, HostConfig};

    const RPC_CACHE_FILE: &str = "testdata/rpc_cache.json";

    const ERC20_TEST_CONTRACT: Address = address!("dAC17F958D2ee523a2206206994597C13D831ec7"); // USDT
    const ERC20_TEST_BLOCK_NO: u64 = 19493153;
    sol! {
        #[derive(Debug, PartialEq, Eq)]
        interface IERC20 {
            function balanceOf(address account) external view returns (uint);
        }
    }

    #[cfg(test)]
    #[ctor::ctor]
    fn init() {
        use std::env::set_var;
        set_var("RISC0_DEV_MODE", "1")
    }

    #[test]
    fn erc20_balance_of() {
        let call = IERC20::balanceOfCall {
            account: address!("F977814e90dA44bFA03b6295A0616a897441aceC"), // Binance 8
        };
        let call = Call {
            caller: ERC20_TEST_CONTRACT,
            to: ERC20_TEST_CONTRACT,
            data: call.abi_encode(),
        };

        let test_provider = EthFileProvider::from_file(&RPC_CACHE_FILE.into()).unwrap();
        let host = Host::try_new_with_provider(
            test_provider,
            HostConfig::new("", SEPOLIA_ID, ERC20_TEST_BLOCK_NO),
        )
        .unwrap();
        let host_result =
            balanceOfCall::abi_decode_returns(&host.run(call).unwrap().evm_call_result, false)
                .unwrap();
        assert_eq!(host_result._0, uint!(3000000000000000_U256));
    }
}
