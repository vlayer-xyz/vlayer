use anyhow::Context;
use ethers_providers::{Http, Provider, RetryClient};
use guest_wrapper::GUEST_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv, ProveInfo};
use vlayer_steel::{
    config::ETH_SEPOLIA_CHAIN_SPEC,
    contract::{call::evm_call, CallTxData},
    ethereum::EthEvmEnv,
    guest_input::GuestInput,
    host::{db::ProofDb, provider::EthersProvider},
};

pub type EthersClient = Provider<RetryClient<Http>>;

pub struct Host {
    env: EthEvmEnv<ProofDb<EthersProvider<EthersClient>>>,
}

impl Host {
    pub fn try_new() -> anyhow::Result<Self> {
        let env = EthEvmEnv::from_rpc("http://localhost:8545", None)?
            .with_chain_spec(&ETH_SEPOLIA_CHAIN_SPEC)?;
        Ok(Host { env })
    }

    pub fn run(
        mut self,
        raw_call_data: Vec<u8>,
        call_data: CallTxData<()>,
    ) -> anyhow::Result<Vec<u8>> {
        let _returns = evm_call(call_data, &mut self.env)?;

        let evm_input = self.env.into_input()?;
        let input = GuestInput {
            evm_input,
            call_data: raw_call_data,
        };
        let env = ExecutorEnv::builder()
            .write(&input)
            .unwrap()
            .build()
            .unwrap();

        let prove_info = Host::prove(env)?;

        Ok(prove_info.receipt.journal.bytes)
    }

    fn prove(env: ExecutorEnv) -> anyhow::Result<ProveInfo> {
        let prover = default_prover();
        prover.prove(env, GUEST_ELF).context("failed to run prover")
    }
}
