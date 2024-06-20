use alloy_primitives::Sealable;
use anyhow::anyhow;
use ethers_providers::Provider as OGEthersProvider;
use ethers_providers::{Http, RetryClient};
use guest_wrapper::GUEST_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv};
use vlayer_engine::guest::{Call, Input, Output};
use vlayer_engine::{
    config::ETH_SEPOLIA_CHAIN_SPEC,
    contract::engine::Engine,
    ethereum::EthEvmEnv,
    host::{
        db::ProofDb,
        provider::{EthersProvider, Provider},
    },
    EvmEnv,
};

const MAX_RETRY: u32 = 3;
const INITIAL_BACKOFF: u64 = 500;

pub type EthersClient = OGEthersProvider<RetryClient<Http>>;

pub struct Host {
    env: EthEvmEnv<ProofDb<EthersProvider<EthersClient>>>,
}

#[derive(Debug, PartialEq)]
pub enum HostError {
    ElfParseError,
    ExecutorEnvBuilderError,
    InvalidInput,
    RpcConnectionError,
    Unknown(String),
}

impl From<anyhow::Error> for HostError {
    fn from(error: anyhow::Error) -> Self {
        let error_str = error.to_string();
        if error_str.contains("Elf parse error") {
            Self::ElfParseError
        } else if error_str.contains(
            "Guest panicked: called `Result::unwrap()` on an `Err` value: DeserializeUnexpectedEnd",
        ) {
            Self::InvalidInput
        } else if error_str.contains("tcp connect error: Connection refused") {
            Self::RpcConnectionError
        } else {
            Self::Unknown(error_str)
        }
    }
}

impl Host {
    pub fn try_new(url: &str) -> Result<Self, HostError> {
        let client = EthersClient::new_client(url, MAX_RETRY, INITIAL_BACKOFF)
            .map_err(|err| anyhow!(err))?;
        let provider = EthersProvider::new(client);
        let block_number = provider.get_block_number().map_err(|err| anyhow!(err))?;
        let header = provider.get_block_header(block_number).unwrap().unwrap();
        let db = ProofDb::new(provider, block_number);
        let env = EvmEnv::new(db, header.seal_slow()).with_chain_spec(&ETH_SEPOLIA_CHAIN_SPEC)?;
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
