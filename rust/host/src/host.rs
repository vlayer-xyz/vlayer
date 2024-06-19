use anyhow::anyhow;

use ethers_providers::{Http, Provider, RetryClient};
use guest_wrapper::GUEST_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv};
use vlayer_engine::{
    config::ETH_SEPOLIA_CHAIN_SPEC,
    contract::engine::Engine,
    ethereum::EthEvmEnv,
    guest::{Call, Input, Output},
    host::{db::ProofDb, provider::EthersProvider},
};

pub type EthersClient = Provider<RetryClient<Http>>;

pub struct Host {
    env: EthEvmEnv<ProofDb<EthersProvider<EthersClient>>>,
}

#[derive(Debug, PartialEq)]
pub enum HostError {
    ElfParseError,
    Unknown(String),
}

impl From<anyhow::Error> for HostError {
    fn from(error: anyhow::Error) -> Self {
        if error.to_string().contains("Elf parse error") {
            HostError::ElfParseError
        } else {
            HostError::Unknown(error.to_string())
        }
    }
}

impl std::fmt::Display for HostError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ElfParseError => write!(f, "ElfParseError"),
            Self::Unknown(err) => write!(f, "HostError::Unknown {:?}", err),
        }
    }
}

impl Host {
    pub fn try_new() -> anyhow::Result<Self> {
        let env = EthEvmEnv::from_rpc("http://localhost:8545", None)?
            .with_chain_spec(&ETH_SEPOLIA_CHAIN_SPEC)?;
        Ok(Host { env })
    }

    pub fn run(mut self, call: Call) -> anyhow::Result<Output> {
        let _returns = Engine::evm_call(&call, &mut self.env)?;

        let input = Input {
            call,
            evm_input: self.env.into_input()?,
        };

        let env = ExecutorEnv::builder()
            .write(&input)
            .unwrap()
            .build()
            .unwrap();

        Host::prove(env, GUEST_ELF).map_err(|err| anyhow!(err))
    }

    pub(crate) fn prove(env: ExecutorEnv, guest_elf: &[u8]) -> Result<Output, HostError> {
        let prover = default_prover();
        prover
            .prove(env, guest_elf)
            .map(|p| p.receipt.journal.into())
            .map_err(HostError::from)
    }
}
