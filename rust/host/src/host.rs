use alloy_primitives::Sealable;
use anyhow::anyhow;
use ethers_providers::Provider as OGEthersProvider;
use ethers_providers::{Http, RetryClient};
use guest_wrapper::GUEST_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv};
use thiserror::Error;
use vlayer_engine::config::SEPOLIA_ID;
use vlayer_engine::ethereum::EthBlockHeader;
use vlayer_engine::guest::{Call, Input, Output};
use vlayer_engine::host::into_input;
use vlayer_engine::{
    contract::engine,
    contract::engine::Engine,
    host::{
        db::ProofDb,
        provider::{EthersProvider, Provider},
    },
};

const MAX_RETRY: u32 = 3;
const INITIAL_BACKOFF: u64 = 500;

pub type EthersClient = OGEthersProvider<RetryClient<Http>>;

pub struct Host {
    db: ProofDb<EthersProvider<EthersClient>>,
    header: EthBlockHeader,
}

#[derive(Error, Debug, PartialEq)]
pub enum HostError {
    #[error("Elf parse error")]
    ElfParseError,
    #[error("ExecutorEnv builder error")]
    ExecutorEnvBuilderError,
    #[error("Invalid input")]
    InvalidInput,
    #[error("Rpc connection error")]
    RpcConnectionError,
    #[error("Engine error")]
    EngineError(#[from] engine::EngineError),
    #[error("Unknown error: {0}")]
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

        Ok(Host { db, header })
    }

    pub fn run(mut self, call: Call) -> Result<Output, HostError> {
        let _returns =
            Engine::try_new(&mut self.db, self.header.clone(), SEPOLIA_ID)?.call(&call)?;

        let input = Input {
            call,
            evm_input: into_input(self.db, self.header.seal_slow())?,
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
