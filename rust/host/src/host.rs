use anyhow::{anyhow, Error};

use ethers_providers::{Http, Provider, RetryClient};
use guest_wrapper::GUEST_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv, ProveInfo};
use vlayer_steel::{
    config::ETH_SEPOLIA_CHAIN_SPEC,
    contract::engine::Engine,
    ethereum::EthEvmEnv,
    guest::{Call, Input},
    host::{db::ProofDb, provider::EthersProvider},
};

pub type EthersClient = Provider<RetryClient<Http>>;

pub struct Host {
    env: EthEvmEnv<ProofDb<EthersProvider<EthersClient>>>,
}

#[derive(Debug)]
pub enum HostError {
    ProveFailed(Error),
}

impl std::fmt::Display for HostError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProveFailed(err) => write!(f, "HostError: {}", err),
        }
    }
}

impl Host {
    pub fn try_new() -> anyhow::Result<Self> {
        let env = EthEvmEnv::from_rpc("http://localhost:8545", None)?
            .with_chain_spec(&ETH_SEPOLIA_CHAIN_SPEC)?;
        Ok(Host { env })
    }

    pub fn run(mut self, call_data: Call) -> anyhow::Result<Vec<u8>> {
        let call = call_data.clone();
        let _returns = Engine::evm_call(call_data, &mut self.env)?;

        let evm_input = self.env.into_input()?;
        let input = Input {
            evm_input,
            call: Call { caller, to, data },
        };
        let env = ExecutorEnv::builder()
            .write(&input)
            .unwrap()
            .build()
            .unwrap();

        let prove_info = Host::prove(env).map_err(|err| anyhow!(err))?;

        Ok(prove_info.receipt.journal.bytes)
    }

    fn prove(env: ExecutorEnv) -> Result<ProveInfo, HostError> {
        let prover = default_prover();
        prover.prove(env, GUEST_ELF).map_err(HostError::ProveFailed)
    }
}
