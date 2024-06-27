#[cfg(test)]
mod test {
    use alloy_primitives::{address, uint, Address};
    use alloy_sol_types::{sol, SolCall};
    use host::Call;
    use vlayer_engine::config::SEPOLIA_ID;
    use IERC20::balanceOfCall;

    use crate::{
        host::{Host, HostConfig},
        provider::EthFileProvider,
    };

    const RPC_CACHE_FILE: &str = "testdata/rpc_cache.json";
    const NULL_RPC_URL: &str = "a null url value because url is not needed in tests";

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
    fn before_all() {
        use std::env::set_var;
        set_var("RISC0_DEV_MODE", "1")
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

        let test_provider = EthFileProvider::from_file(&RPC_CACHE_FILE.into())?;
        let host = Host::try_new_with_provider(
            test_provider,
            HostConfig::new(NULL_RPC_URL, SEPOLIA_ID, ERC20_TEST_BLOCK_NO),
        )?;
        let host_result =
            balanceOfCall::abi_decode_returns(&host.run(call)?.evm_call_result, false)?;
        assert_eq!(host_result._0, uint!(3_000_000_000_000_000_U256));
        Ok(())
    }
}
