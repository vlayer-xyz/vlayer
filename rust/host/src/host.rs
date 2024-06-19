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
    ExecutorEnvBuilderError,
    InvalidInput,
    Unknown(String),
}

impl From<anyhow::Error> for HostError {
    fn from(error: anyhow::Error) -> Self {
        if error.to_string().contains("Elf parse error") {
            HostError::ElfParseError
        } else if error.to_string().contains(
            "Guest panicked: called `Result::unwrap()` on an `Err` value: DeserializeUnexpectedEnd",
        ) {
            HostError::InvalidInput
        } else {
            HostError::Unknown(error.to_string())
        }
    }
}

impl Host {
    pub fn try_new() -> Result<Self, HostError> {
        let env = EthEvmEnv::from_rpc("http://localhost:8545", None)?
            .with_chain_spec(&ETH_SEPOLIA_CHAIN_SPEC)?;
        Ok(Host { env })
    }

    pub fn run(mut self, call: Call) -> Result<Output, HostError> {
        let _returns = Engine::evm_call(&call, &mut self.env)?;

        let input = Input {
            call,
            evm_input: self.env.into_input()?,
        };

        let env = ExecutorEnv::builder()
            .write(&input)
            .map_err(|_| HostError::ExecutorEnvBuilderError)?
            .build()
            .map_err(|_| HostError::ExecutorEnvBuilderError)?;

        Host::prove(env, GUEST_ELF)
    }

    pub(crate) fn prove(env: ExecutorEnv, guest_elf: &[u8]) -> Result<Output, HostError> {
        let prover = default_prover();
        prover
            .prove(env, guest_elf)
            .map(|p| p.receipt.journal.into())
            .map_err(HostError::from)
    }
}
